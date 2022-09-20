use crate::error::Error;
use std::collections::HashMap;

/// Pagination struct to add structure to our receiving query params
#[derive(Debug)]
pub struct Pagination {
    /// Index of first item to be returned
    pub start: usize,
    /// Index of final item to be returned
    pub end: usize,
}

// TODO:
// NEED TO REFURBISH ERROR HANDLING
// What if params > Store.size
// What if end < start
// etc...
/// Extract query parameters from the `/artists` route
/// # Example query
/// GET requests to this route can have a pagination attached so we just return the artists we need
/// `/artists?start=1&end=10 `
/// # Example usage
/// ```rust
/// let mut query = HashMap::new();
/// query.insert("start".to_string(), "1".to_string());
/// query.insert("end".to_string(). "10".to_string());
/// let p = types::pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.start, 1);
/// assert_eq!(p.end, 10);
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    // check if both parameters are present
    // Could be improved in the future
    if params.contains_key("start") && params.contains_key("end") {
        // if both params are present, wrap them in Ok(Pagination) and return early
        return Ok(Pagination {
            // Takes the "start" parameter in the query and tries to convert it to a number
            start: params
                // get returns an option
                .get("start")
                // since we already verified both params are present we can unwrap with a clear conscience
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            // Takes the "end" parameter in the query and tries to convert it to a number
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }
    // If either param is missing, return our custom error type
    Err(Error::MissingParameters)
}
