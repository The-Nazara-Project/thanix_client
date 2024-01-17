use std::sync::Mutex;

pub static REQWEST_CLIENT: Mutex<Option<reqwest::blocking::Client>> = Mutex::new(None);
pub static REQWEST_BASE_URL: &str = "https://demo.netbox.dev";
