/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * (c)2021 ZeroTier, Inc.
 * https://www.zerotier.com/
 */

use std::cell::Cell;
use std::collections::{HashMap, LinkedList};
use std::env::VarError;
use std::ffi::CString;
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use serde_json::{Map, Value};

const QUERY_TIMEOUT_MS: u64 = 2000;

/// Client that queries and caches JSON state from the service.
/// Currently it doesn't do much parsing since the JSON is just shoved
/// through to the JavaScript UI for most of what's done with it.
pub struct ServiceClient {
    refresh_base_paths: Vec<&'static str>,
    auth_token: String,
    port: u16,
    base_url: String,
    saved_networks: Map<String, Value>,
    state_hash: HashMap<String, u64>,
    state: Map<String, Value>,
    post_queue: LinkedList<(String, String)>,
    delete_queue: LinkedList<String>,
    dirty: Arc<AtomicBool>,
    online: bool,
    try_escalate_privs: isize,
}

pub fn ms_since_epoch() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_or(0, |t| t.as_millis() as i64)
}

pub fn get_user_home_dir() -> Result<String, VarError> {
    #[cfg(windows)]
    let home = std::env::var("USERPROFILE");

    #[cfg(not(windows))]
    let home = std::env::var("HOME");

    home
}

pub fn get_auth_token_and_port(
    spawn_elevated: bool,
    home: &Result<String, VarError>,
) -> Option<(String, u16)> {
    let mut port = 0_u16;
    let mut token = String::new();

    for p in [crate::GLOBAL_SERVICE_HOME_V2, crate::GLOBAL_SERVICE_HOME_V1] {
        let p = Path::new(p);
        for port_path in [p.join("zerotier.port"), p.join("zerotier-one.port")] {
            let _ = std::fs::read(port_path).map(|pp| {
                String::from_utf8(pp)
                    .map(|pp| u16::from_str_radix(pp.trim(), 10).map(|pp| port = pp))
            });
            if port != 0 {
                break;
            }
        }
    }

    for attempt in 0..2 {
        let _ = home.clone().map(|mut p| {
            #[cfg(target_os = "macos")]
            {
                p.push_str("/Library/Application Support/ZeroTier/One");
                let _ = std::fs::create_dir_all(&p);
                unsafe {
                    let cstr = CString::new(p.as_str()).unwrap();
                    crate::c_lock_down_file(cstr.as_ptr(), 1);
                }
                p.push_str("/authtoken.secret");
            }

            #[cfg(windows)]
            p.push_str("\\AppData\\Local\\ZeroTier\\One\\authtoken.secret");

            #[cfg(all(unix, not(target_os = "macos")))]
            p.push_str("/.zeroTierOneAuthToken");

            let _ = std::fs::read(p)
                .map(|tok| String::from_utf8(tok).map(|tok| token = tok.trim().into()));
        });

        if token.is_empty() {
            let _ = home.clone().map(|mut p| {
                #[cfg(target_os = "macos")]
                {
                    p.push_str("/Library/Application Support/ZeroTier");
                    let _ = std::fs::create_dir_all(&p);
                    unsafe {
                        let cstr = CString::new(p.as_str()).unwrap();
                        crate::c_lock_down_file(cstr.as_ptr(), 1);
                    }
                    p.push_str("/authtoken.secret");
                }

                #[cfg(windows)]
                p.push_str("\\AppData\\Local\\ZeroTier\\authtoken.secret");

                #[cfg(all(unix, not(target_os = "macos")))]
                p.push_str("/.zerotier-local-auth");

                let _ = std::fs::read(p)
                    .map(|tok| String::from_utf8(tok).map(|tok| token = tok.trim().into()));
            });
        }

        if token.is_empty() {
            for p in [crate::GLOBAL_SERVICE_HOME_V2, crate::GLOBAL_SERVICE_HOME_V1] {
                let p = Path::new(p);
                let _ = std::fs::read(p.join("authtoken.secret"))
                    .map(|tok| String::from_utf8(tok).map(|tok| token = tok.trim().into()));
                if !token.is_empty() {
                    break;
                }
            }

            if token.is_empty() {
                if spawn_elevated && attempt == 0 {
                    let home = home.as_ref().expect("home lookup error");
                    let runas_result = runas::Command::new(std::env::current_exe().unwrap())
                        .arg("copy_authtoken")
                        .arg(home)
                        .gui(true)
                        .status();
                    if runas_result.is_err() {
                        eprintln!("ERROR: unable to escalate privileges: {}", runas_result.err().unwrap().to_string());
                    }
                }
            } else {
                // Save in both places for now for backward compatibility.
                let _ = home.clone().map(|mut p| {
                    #[cfg(target_os = "macos")]
                    p.push_str("/Library/Application Support/ZeroTier/authtoken.secret");

                    #[cfg(windows)]
                    p.push_str("\\AppData\\Local\\ZeroTier\\authtoken.secret");

                    #[cfg(all(unix, not(target_os = "macos")))]
                    p.push_str("/.zerotier-local-auth");

                    let _ = std::fs::write(&p, token.as_bytes());
                });
                let _ = home.clone().map(|mut p| {
                    #[cfg(target_os = "macos")]
                    p.push_str("/Library/Application Support/ZeroTier/One/authtoken.secret");

                    #[cfg(windows)]
                    p.push_str("\\AppData\\Local\\ZeroTier\\One\\authtoken.secret");

                    #[cfg(all(unix, not(target_os = "macos")))]
                    p.push_str("/.zeroTierOneAuthToken");

                    let _ = std::fs::write(&p, token.as_bytes());
                });
            }
        }

        if !token.is_empty() {
            break;
        }
    }

    if token.is_empty() {
        None
    } else {
        if port == 0 {
            port = 9993;
        }
        Some((token, port))
    }
}

