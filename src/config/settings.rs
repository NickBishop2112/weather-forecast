use crate::error::{Error, Result};
use figment::{Figment, providers::Env};
use log::error;
use mockall::automock;
use once_cell::sync::OnceCell;
use serde::Deserialize;

#[derive(Default, Deserialize, Debug)]
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
    dotenvy::dotenv().map_err(|e| {
        error!("Failed to load .env file: {}", e);
        Error::ConfigError {
            message: e.to_string(),
        }
    })?;

    let config: AppConfig = Figment::new().merge(Env::raw()).extract().map_err(|e| {
        error!("Failed to load configuration: {}", e);
        Error::ConfigError {
            message: e.to_string(),
        }
    })?;

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

pub trait ConfigProvider: Send + Sync {
    fn get_config(&self) -> Result<&'static AppConfig>;
}
pub struct RealConfigProvider;

impl ConfigProvider for RealConfigProvider {
    fn get_config(&self) -> Result<&'static AppConfig> {
        CONFIG.get().ok_or_else(|| {
            error!("Failed to load configuration:");
            Error::ConfigError {
                message: "Configuration not initialized".to_string(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;

    #[actix_web::test]
    async fn test_get_config() -> Result<()> {
        init_config()?;

        let config_provider = RealConfigProvider;

        let config = config_provider.get_config()?;
        assert_eq!(
            config.openweather_api_key,
            "52eab1a47f693f5383a897a464ab5ce4"
        );
        Ok(())
    }
}
