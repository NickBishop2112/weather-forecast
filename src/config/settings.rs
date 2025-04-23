use crate::error::{Error, Result};
use figment::{Figment, providers::Env};
use log::error;
use mockall::automock;
use once_cell::sync::OnceCell;
use serde::Deserialize;

#[derive(Default, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub openweather_api_key: String,
}

static CONFIG: OnceCell<AppConfig> = OnceCell::new();

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Error::ConfigError {
            message: e.to_string(),
        }
    }
}

fn load_config() -> Result<AppConfig> {
    println!("Loading config...0");
    dotenvy::dotenv().map_err(|e| {
        error!("Failed to load .env file: {}", e);
        Error::ConfigError {
            message: e.to_string(),
        }
    })?;

    println!("Loading config...1");
    let config: AppConfig = Figment::new().merge(Env::raw()).extract().map_err(|e| {
        error!("Failed to load configuration: {}", e);
        Error::ConfigError {
            message: e.to_string(),
        }
    })?;

    println!("Loading config...2");
    Ok(config)
}

pub fn init_config() -> Result<()> {
    let config = load_config()?;
    CONFIG.set(config).map_err(|_| Error::ConfigError {
        message: "Configuration already initialized".to_string(),
    })?;
    Ok(())
}

#[automock]
pub trait ConfigProvider {
    fn get_config(& self) -> Result<AppConfig>;
}

pub struct RealConfigProvider;

impl ConfigProvider for RealConfigProvider {
    fn get_config(&self) -> Result<AppConfig> {
        CONFIG.get().cloned().ok_or_else(|| {
            error!("Failed to load configuration:");
            Error::ConfigError {
                message: "Configuration not initialized".to_string(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use temp_env::with_var;

    use super::*;
    use crate::error::Result;

    #[actix_web::test]
    async fn test_get_config() -> Result<()> {
        with_var("OPENWEATHER_API_KEY", Some("abc"), || {
            init_config().expect("initial config");

            let config_provider = RealConfigProvider;

            let config = config_provider.get_config().unwrap();

            assert_eq!(config.openweather_api_key, "abc");
        });

        Ok(())
    }

    #[actix_web::test]
    async fn test_get_config1() -> Result<()> {
        with_var("OPENWEATHER_API_KEY", None::<&str>, || {
            let result = init_config();

            assert!(result.is_err());
        });

        Ok(())
    }
}
