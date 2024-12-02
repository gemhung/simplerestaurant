#![allow(clippy::result_large_err)]
use actix_web::http::StatusCode;

use crate::config::settings;

#[allow(unused)]
#[derive(thiserror::Error, Debug, strum::AsRefStr)]
pub enum InnerAppError {
    #[error("{self:?}")]
    InvalidParameter(String),

    #[error(transparent)]
    ActixWeb(#[from] actix_web::Error),

    #[error(transparent)]
    StdIoError(#[from] std::io::Error),

    #[error("{self:?}")]
    NoSuchSlug(String),

    #[error("{self:?}")]
    NoSuchPrimaryKey(String),

    #[error("Property(id: {0:?}) is required bu no value is offered")]
    MissingItemValue(i32),

    #[error("{self:?}")]
    EmptyItemValue(String),

    #[error("{self:?}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("{self:?}")]
    TextValidationError(String),

    #[error("{self:?}")]
    PhoneValidationError(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),

    #[error("{self:?}")]
    MissingAttribute(String),

    #[error("{self:?}")]
    InvalidAttributes(String),

    #[error("{self:?}")]
    Validation(String),

    #[error("{self:?}")]
    ParseFlotError(#[from] std::num::ParseFloatError),

    #[error("{self:?}")]
    SqlxError(#[from] sqlx::Error),

    #[error("{self:?}")]
    InvalidDomain(String),

    #[error("{self:?}")]
    MissingSubdomain(String),

    #[error("{self:?}")]
    WebSiteNotFound(String),

    //#[error(transparent)]
    //UuidError(#[from] uuid::Error),
    #[error(transparent)]
    SpawnJointError(#[from] actix_web::rt::task::JoinError),

    #[error("{self:?}")]
    NotAllowAnonymous(String),

    #[error("{self:?}")]
    ChangePasswordError(String),

    //#[error(transparent)]
    //DeadpoolRedisPoolError(#[from] deadpool_redis::PoolError),

    //#[error(transparent)]
    //RedisError(#[from] deadpool_redis::redis::RedisError),
    #[error("{self:?}")]
    TokenNotExist,

    //#[error(transparent)]
    //SerdeQs(#[from] serde_qs::Error),
    #[error(transparent)]
    PathStripPrefixError(#[from] std::path::StripPrefixError),

    #[error("{self:?}")]
    UnknownUrlForService(String),

    #[error("{self:?}")]
    MetaDataParserErorr(String),

    #[error("{self:?}")]
    NoSuchView(i64),

    #[error("{self:?}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("{self:?}")]
    CantFindTimestamp(String),

    #[error("{self:?}")]
    InvalidStatusCode(String),

    #[error("{self:?}")]
    BadRequest(String),

    #[error("{self:?}")]
    NoSuchMethod(String),

    #[error("{self:?}")]
    NoSuchRoute(String),

    #[error("{self:?}")]
    InfalliableErorr,

    #[error("{self:?}")]
    TimeOut(#[from] tokio::time::error::Elapsed),

    #[error("{self:?}")]
    ToStrError(#[from] actix_web::http::header::ToStrError),

    #[error("{self:?}")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("{self:?}")]
    AnotherUserAlreadyLogin(String),

    #[error("{self:?}")]
    IntoSeaQueryValue(String),

    //#[error("{self:?}")]
    //DeriveMoreFromStr(#[from] derive_more::FromStrError),
    #[error("{self:?}")]
    Unescape(String),

    #[error("{self:?}")]
    EmptyContentType(String),

    #[error("{self:?}")]
    Downcast(String),
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
    pub fn downcast(err: impl Into<String>) -> Self {
        AppError {
            message: InnerAppError::Downcast(err.into()),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn empty_content_type(err: impl Into<String>) -> Self {
        AppError {
            message: InnerAppError::EmptyContentType(err.into()),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn unescape(err: impl Into<String>) -> Self {
        AppError {
            message: InnerAppError::Unescape(err.into()),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn into_sea_query_value(err: impl Into<String>) -> Self {
        AppError {
            message: InnerAppError::IntoSeaQueryValue(err.into()),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn another_user_already_login(user_name: impl Into<String>) -> Self {
        AppError {
            message: InnerAppError::AnotherUserAlreadyLogin(user_name.into()),
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

    #[track_caller]
    pub fn message<E: Into<InnerAppError>>(msg: E) -> Self {
        AppError {
            message: msg.into(),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn no_such_slug(s: String) -> Self {
        AppError {
            message: InnerAppError::NoSuchSlug(s),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn no_such_primary_key(s: String) -> Self {
        AppError {
            message: InnerAppError::NoSuchPrimaryKey(s),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn text_validation(s: String) -> Self {
        AppError {
            message: InnerAppError::TextValidationError(s),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn missing_item_value(id: i32) -> Self {
        AppError {
            message: InnerAppError::MissingItemValue(id),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn missing_attribute(s: String) -> Self {
        AppError {
            message: InnerAppError::MissingAttribute(s),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn validation(s: String) -> Self {
        AppError {
            message: InnerAppError::Validation(s),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn phone_validation(s: String) -> Self {
        AppError {
            message: InnerAppError::PhoneValidationError(s),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn invalid_attribute(s: String) -> Self {
        AppError {
            message: InnerAppError::InvalidAttributes(s),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn invalid_domain(s: String) -> Self {
        AppError {
            message: InnerAppError::InvalidDomain(s),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn missing_subdomain(s: String) -> Self {
        AppError {
            message: InnerAppError::MissingSubdomain(s),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn web_site_not_found(s: String) -> Self {
        AppError {
            message: InnerAppError::WebSiteNotFound(s),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn not_allowed_anonymous(s: String) -> Self {
        AppError {
            message: InnerAppError::NotAllowAnonymous(s),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn change_password(s: String) -> Self {
        AppError {
            message: InnerAppError::ChangePasswordError(s),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn token_not_exist() -> Self {
        AppError {
            message: InnerAppError::TokenNotExist,
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn unknown_url_for_service(service: String) -> Self {
        AppError {
            message: InnerAppError::UnknownUrlForService(service),
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn metadata_parser(s: String) -> Self {
        AppError {
            message: InnerAppError::MetaDataParserErorr(s),
            location: std::panic::Location::caller(),
        }
    }
    #[track_caller]
    pub fn no_such_view(id: i64) -> Self {
        AppError {
            message: InnerAppError::NoSuchView(id),
            location: std::panic::Location::caller(),
        }
    }
    #[track_caller]
    pub fn cant_find_timestamp(json: String) -> Self {
        AppError {
            message: InnerAppError::CantFindTimestamp(json),
            location: std::panic::Location::caller(),
        }
    }
    #[track_caller]
    pub fn invalid_status_code(msg: String) -> Self {
        AppError {
            message: InnerAppError::InvalidStatusCode(msg),
            location: std::panic::Location::caller(),
        }
    }
    #[track_caller]
    pub fn bad_request(msg: impl Into<String>) -> Self {
        AppError {
            message: InnerAppError::BadRequest(msg.into()),
            location: std::panic::Location::caller(),
        }
    }
    #[track_caller]
    pub fn no_such_method(msg: String) -> Self {
        AppError {
            message: InnerAppError::NoSuchMethod(msg),
            location: std::panic::Location::caller(),
        }
    }
    #[track_caller]
    pub fn no_such_route(msg: String) -> Self {
        AppError {
            message: InnerAppError::NoSuchRoute(msg),
            location: std::panic::Location::caller(),
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        tracing::error!("{self}");
        use actix_web::ResponseError;
        let mut builder = actix_web::HttpResponseBuilder::new(self.status_code());
        match self {
            // InvalidDomain
            AppError {
                message: InnerAppError::InvalidDomain(_msg),
                ..
            } => {
                //let data = serde_json::json!({
                //    "settings": settings().serialize_for_view(""),
                //});
                //let rendered =
                //    crate::template_engine::render("errors/invalid_domain", "ignore", Some(data))
                //        .unwrap_or_default();

                builder
                    .status(actix_web::http::StatusCode::FORBIDDEN)
                    .body(())
            }
            // WebSiteNotFound
            AppError {
                message: InnerAppError::WebSiteNotFound(_msg),
                ..
            } => {
                //let rendered =
                //    crate::template_engine::render("errors/web_site_not_found", "ignore", None)
                //        .unwrap_or_default();
                builder
                    .status(actix_web::http::StatusCode::NOT_FOUND)
                    .body(())
            }
            // NoSuchRoute
            AppError {
                message: InnerAppError::NoSuchRoute(_msg),
                ..
            } => builder
                .status(actix_web::http::StatusCode::BAD_REQUEST)
                .body(_msg.clone()),
            // TODO: Redirect all errors to "/dashboard" temporarily
            AppError { message, .. } => builder
                .status(actix_web::http::StatusCode::BAD_REQUEST)
                .body(format!("{:?}", message)),
        }
    }
}

pub type CMSResult<T> = Result<T, AppError>;

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
    fn from(infall: std::convert::Infallible) -> Self {
        InnerAppError::InfalliableErorr
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.message, self.location)
    }
}

#[derive(Debug)]
struct Super<'a, T>(pub &'a T);

impl<T: std::fmt::Display> std::fmt::Display for Super<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<AppError> for anyhow::Error {
    fn from(err: AppError) -> Self {
        anyhow::Error::msg(err.to_string())
    }
}

// Default implementation for ResponseError
impl<T: std::fmt::Debug + std::fmt::Display> actix_web::ResponseError for Super<'_, T> {}

impl actix_web::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        self.error_response()
    }
}
