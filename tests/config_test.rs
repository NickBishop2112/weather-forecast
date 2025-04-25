use serial_test::serial;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;
use weather::Result;
use weather::config::settings::{get_config, init_config};
#[actix_web::test]
#[serial]
async fn test_get_config() -> Result<()> {
    // Arrange
    let dir = tempdir().expect("Failed to create temp directory");
    let env_file_path = dir.path().join(".env");
    let mut env_file = File::create(&env_file_path).expect("Failed to create .env file");
    write!(env_file, "OPENWEATHER_API_KEY=abc").expect("Failed to write to .env file");

    init_config(dir.into_path()).expect("initial config");

    // Act
    let config = get_config().unwrap();

    // Assert
    assert_eq!(config.openweather_api_key, "abc");

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
        Err(err) => assert_eq!(
            err.to_string(),
            "Config error: The system cannot find the file specified. (os error 2)"
        ),
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
        Err(err) => assert_eq!(
            err.to_string(),
            "Config error: missing field `openweather_api_key`"
        ),
    }

    Ok(())
}

#[actix_web::test]
#[serial]
async fn get_config_config_not_loaded() -> Result<()> {
    // Arrange

    // Act
    let result = get_config();

    // Assert
    match &result {
        Ok(value) => panic!("init config should be false, but it is {:?}", value),
        Err(err) => assert_eq!(
            err.to_string(),
            "Config error: Configuration not initialized"
        ),
    }

    Ok(())
}
