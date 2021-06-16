use std::time::Duration;
use std::collections::{LinkedList, HashMap};
use serde_json::{Map, Value};
use std::cell::Cell;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

const QUERY_TIMEOUT_MS: u64 = 2000;

/// Client that queries and caches JSON state from the service.
/// Currently it doesn't do much parsing since the JSON is just shoved
/// through to the JavaScript UI for most of what's done with it.
pub struct ServiceClient {
    refresh_base_paths: Vec<&'static str>,
    auth_token: String,
    port: u16,
    base_url: String,
    state: Map<String, Value>,
    state_crc64: HashMap<String, u64>,
    post_queue: LinkedList<(String, String)>,
    dirty: Arc<AtomicBool>,
    online: bool,
}

#[cfg(target_os = "macos")]
const ZT_SERVICE_GLOBAL_ROOT: &'static str = "/Library/Application Support/ZeroTier/One/";

#[cfg(windows)]
const ZT_SERVICE_GLOBAL_ROOT: &'static str = "\\ProgramData\\ZeroTier\\One\\";

pub fn get_auth_token_and_port() -> Option<(String, u16)> {
    let port = std::fs::read_to_string(format!("{}zerotier-one.port", ZT_SERVICE_GLOBAL_ROOT).as_str()).map_or(9993_u16, |port| u16::from_str_radix(port.as_str(), 10).unwrap_or(9993_u16) );

    #[cfg(windows)]
    let home = std::env::var("USERPROFILE");
    #[cfg(not(windows))]
    let home = std::env::var("HOME");

    if home.is_ok() {
        let mut p = home.unwrap();

        #[cfg(target_os = "macos")]
        p.push_str(format!("/Library/Application Support/ZeroTier/One/authtoken.secret").as_str());

        #[cfg(windows)]
        p.push_str(format!("\\AppData\\Local\\ZeroTier\\One").as_str());

        #[cfg(all(unix, not(target_os = "macos")))]
        p.push_str(format!("/.zeroTierOneAuthToken").as_str());

        let token = std::fs::read_to_string(p);
        if token.is_ok() {
            return Some((token.unwrap().trim().into(), port));
        }
    }

    let token = std::fs::read_to_string(format!("{}authtoken.secret", ZT_SERVICE_GLOBAL_ROOT).as_str());
    if token.is_ok() {
        return Some((token.unwrap().trim().into(), port));
    }

    None
}

impl ServiceClient {
    /// Create a new service client and return the client and a flag that can be atomically checked to indicate changes.
    pub fn new(refresh_base_paths: Vec<&'static str>) -> (ServiceClient, Arc<AtomicBool>) {
        let dirty_flag = Arc::new(AtomicBool::new(true));
        (ServiceClient {
            refresh_base_paths,
            auth_token: String::new(),
            port: 0,
            base_url: String::new(),
            state: Map::new(),
            state_crc64: HashMap::new(),
            post_queue: LinkedList::new(),
            dirty: dirty_flag.clone(),
            online: false,
        }, dirty_flag)
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
            m.map_or_else(|| {
                v.replace(&Value::Null); // null if looking past tree of maps
            }, |mv| {
                mv.get((*s).as_ref()).map_or_else(|| {
                    v.replace(&Value::Null); // null if key not found
                }, |nv| {
                    m = nv.as_object(); // sets to None if not a map
                    if m.is_none() {
                        v.replace(nv); // if not a map, it's a value
                    }
                });
            });
        }
        f(v.into_inner())
    }

    #[inline(always)]
    pub fn get<T: AsRef<str>>(&self, path: &[T]) -> Value {
        self.with(path, |v| v.clone())
    }

    #[inline(always)]
    pub fn get_str<T: AsRef<str>>(&self, path: &[T]) -> String {
        self.get(path).as_str().map_or_else(|| String::new(), |s| s.into())
    }

    #[inline(always)]
    pub fn get_all_json(&self) -> String {
        serde_json::to_string(&self.state).unwrap()
    }

    pub fn networks(&self) -> Vec<(String, Map<String, Value>)> {
        let mut nw: Vec<(String, Map<String, Value>)> = Vec::new();
        self.with(&["network"], |nws| {
            let _ = nws.as_array().map(|a| a.iter().for_each(|network| {
                let _ = network.as_object().map(|network| {
                    network.get("id").map(|id| {
                        id.as_str().map(|id| {
                            nw.push((id.into(), network.clone()))
                        });
                    });
                });
            }));
        });
        nw.sort_by(|a, b| (*a).0.cmp(&((*b).0)) );
        nw
    }

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

    fn http_get(&self, path: &str) -> (u16, String) {
        if self.auth_token.is_empty() || self.base_url.is_empty() {
            (0, String::new())
        } else {
            ureq::get(format!("{}{}", self.base_url, path).as_str()).timeout(Duration::from_millis(QUERY_TIMEOUT_MS)).set("X-ZT1-Auth", self.auth_token.as_str()).call().map_or_else(|_| {
                (0, String::new())
            }, |res| {
                let status = res.status();
                let body = res.into_string();
                body.map_or_else(|_| {
                    (0, String::new())
                }, |res| {
                    (status, res)
                })
            })
        }
    }

    fn http_post(&self, path: &str, payload: &str) -> (u16, String) {
        //println!("POST {} {}", path, payload);
        if self.auth_token.is_empty() || self.base_url.is_empty() {
            (0, String::new())
        } else {
            ureq::post(format!("{}{}", self.base_url, path).as_str()).timeout(Duration::from_millis(QUERY_TIMEOUT_MS)).set("X-ZT1-Auth", self.auth_token.as_str()).send_string(payload).map_or_else(|_| {
                (0, String::new())
            }, |res| {
                let status = res.status();
                let body = res.into_string();
                body.map_or_else(|_| {
                    (0, String::new())
                }, |res| {
                    (status, res)
                })
            })
        }
    }

    #[inline(always)]
    pub fn enqueue_post(&mut self, path: String, payload: String) {
        self.post_queue.push_back((path, payload));
    }

    /// Check auth token and port for running service and update if changed.
    pub fn sync_client_config(&mut self) {
        get_auth_token_and_port().map(|token_port| {
            if self.auth_token != token_port.0 || self.port != token_port.1 {
                self.auth_token = token_port.0.clone();
                self.port = token_port.1;
                self.base_url = format!("http://127.0.0.1:{}/", self.port);
            }
        });
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
                    if self.http_post(pq.0.as_str(), pq.1.as_str()).0 == 200 {
                        posted = true;
                    } else {
                        self.post_queue.push_front(pq);
                        break;
                    }
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

    /// Submit queued posts and get current service state.
    pub fn sync(&mut self) {
        if !self.is_initialized() || !self.is_online() {
            self.sync_client_config();
        }
        if self.is_initialized() {
            for endpoint in self.refresh_base_paths.iter() {
                let endpoint = *endpoint;
                let data = self.http_get(endpoint);
                if data.0 == 200 {
                    let mut c64 = crc64fast::Digest::new();
                    c64.write(data.1.as_bytes());
                    let c64 = c64.sum64();
                    if self.state_crc64.insert(endpoint.into(), c64).map_or(true, |s| s != c64) {
                        self.state.insert((*endpoint).into(), serde_json::from_str::<Value>(data.1.as_str()).unwrap_or(Value::Null));
                        self.dirty.store(true, Ordering::Relaxed);
                        self.online = true;
                    }
                }
            }
        }
    }
}