const SEP_BYTE: [u8; 1] = [0_u8];

fn hash_result(v: &Value, h: &mut crc64::Crc64) {
    let _ = h.write(&SEP_BYTE);
    match v {
        Value::Array(a) => {
            for x in a.iter() {
                hash_result(x, h);
            }
        }
        Value::Object(o) => {
            for x in o.iter() {
                let _ = h.write(x.0.as_bytes());
                if !x.0.eq("clock") && !x.0.eq("netconfRevision") {
                    // omit fields that change meaninglessly
                    hash_result(x.1, h);
                }
            }
        }
        Value::Bool(b) => {
            let _ = h.write(&[*b as u8]);
        }
        Value::Number(n) => {
            let _ = h.write(n.to_string().as_bytes());
        }
        Value::String(s) => {
            let _ = h.write(s.as_bytes());
        }
        _ => {}
    }
}

impl ServiceClient {
    /// Create a new service client and return the client and a flag that can be atomically checked to indicate changes.
    pub fn new(refresh_base_paths: Vec<&'static str>) -> (ServiceClient, Arc<AtomicBool>) {
        let dirty_flag = Arc::new(AtomicBool::new(true));
        (
            ServiceClient {
                refresh_base_paths,
                auth_token: String::new(),
                port: 0,
                base_url: String::new(),
                saved_networks: std::fs::read(unsafe { crate::NETWORK_CACHE_PATH.as_str() })
                    .map_or_else(
                        |_| serde_json::Map::new(),
                        |j| {
                            serde_json::from_slice(j.as_slice())
                                .map_or_else(|_| serde_json::Map::new(), |r| r)
                        },
                    ),
                state_hash: HashMap::new(),
                state: Map::new(),
                post_queue: LinkedList::new(),
                delete_queue: LinkedList::new(),
                dirty: dirty_flag.clone(),
                online: false,
                try_escalate_privs: 2, // try this twice at startup, but not forever
            },
            dirty_flag,
        )
    }

