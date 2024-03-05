pub mod auth;
pub mod db;
pub mod jobs;
pub mod users;

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    SqlXError(#[from] sqlx::Error),

    #[error("Failed to read environment variable")]
    EnvironmentVariableReadFailure(#[from] std::env::VarError),

    #[error("Failed to read sql files")]
    SqlFileReadFailure(#[from] std::io::Error),

    #[error("Failed to apply migrations")]
    MigrationFailed(#[from] sqlx::migrate::MigrateError),

    #[error("Invalid application data: {0}")]
    InvalidApplicationData(String),

    #[error("Not authorized to perform this action")]
    NotAuthorized,

    #[error("{0} already in use")]
    EmailTaken(String),

    #[error("POST or UPDATE data is empty or missing")]
    MissingData,
}
