use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
    pub hosts: Vec<Host>
}

#[derive(Deserialize)]
pub struct Host {
    pub label: String,
    pub on: String
}
