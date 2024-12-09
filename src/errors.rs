#![allow(clippy::result_large_err)]
use actix_web::http::StatusCode;

#[allow(unused)]
#[derive(thiserror::Error, Debug, strum::AsRefStr)]
pub enum InnerAppError {
    #[error("{self:?}")]
    NoSuchItem(String),

    #[error("{self:?}")]
    MissingFilter(String),

    #[error("{self:?}")]
    InvalidFilter(String),

    #[error("{self:?}")]
    InvalidParameter(String),

    #[error(transparent)]
    ActixWeb(#[from] actix_web::Error),

    #[error(transparent)]
    StdIoError(#[from] std::io::Error),

    #[error("{self:?}")]
    NoSuchSlug(String),

    #[error("Property(id: {0:?}) is required bu no value is offered")]
    MissingItemValue(i32),

    #[error("{self:?}")]
    EmptyItemValue(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),

    #[error("{self:?}")]
    SqlxError(#[from] sqlx::Error),

    #[error("{self:?}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("{self:?}")]
    InfalliableErorr,

    #[error("{self:?}")]
    TimeOut(#[from] tokio::time::error::Elapsed),

    #[error("{self:?}")]
    ToStrError(#[from] actix_web::http::header::ToStrError),

    #[error("{self:?}")]
    Utf8Error(#[from] std::str::Utf8Error),
}

#[derive(Debug)]
pub struct AppError {
    pub message: InnerAppError,
    pub location: &'static std::panic::Location<'static>,
}

impl Default for AppError {
    #[track_caller]
    fn default() -> Self {
        let caller = std::panic::Location::caller();
        Self::new_with_location(caller)
    }
}

#[allow(unused)]
impl AppError {
    #[track_caller]
    pub fn new() -> Self {
        AppError {
            message: InnerAppError::Other(anyhow::anyhow!("Allocate an app error")),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn missing_filter(err: impl Into<String>) -> Self {
        AppError {
            message: InnerAppError::MissingFilter(err.into()),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn invalid_filter(err: impl Into<String>) -> Self {
        AppError {
            message: InnerAppError::InvalidFilter(err.into()),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn no_such_item(err: impl Into<String>) -> Self {
        AppError {
            message: InnerAppError::NoSuchItem(err.into()),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn new_with_location(location: &'static std::panic::Location<'static>) -> Self {
        AppError {
            message: InnerAppError::Other(anyhow::anyhow!("Allocate an app error")),
            location,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        tracing::error!("{self}");
        use actix_web::ResponseError;
        let mut builder = actix_web::HttpResponseBuilder::new(self.status_code());
        let AppError { message, .. } = self;
        builder
            .status(actix_web::http::StatusCode::BAD_REQUEST)
            .json(format!("{:?}", message))
    }
}

impl<E: Into<InnerAppError>> From<E> for AppError {
    #[track_caller]
    fn from(err: E) -> Self {
        let message: InnerAppError = err.into();
        AppError {
            message,
            location: std::panic::Location::caller(),
        }
    }
}

impl std::convert::From<std::convert::Infallible> for InnerAppError {
    fn from(_infall: std::convert::Infallible) -> Self {
        InnerAppError::InfalliableErorr
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.message, self.location)
    }
}

impl actix_web::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        self.error_response()
    }
}
