use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub host_name: String,
    pub port: u16,
    pub database_url: String,
}

pub fn load_config(env: &str) -> Result<Server, config::ConfigError> {
    let path = format!("common/config/server/{}.toml", env);
    load_config_from_path(&path)
}

fn load_config_from_path(path: &str) -> Result<Server, config::ConfigError> {
    Config::builder()
        .add_source(File::with_name(path))
        .build()?
        .try_deserialize::<Server>()
}

#[cfg(test)]
mod tests {
    use super::load_config_from_path;

    #[test]
    fn loads_environment_configs() {
        let config_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/config/server");
        let local = load_config_from_path(&format!("{config_dir}/local.toml")).unwrap();
        assert_eq!(local.host_name, "127.0.0.1");
        assert_eq!(local.port, 8080);
        assert_eq!(local.database_url, "sqlite://resources/db/honey_note.db");

        let prod = load_config_from_path(&format!("{config_dir}/prod.example.toml")).unwrap();
        assert_eq!(prod.host_name, "127.0.0.1");
        assert_eq!(prod.port, 8081);
        assert_eq!(
            prod.database_url,
            "sqlite:///var/lib/honey-note/honey_note.db"
        );
    }

    #[test]
    fn reports_missing_config() {
        let result = load_config_from_path(&format!(
            "{}/config/server/missing.toml",
            env!("CARGO_MANIFEST_DIR")
        ));
        assert!(result.is_err());
    }
}
