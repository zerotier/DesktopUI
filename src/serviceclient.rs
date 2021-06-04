use std::sync::Mutex;
use std::time::{SystemTime, Duration};
use serde_json::Value;

const SERVICE_TIMEOUT_SECONDS: u64 = 5;

pub struct ServiceClientInner {
    last_successful_refresh_time: SystemTime,
    pub current_status: String,
    pub current_networks: String,
    pub current_peers: String,
}

/// Client that queries and caches JSON state from the service.
/// Currently it doesn't do much parsing since the JSON is just shoved
/// through to the JavaScript UI for most of what's done with it.
pub struct ServiceClient {
    auth_token: String,
    base_url: String,
    inner: Mutex<ServiceClientInner>,
}

#[cfg(target_os = "macos")]
pub fn get_auth_token_and_port() -> Option<(String, u16)> {
    let port = std::fs::read_to_string("/Library/Application Support/ZeroTier/One/zerotier-one.port").map_or(9993_u16, |port| u16::from_str_radix(port.as_str(), 10).unwrap_or(9993_u16) );

    let home = std::env::var("HOME");
    if home.is_ok() {
        let mut p = home.unwrap();
        p.push_str("/Library/Application Support/ZeroTier/One/authtoken.secret");
        let token = std::fs::read_to_string(p);
        if token.is_ok() {
            return Some((token.unwrap().trim().into(), port));
        }
    }

    let token = std::fs::read_to_string("/Library/Application Support/ZeroTier/One/authtoken.secret");
    if token.is_ok() {
        return Some((token.unwrap().trim().into(), port));
    }

    None
}

impl ServiceClient {
    pub fn new(auth_token: String, port: u16) -> ServiceClient {
        ServiceClient{
            auth_token,
            base_url: format!("http://127.0.0.1:{}/", port),
            inner: Mutex::new(ServiceClientInner{
                last_successful_refresh_time: SystemTime::UNIX_EPOCH,
                current_status: String::new(),
                current_networks: String::new(),
                current_peers: String::new()
            })
        }
    }

    pub fn is_online(&self) -> bool {
        SystemTime::now().duration_since(self.inner.lock().unwrap().last_successful_refresh_time).map_or(false, |d| d.as_secs() > SERVICE_TIMEOUT_SECONDS)
    }

    pub fn refresh(&self) {
        // Yuck, but okay for now...
        let timeout = Duration::from_secs(SERVICE_TIMEOUT_SECONDS);
        let res = ureq::get(format!("{}status", self.base_url).into()).timeout(timeout).set("X-ZT1-Auth", self.auth_token.into()).call();
        if res.is_ok() {
            let res = res.unwrap();
            if res.status() == 200 {
                let status = res.into_string();
                if status.is_ok() {
                    let res = ureq::get(format!("{}network", self.base_url).into()).timeout(timeout).set("X-ZT1-Auth", self.auth_token.into()).call();
                    if res.is_ok() {
                        let res = res.unwrap();
                        if res.status() == 200 {
                            let networks = res.into_string();
                            if networks.is_ok() {
                                let res = ureq::get(format!("{}peer", self.base_url).into()).timeout(timeout).set("X-ZT1-Auth", self.auth_token.into()).call();
                                if res.is_ok() {
                                    let res = res.unwrap();
                                    if res.status() == 200 {
                                        let peers = res.into_string();
                                        if peers.is_ok() {
                                            let mut inner = self.inner.lock().unwrap();
                                            inner.last_successful_refresh_time = SystemTime::now();
                                            inner.current_status = status.unwrap();
                                            inner.current_networks = networks.unwrap();
                                            inner.current_peers = peers.unwrap();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn networks(&self) -> Vec<(u64, String)> {
        let mut l = Vec::new();
        let networks: Value = serde_json::from_str(self.inner.lock().unwrap().current_networks.as_str()).unwrap_or(Value::Null);
        let networks_array = networks.as_array();
        if networks_array.is_some() {
            let networks_array = networks_array.unwrap();
            for nw in networks_array.iter() {
                let network = nw.as_object();
                if network.is_some() {
                    let network = network.unwrap();
                    let network_id = u64::from_str_radix(network.get("id").unwrap_or(&Value::Null).as_str().unwrap_or(""), 16).unwrap_or(0);
                    let network_name = network.get("name").unwrap_or(&Value::Null).as_str().unwrap_or("");
                    if network_id != 0 {
                        l.push((network_id, network_name.into()));
                    }
                }
            }
        }
        l.sort_by(|a, b| (*a).0.cmp(&(*b).0));
        l
    }

    pub fn address(&self) -> u64 {
        let status: Value = serde_json::from_str(self.inner.lock().unwrap().current_status.as_str()).unwrap_or(Value::Null);
        let status_obj = status.as_object();
        if status_obj.is_some() {
            let address = status_obj.unwrap().get("address");
            if address.is_some() {
                return u64::from_str_radix(address.unwrap().as_str().unwrap_or(""), 16).unwrap_or(0);
            }
        }
        0
    }
}
