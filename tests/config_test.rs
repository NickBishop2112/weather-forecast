use std::env;
use std::fs::File;
use serial_test::serial;
use temp_env::with_var;
use tempfile::tempdir;
use weather::config::settings::{init_config, ConfigProvider, RealConfigProvider};
use weather::Result;
use std::io::{Write};
#[actix_web::test]
async fn test_get_config() -> Result<()> {
    with_var("OPENWEATHER_API_KEY", Some("abc"), || {
        init_config(env::current_dir().expect("Current folder should be set")).expect("initial config");

        let config_provider = RealConfigProvider;

        let config = config_provider.get_config().unwrap();

        assert_eq!(config.openweather_api_key, "abc");
    });

    Ok(())
}

#[actix_web::test]
#[serial]
async fn init_config_env_file_not_found() -> Result<()> {

    // Arrange
    let dir = tempdir().expect("Failed to create temp directory");
    
    // Act
    let result = init_config(dir.into_path());

    // Assert
    match &result {
        Ok(value) => panic!("init config should be false, but it is {:?}", value),
        Err(err) => assert_eq!(err.to_string(), "Config error: The system cannot find the file specified. (os error 2)")
    }     
   
    Ok(())
}

#[actix_web::test]
#[serial]
async fn init_config_env_file_not_loaded() -> Result<()> {

    // Arrange
    let dir = tempdir().expect("Failed to create temp directory");
    let env_file_path = dir.path().join(".env");

    let mut env_file = File::create(&env_file_path).expect("Failed to create .env file");
    writeln!(env_file, "MY_ENV_VAR=HelloWorld").expect("Failed to write to .env file");

    // Act
    let result = init_config(dir.into_path());

    // Assert
    match &result {
        Ok(value) => panic!("init config should be false, but it is {:?}", value),
        Err(err) => assert_eq!(err.to_string(), "Config error: missing field `openweather_api_key`")
    }

    Ok(())
}
