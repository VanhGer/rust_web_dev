use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::StatusCode,
    reject::Reject,
    Rejection, Reply,
};

use tracing::{event, Level, instrument};
use argon2::Error as ArgonError;
use reqwest::Error as ReqwestError;
use reqwest_middleware::Error as MiddlewareReqwestError;
#[derive(Debug, Clone)]
pub struct APILayerError { 
    pub status: u16,
    pub message: String,
}

impl std::fmt::Display for APILayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}

#[derive(Debug)]
pub enum CustomError {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    WrongPassword,
    CannotDecryptToken,
    Unauthorized,
    ArgonLibraryError(ArgonError),
    DatabaseQueryError(sqlx::Error),
    MigrationError(sqlx::migrate::MigrateError),
    ReqwestAPIError(ReqwestError),
    MiddlewareReqwestAPIError(MiddlewareReqwestError),
    ClientError(APILayerError),
    ServerError(APILayerError)
}
impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CustomError::ParseError(err) => {
                write!(f, "Cannot parse paramenter: {}", err)
            }
            CustomError::MissingParameters => write!(f, "Missing paramenter"),
            CustomError::WrongPassword => write!(f, "Wrong password"),
            CustomError::CannotDecryptToken => write!(f, "Cannot decrypt error"),
            CustomError::Unauthorized => write!(
                f,
                "No permission to change the underlying resource"
            ),
            CustomError::ArgonLibraryError(_) => {
                write!(f, "Cannot verifiy password")
            },
            CustomError::DatabaseQueryError(_) => {
                write!(f, "Cannot update, invalid data.")
            },
            CustomError::MigrationError(_) => write!(f, "Cannot migrate data"),
            CustomError::ReqwestAPIError(err) => {
                write!(f, "External API error: {}", err)
            },
            CustomError::MiddlewareReqwestAPIError(err) => {
                write!(f, "External API error: {}", err)
            },
            CustomError::ClientError(err) => {
                write!(f, "External Client error: {}", err)
            },
            CustomError::ServerError(err) => {
                write!(f, "External Server error: {}", err)
            },
        }
    }
}
impl Reject for CustomError {}
impl Reject for APILayerError {}
const DUPLICATE_KEY: u32 = 23505;

#[instrument]
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(crate::CustomError::DatabaseQueryError(e)) = r.find() {
        event!(Level::ERROR, "Database query error");

        match e {
            sqlx::Error::Database(err) => {
                if err.code().unwrap().parse::<u32>().unwrap()
                    == DUPLICATE_KEY
                {
                    Ok(warp::reply::with_status(
                        "Account already exsists".to_string(),
                        StatusCode::UNPROCESSABLE_ENTITY,
                    ))
                } else {
                    Ok(warp::reply::with_status(
                        "Cannot update data".to_string(),
                        StatusCode::UNPROCESSABLE_ENTITY,
                    ))
                }
            }
            _ => Ok(warp::reply::with_status(
                "Cannot update data".to_string(),
                StatusCode::UNPROCESSABLE_ENTITY,
            )),
        }
    } else if let Some(crate::CustomError::ReqwestAPIError(e)) = r.find() { 
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }  else if let Some(crate::CustomError::Unauthorized) = r.find() {
        event!(Level::ERROR, "Not matching account id");
        Ok(warp::reply::with_status(
            "No permission to change underlying resource".to_string(),
            StatusCode::UNAUTHORIZED,
        ))
    } else if let Some(crate::CustomError::WrongPassword) = r.find() {
        event!(Level::ERROR, "Entered wrong password");
        Ok(warp::reply::with_status(
            "Wrong E-Mail/Password combination".to_string(),
            StatusCode::UNAUTHORIZED,
        )) 
    }
    else if let Some(crate::CustomError::MiddlewareReqwestAPIError(e)) = r.find() {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(crate::CustomError::ClientError(e)) = r.find() {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(crate::CustomError::ServerError(e)) = r.find() {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        event!(Level::ERROR, "CORS forbidden error: {}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        event!(Level::ERROR, "Cannot deserizalize request body: {}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(error) = r.find::<CustomError>() {
        event!(Level::ERROR, "{}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        event!(Level::WARN, "Requested route was not found");
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
