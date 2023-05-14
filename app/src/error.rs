use config::ConfigError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("internal error: {0}")]
    Internal(#[from] Box<dyn std::error::Error + Send + Sync>),
}

macro_rules! impl_internal_errors {
    ( $( $type:ty ),* $(,)? ) => {
        $(
        impl From<$type> for AppError {
            fn from(err: $type) -> Self {
                AppError::Internal(Box::new(err))
            }
        }
        )*
    };
}
impl_internal_errors!(ConfigError,);

pub type AppResult<T> = Result<T, AppError>;