    #[inline(always)]
    pub fn is_initialized(&self) -> bool {
        self.port != 0 && !self.auth_token.is_empty()
    }

    #[inline(always)]
    pub fn is_online(&self) -> bool {
        self.is_initialized() && self.online
    }

    pub fn with<T: AsRef<str>, R, F: FnOnce(&Value) -> R>(&self, path: &[T], f: F) -> R {
        let mut m = Some(&self.state);
        let v = Cell::new(&Value::Null);
        for s in path.iter() {
            m.map_or_else(
                || {
                    v.replace(&Value::Null); // null if looking past tree of maps
                },
                |mv| {
                    mv.get((*s).as_ref()).map_or_else(
                        || {
                            v.replace(&Value::Null); // null if key not found
                        },
                        |nv| {
                            m = nv.as_object(); // sets to None if not a map
                            if m.is_none() {
                                v.replace(nv); // if not a map, it's a value
                            }
                        },
                    );
                },
            );
        }
        f(v.into_inner())
    }

    #[inline(always)]
    pub fn get<T: AsRef<str>>(&self, path: &[T]) -> Value {
        self.with(path, |v| v.clone())
    }

    #[inline(always)]
    pub fn get_str<T: AsRef<str>>(&self, path: &[T]) -> String {
        self.get(path)
            .as_str()
            .map_or_else(|| String::new(), |s| s.into())
    }

    #[inline(always)]
    pub fn get_all_json(&self) -> String {
        serde_json::to_string(&self.state).unwrap()
    }

    pub fn networks(&self) -> Vec<(String, Map<String, Value>)> {
        let mut nw: Vec<(String, Map<String, Value>)> = Vec::new();
        self.with(&["network"], |nws| {
            let _ = nws.as_array().map(|a| {
                a.iter().for_each(|network| {
                    let _ = network.as_object().map(|network| {
                        network.get("id").map(|id| {
                            id.as_str().map(|id| nw.push((id.into(), network.clone())));
                        });
                    });
                })
            });
        });
        nw.sort_by(|a, b| (*a).0.cmp(&((*b).0)));
        nw
    }

    /*
    pub fn network_has_error(&self, nwid: &str) -> bool {
        self.state.get("network").map_or(true, |network| network.as_array().map_or(true, |network| {
            let mut has_error = true;
            for n in network.iter() {
                if n.as_object().map_or(false, |n| {
                    n.get("id").map_or(false, |id| id.as_str().map_or(false, |id| id == nwid)) && n.get("status").map_or(false, |status| status.as_str().map_or(false, |status| status == "OK"))
                }) {
                    has_error = false;
                    break;
                }
            }
            has_error
        }))
    }
    */

    pub fn sso_auth_needed_networks(&self) -> Vec<(String, String, String)> {
        let mut nw: Vec<(String, String, String)> = Vec::new();
        self.with(&["network"], |nws| {
            let _ = nws.as_array().map(|a| {
                a.iter().for_each(|network| {
                    let _ = network.as_object().map(|network| {
                        let id = network.get("id").map_or("", |id| id.as_str().unwrap_or(""));
                        let sso_enabled = network
                            .get("ssoEnabled")
                            .map_or(false, |sso_enabled| sso_enabled.as_bool().unwrap_or(false));
                        let auth_url = network
                            .get("authenticationURL")
                            .map_or("", |auth_url| auth_url.as_str().unwrap_or(""));
                        let status = network
                            .get("status")
                            .map_or("", |status| status.as_str().unwrap_or(""));
                        if sso_enabled && !auth_url.is_empty() {
                            if status == "AUTHENTICATION_REQUIRED" {
                                //println!("Auth Required for {}", id);
                                nw.push((id.into(), auth_url.into(), status.into()));
                            }
                        }
                    });
                })
            });
        });
        nw.sort_by(|a, b| (*a).0.cmp(&((*b).0)));
        nw
    }

