use std::env;
use config::{ConfigError, Config, File, Environment};
use serde::Deserialize;

pub struct SettingsLoader;

impl SettingsLoader {
    pub fn load<'de, T>() -> Result<T, ConfigError> where T: Deserialize<'de> {
        let mut s = Config::new();

        // load defaults from config/default.toml
        s.merge(File::with_name("config/default"))?;

        // get run mode from environment variables so we can override this on other environments
        let env = env::var("RUN_MODE").unwrap_or("development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // load local overrides from config/local.toml
        // note that this file isn't checked in to git
        s.merge(File::with_name("config/local").required(false))?;

        // load overrides from environment variables with the prefix "APP_"
        s.merge(Environment::with_prefix("app").separator("_"))?;

        println!("Configurations for {:?} environment is now loaded.", env);

        if s.get_bool("debug").unwrap_or(false) {
            println!("Debug is enabled");
        }

        s.try_into()
    }
}
