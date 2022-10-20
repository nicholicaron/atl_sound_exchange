use crate::error::Error;
use std::collections::HashMap;

/// Pagination struct to add structure to our receiving query params
#[derive(Default, Debug)]
pub struct Pagination {
    /// Index of last item to be returned
    // optional -- postgreSQL will ignore in None case, saving us the complexity of handling ourselves
    pub limit: Option<i32>,
    /// Index of first item to be returned
    pub offset: i32,
}

// TODO:
// NEED TO REFURBISH ERROR HANDLING
// What if params > Store.size
// What if end < start
// etc...
/// Extract query parameters from the `/artists` route
/// # Example query
/// GET requests to this route can have a pagination attached so we just return the artists we need
/// `/artists?limit=10&offset=1`
/// # Example usage
/// ```rust
/// let mut query = HashMap::new();
/// query.insert("limit".to_string(), "10".to_string());
/// query.insert("offset".to_string(). "1".to_string());
/// let p = types::pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.limit, 10);
/// assert_eq!(p.offset, 1);
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    // check if both parameters are present
    // Could be improved in the future
    if params.contains_key("limit") && params.contains_key("offset") {
        // if both params are present, wrap them in Ok(Pagination) and return early
        return Ok(Pagination {
            // Takes the "start" parameter in the query and tries to convert it to a number
            limit: Some(
                params
                    // get returns an option
                    .get("limit")
                    // since we already verified both params are present we can unwrap with a clear conscience
                    .unwrap()
                    .parse::<i32>()
                    .map_err(Error::ParseError)?,
            ),
            // Takes the "end" parameter in the query and tries to convert it to a number
            offset: params
                .get("end")
                .unwrap()
                .parse::<i32>()
                .map_err(Error::ParseError)?,
        });
    }
    // If either param is missing, return our custom error type
    Err(Error::MissingParameters)
}
