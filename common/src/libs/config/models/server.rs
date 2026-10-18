use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub host_name: String,
    pub port: u16,
    pub database_url: String,
    #[serde(default)]
    pub rate_limit: RateLimit,
}

/// 環境別のレート制限。未設定の値は従来の制限を引き継ぐ。
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct RateLimit {
    #[serde(deserialize_with = "deserialize_request_interval_ms")]
    pub request_interval_ms: u32,
    #[serde(deserialize_with = "deserialize_burst_size")]
    pub burst_size: u32,
}

impl Default for RateLimit {
    fn default() -> Self {
        Self {
            request_interval_ms: 2000,
            burst_size: 20,
        }
    }
}

fn deserialize_request_interval_ms<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserialize_positive_u32(deserializer, "rate_limit.request_interval_ms")
}

fn deserialize_burst_size<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserialize_positive_u32(deserializer, "rate_limit.burst_size")
}

/// 設定ライブラリによる文字列・小数から整数への暗黙変換を避ける。
fn deserialize_positive_u32<'de, D>(deserializer: D, field: &'static str) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct PositiveIntegerVisitor(&'static str);

    impl<'de> serde::de::Visitor<'de> for PositiveIntegerVisitor {
        type Value = u32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "{} に指定する1以上4294967295以下の整数", self.0)
        }

        fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<u32, E> {
            u32::try_from(value)
                .ok()
                .filter(|value| *value > 0)
                .ok_or_else(|| {
                    E::custom(format!(
                        "{} は1以上4294967295以下の整数を指定してください",
                        self.0
                    ))
                })
        }

        fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<u32, E> {
            let value = u64::try_from(value)
                .map_err(|_| E::custom(format!("{} は1以上の整数を指定してください", self.0)))?;
            self.visit_u64(value)
        }
    }

    deserializer.deserialize_any(PositiveIntegerVisitor(field))
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
    use super::{load_config_from_path, Server};
    use config::{Config, File, FileFormat};

    fn config_with_rate_limit(section: &str) -> Result<Server, config::ConfigError> {
        let contents = format!(
            "host_name = \"127.0.0.1\"\nport = 8080\ndatabase_url = \"sqlite::memory:\"\n{section}"
        );
        Config::builder()
            .add_source(File::from_str(&contents, FileFormat::Toml))
            .build()?
            .try_deserialize()
    }

    #[test]
    fn loads_environment_configs() {
        let config_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/config/server");
        let local = load_config_from_path(&format!("{config_dir}/local.toml")).unwrap();
        assert_eq!(local.host_name, "127.0.0.1");
        assert_eq!(local.port, 8080);
        assert_eq!(local.database_url, "sqlite://resources/db/honey_note.db");
        assert_eq!(local.rate_limit.request_interval_ms, 100);
        assert_eq!(local.rate_limit.burst_size, 60);

        let prod = load_config_from_path(&format!("{config_dir}/prod.example.toml")).unwrap();
        assert_eq!(prod.host_name, "127.0.0.1");
        assert_eq!(prod.port, 8081);
        assert_eq!(
            prod.database_url,
            "sqlite:///var/lib/honey-note/honey_note.db"
        );
        assert_eq!(prod.rate_limit.request_interval_ms, 2000);
        assert_eq!(prod.rate_limit.burst_size, 20);
    }

    #[test]
    fn reports_missing_config() {
        let result = load_config_from_path(&format!(
            "{}/config/server/missing.toml",
            env!("CARGO_MANIFEST_DIR")
        ));
        assert!(result.is_err());
    }

    #[test]
    fn defaults_rate_limit_for_existing_configs() {
        let config = config_with_rate_limit("").unwrap();
        assert_eq!(config.rate_limit.request_interval_ms, 2000);
        assert_eq!(config.rate_limit.burst_size, 20);
    }

    #[test]
    fn defaults_each_missing_rate_limit_field() {
        for (section, interval, burst) in [
            ("[rate_limit]", 2000, 20),
            ("[rate_limit]\nrequest_interval_ms = 250", 250, 20),
            ("[rate_limit]\nburst_size = 40", 2000, 40),
        ] {
            let config = config_with_rate_limit(section).unwrap();
            assert_eq!(config.rate_limit.request_interval_ms, interval);
            assert_eq!(config.rate_limit.burst_size, burst);
        }
    }

    #[test]
    fn rejects_invalid_rate_limit_values() {
        for field in ["request_interval_ms", "burst_size"] {
            for value in ["0", "-1", "1.5", "true", "\"100\"", "4294967296"] {
                let error = config_with_rate_limit(&format!("[rate_limit]\n{field} = {value}"))
                    .unwrap_err()
                    .to_string();
                assert!(error.contains(field), "{field} = {value}: {error}");
            }
        }
    }
}
