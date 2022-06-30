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

use warp::{
use serde::{Deserialize, Serialize};
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

// local store -- to later be replaced by a DB
#[derive(Clone)]
struct Store {
    // Using a hashmap here so that we can index an item given its ID w/o traversing the whole
    // collection
    posts: HashMap<PostID, Post>,
}

impl Store {
    fn new() -> Self {
        Store {
            posts: HashMap::new(),
        }
    }
    fn init() -> HashMap<PostID, Post> {
        // REMEMBER TO USE SOMETHING OTHER THAN FROM_STR FOR ARTIST_DATA STRUCTS
        // JSON Files will be way too large
        let file = include_str!("../posts.json");
        serde_json::from_str(file).expect("Can't read posts.json file")
    }
}


#[derive(Debug, Serialize)]
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
#[derive(Debug, Clone, Serialize, Hash, Eq, PartialEq)]
struct PostID(String); 

#[derive(Debug, Clone, Serialize)]
struct Username(String);


async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    // r.find() allows us to search for specific rejections
    if let Some(error) = r.find::<CorsForbidden>() {
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
async fn get_post() -> Result<impl Reply, Rejection> {
    // .values() & .cloned()  methods from hashmap??? Check docs
    let res: Vec<Post> = store.questions.values().cloned().collect()
        Ok(warp::reply::json(&res))
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
        .and(store_filter)
        .and_then(get_post)
        // error handling filter, fetches every prev rejection and check
        // which HTTP message we need to send back
        .recover(return_error);

    let routes = get_items.with(cors);

    // start the server and pass the route filter to it
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
