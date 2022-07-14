// GO vs RUST for web api's
//
// Pro Rust: 
//      - Data processing capabilities
//      - Error handling
//      - generics
//      - Closure serialization (makes 'moving compute to data' [hadoop/spark] easier)
//      - Clean interface to C (not sure if this will be relevant to this project)
//      - 
//
// Pro Go (GoPro? lol):
//      - Far more ergonomic for this sorta thing
//      - Better libraries

use serde::{Deserialize, Serialize};
use warp::{
    Filter,
    filters::{
        cors::CorsForbidden
    },
    reject::Reject,
    http::Method,
    Rejection,
    Reply,
    http::StatusCode
};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;


// local store -- to later be replaced by a DB
#[derive(Clone)]
struct Store {
    // Using a hashmap here so that we can index an item given its ID w/o traversing the whole
    // collection
    posts: Arc<RwLock<HashMap<PostID, Post>>>,
}

impl Store {
    fn new() -> Self {
        Store {
            posts: Arc::New(RwLock::new(Self::init())),
            responses: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    fn init() -> HashMap<PostID, Post> {
        // REMEMBER TO USE SOMETHING OTHER THAN FROM_STR FOR ARTIST_DATA STRUCTS
        // JSON Files will be way too large
        let file = include_str!("../posts.json");
        serde_json::from_str(file).expect("Can't read posts.json file")
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
struct Post {
    id: PostID,
    author: Username,
    title: String, 
    content: String,
    tags: Option<Vec<String>>,
}

// impl Post {
    // Instead of instantiating a new question instance, we could edit the existing instance,
    // but then we wade into the "lifetime" waters
    // fn update_title(&self, new_title: String) -> Post {
    //    Post::new(self.id.clone(), self.author.clone(), new_title, self.content.clone(), self.tags.clone())
    //}
//}

// Newtype Idiom differentiates QuestionID types from normal strings
// https://doc.rust-lang.org/rust-by-example/generics/new_types.html
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
struct PostID(String); 

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Username(String);

#[derive(Debug)]
enum Error {
    // when rust can't parse an into out of a string we get a ParseIntError
    ParseError(std::num::ParseIntError),
    MissingParameters,
}

// Let's get some custom error messages going to disambiguate a bit
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self{
            // ref???
            Error::ParseError(ref err) => write!(f, "Cannot parse parameter: {}", err),
            Error::MissingParameters => write!(f, "Missing parameter")
        }
    }
}

// marker trait so that's why the body's empty
impl Reject for Error{}

// Pagination struct to add structure to our receiving query params
#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}
// -----------------------------------------------------------------------------------------------------------------
// NEED TO REFURBISH ERROR HANDLING
// What if params > Store.size 
// What if end < start
// etc...
// ----------------------------------------------------------------------------------------------------------------
fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    // check if both parameters are present
    if params.contains_key("start") && params.contains_key("end") {
        // if both params are present, wrap them in Ok(Pagination) and return early
        return Ok(Pagination {
            start: params
                // get returns an option
                .get("start")
                // since we already verified both params are present we can unwrap with a clear
                // conscience
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?
        });
    }
    // If either param is missing, return our custom error type
    Err(Error::MissingParameters)
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    // r.find() allows us to search for specific rejections
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
                error.to_string(),
                StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
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

// First route handler, returns either a reply or rejection
async fn get_posts(params: HashMap<String, String>, store: Store) -> Result<impl Reply, Rejection> {
    if !params.is_empty(){
        let pagination = extract_pagination(params)?;
        let result: Vec<Post> = store.posts.values().cloned().collect();
        let result = &result[pagination.start..pagination.end];
        Ok(warp::reply::json(&result))
    } else {
        let result: Vec<Post> = store.posts.values().cloned().collect();
        Ok(warp::reply::json(&result))
    }
}


#[tokio::main]
async fn main() {
    let store = Store::new();
    // The any filter matches any request, so this statement evaluates for any and all requests
    // Map (w/ move closure) passes the store by value (following cloning) into the filter  so that
    // each route handler has access to the store 
    let store_filter = warp::any().map(move || store.clone());

    // Cross-Origin Resource Sharing (https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS)
    // "an HTTP-header based mechanism that allows a server to indicate any origins other than its
    // own from which a browser should permit loading resources"
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("not-in-the-request")
        .allow_method(&Method::PUT)
        .allow_method(&Method::DELETE)
        .allow_method(&Method::GET)
        .allow_method(&Method::POST);

    // What is a filter?
    // Each HTTP requeset runs through the filters we setup and adds or modifies the data along the
    // way
    //
    // create a path Filter, chaining several filters
    let get_items = warp::get()
        .and(warp::path("posts"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter)
        .and_then(get_posts)
        // error handling filter, fetches every prev rejection and check
        // which HTTP message we need to send back
        .recover(return_error);

    let routes = get_items.with(cors);

    // start the server and pass the route filter to it
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
