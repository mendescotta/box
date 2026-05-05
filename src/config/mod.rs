use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub mod_key: Option<String>,
}

pub fn load() -> Config {
    Config { mod_key: Some("Alt".into()) }
}