    pub fn saved_networks(&self) -> Vec<(String, String, String)> {
        let mut nw: Vec<(String, String, String)> = Vec::new();
        for kv in self.saved_networks.iter() {
            let _ = kv.1.as_object().map(|n| {
                n.get("id").map(|id| {
                    id.as_str().map(|id| {
                        if id.len() == 16 {
                            nw.push((
                                id.into(),
                                n.get("name").map_or_else(
                                    || String::new(),
                                    |name| name.as_str().unwrap_or("").into(),
                                ),
                                n.get("settings").map_or_else(
                                    || String::new(),
                                    |name| name.as_str().unwrap_or("").into(),
                                ),
                            ));
                        }
                    })
                })
            });
        }
        nw.sort_by(|a, b| (*a).0.cmp(&((*b).0)));
        nw
    }

    /*
    pub fn peers(&self) -> Vec<(String, Map<String, Value>)> {
        let mut pp: Vec<(String, Map<String, Value>)> = Vec::new();
        self.with(&["peer"], |nws| {
            let _ = nws.as_array().map(|a| a.iter().for_each(|peer| {
                let _ = peer.as_object().map(|peer| {
                    peer.get("address").map(|id| {
                        id.as_str().map(|id| {
                            pp.push((id.into(), peer.clone()));
                        });
                    });
                });
            }));
        });
        pp.sort_by(|a, b| (*a).0.cmp(&((*b).0)) );
        pp
    }
    */

    fn http_get(&self, path: &str) -> (u16, String) {
        if self.auth_token.is_empty() || self.base_url.is_empty() {
            (0, String::new())
        } else {
            ureq::get(format!("{}{}", self.base_url, path).as_str())
                .timeout(Duration::from_millis(QUERY_TIMEOUT_MS))
                .set("X-ZT1-Auth", self.auth_token.as_str())
                .call()
                .map_or_else(
                    |e| match e {
                        ureq::Error::Status(status, _) => (status, String::new()),
                        _ => (0, String::new()),
                    },
                    |res| {
                        let status = res.status();
                        let body = res.into_string();
                        body.map_or_else(|_| (0, String::new()), |res| (status, res))
                    },
                )
        }
    }

    fn http_post(&self, path: &str, payload: &str) -> (u16, String) {
        //println!("POST {} {}", path, payload);
        if self.auth_token.is_empty() || self.base_url.is_empty() {
            (0, String::new())
        } else {
            ureq::post(format!("{}{}", self.base_url, path).as_str())
                .timeout(Duration::from_millis(QUERY_TIMEOUT_MS))
                .set("X-ZT1-Auth", self.auth_token.as_str())
                .send_string(payload)
                .map_or_else(
                    |e| match e {
                        ureq::Error::Status(status, _) => (status, String::new()),
                        _ => (0, String::new()),
                    },
                    |res| {
                        let status = res.status();
                        let body = res.into_string();
                        body.map_or_else(|_| (0, String::new()), |res| (status, res))
                    },
                )
        }
    }

    #[inline(always)]
    pub fn enqueue_post(&mut self, path: String, payload: String) {
        self.post_queue.push_back((path, payload));
    }

    #[inline(always)]
    pub fn enqueue_delete(&mut self, path: String) {
        self.delete_queue.push_back(path);
    }

    /// Check auth token and port for running service and update if changed.
    pub fn sync_client_config(&mut self) {
        let homedir = get_user_home_dir();
        if let Some((token, port)) = get_auth_token_and_port(
            if self.auth_token.is_empty() {
                self.try_escalate_privs = self.try_escalate_privs.saturating_sub(1);
                self.try_escalate_privs > 0
            } else {
                false
            },
            &homedir,
        ) {
            if self.auth_token != token || self.port != port {
                self.auth_token = token;
                self.port = port;
                self.base_url = format!("http://127.0.0.1:{}/", self.port);
            }
        }
    }

