use crate::error::Error;
use std::collections::HashMap;

// Pagination struct to add structure to our receiving query params
#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

// TODO:
// NEED TO REFURBISH ERROR HANDLING
// What if params > Store.size
// What if end < start
// etc...
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    // check if both parameters are present
    if params.contains_key("start") && params.contains_key("end") {
        // if both params are present, wrap them in Ok(Pagination) and return early
        return Ok(Pagination {
            start: params
                // get returns an option
                .get("start")
                // since we already verified both params are present we can unwrap with a clear conscience
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
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
