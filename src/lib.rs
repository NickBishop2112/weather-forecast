mod config;
mod error;

pub mod routes;
pub mod models;
pub mod handlers;
pub mod services;

pub use error::{Error, Result};