    /// Send enqueued posts, if there are any.
    pub fn do_posts(&mut self) -> bool {
        if !self.is_initialized() || !self.is_online() {
            self.sync_client_config();
        }
        if self.is_initialized() {
            let mut posted = false;
            loop {
                let pq = self.post_queue.pop_front();
                if pq.is_some() {
                    let pq = pq.unwrap();
                    posted = true;
                    let _ = self.http_post(pq.0.as_str(), pq.1.as_str());
                } else {
                    break;
                }
            }
            loop {
                let pq = self.delete_queue.pop_front();
                if pq.is_some() {
                    let pq = pq.unwrap();
                    posted = true;
                    let _ = ureq::delete(format!("{}{}", self.base_url, pq).as_str())
                        .timeout(Duration::from_millis(QUERY_TIMEOUT_MS))
                        .set("X-ZT1-Auth", self.auth_token.as_str())
                        .call()
                        .map_or(0_u16, |res| res.status());
                } else {
                    break;
                }
            }
            if posted {
                self.dirty.store(true, Ordering::Relaxed);
            }
            posted
        } else {
            false
        }
    }

    pub fn remember_network(&mut self, id: String, name: String, settings: String) {
        let mut n: serde_json::Map<String, Value> = serde_json::Map::new();
        n.insert("id".into(), Value::from(id.clone()));
        n.insert("name".into(), Value::from(name));
        n.insert("settings".into(), Value::from(settings));
        self.saved_networks.insert(id, Value::from(n));
        self.state.insert(
            "saved_networks".into(),
            serde_json::Value::from(self.saved_networks.clone()),
        );
        let _ = serde_json::to_vec(&self.saved_networks)
            .map(|json| std::fs::write(unsafe { crate::NETWORK_CACHE_PATH.as_str() }, &json));
        self.dirty.store(true, Ordering::Relaxed);
    }

    pub fn forget_network(&mut self, id: &String) {
        self.saved_networks.remove(id);
        self.state.insert(
            "saved_networks".into(),
            serde_json::Value::from(self.saved_networks.clone()),
        );
        let _ = serde_json::to_vec(&self.saved_networks)
            .map(|json| std::fs::write(unsafe { crate::NETWORK_CACHE_PATH.as_str() }, &json));
        self.dirty.store(true, Ordering::Relaxed);
    }

    /// Submit queued posts and get current service state.
    pub fn sync(&mut self) {
        if !self.is_initialized() || !self.is_online() {
            self.sync_client_config();
            self.state.insert(
                "saved_networks".into(),
                serde_json::Value::from(self.saved_networks.clone()),
            );
        }
        if self.is_initialized() {
            let mut dirty = false;
            for endpoint in self.refresh_base_paths.iter() {
                let endpoint = *endpoint;
                let data = self.http_get(endpoint);
                if data.0 == 200 {
                    let endpoint = String::from(endpoint);
                    let data =
                        serde_json::from_str::<Value>(data.1.as_str()).unwrap_or(Value::Null);

                    let mut c64 = crc64::Crc64::new();
                    hash_result(&data, &mut c64);
                    let c64 = c64.get();

                    self.online = true;
                    if self.state_hash.insert(endpoint.clone(), c64).unwrap_or(0) != c64 {
                        self.state.insert(endpoint, data);
                        self.dirty.store(true, Ordering::Relaxed);
                        dirty = true;
                    }
                } else if data.0 == 0 {
                    self.online = false;
                } else {
                    self.online = false;
                    // HACK: exit on failed authentication if we are finished trying to request an auth token.
                    if self.try_escalate_privs == 0 {
                        std::process::exit(1);
                    } else {
                        self.auth_token.clear();
                        self.port = 0;
                    }
                }
            }
            if dirty {
                self.state.insert(
                    "saved_networks".into(),
                    serde_json::Value::from(self.saved_networks.clone()),
                );
            }
        }
    }
}
