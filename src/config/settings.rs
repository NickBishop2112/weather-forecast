use crate::error::{Error, Result};
use figment::{Figment, providers::Env};
use log::error;
use mockall::automock;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::path::PathBuf;

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

pub fn init_config(directory: PathBuf) -> Result<()> {
    let env_path = directory.join(".env");

    dotenvy::from_path(env_path).map_err(|e| {
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

    CONFIG.set(config).map_err(|_| Error::ConfigError {
        message: "Configuration already initialized".to_string(),
    })?;
    Ok(())
}

#[automock]
pub trait ConfigProvider {
    fn get_config(&self) -> Result<AppConfig>;
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
