use std::path::Path;

use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub host_name: String,
    pub port: u16,
}

pub fn load_config(env: &str) -> Option<Server> {
    let path = format!("common/config/server/{}.toml", env);
    if !Path::new(&path).exists() {
    println!("ファイルが見つかりません: {}", path);
}
    Config::builder()
        .add_source(File::with_name(path.as_str()))
        .build()
        .ok()?
        .try_deserialize::<Server>()
        .ok()
}
