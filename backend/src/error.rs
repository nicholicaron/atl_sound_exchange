use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::StatusCode,
    reject::Reject,
    Rejection, Reply,
};

#[derive(Debug)]
pub enum Error {
    // when rust can't parse an int out of a string we get a ParseIntError
    ParseError(std::num::ParseIntError),
    MissingParameters,
    ArtistNotFound,
}

// Let's get some custom error messages going to disambiguate a bit
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            // ref???
            Error::ParseError(ref err) => write!(f, "Cannot parse parameter: {}", err),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::ArtistNotFound => write!(f, "Artist not found"),
        }
    }
}

// marker trait so that's why the body's empty
impl Reject for Error {}

// TODO:
// Cache error and return more user friendly error message
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    // r.find() allows us to search for specific rejections
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
