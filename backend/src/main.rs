use std::io::{Error,ErrorKind};
use std::str::FromStr;
use serde::Serialize;
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

// local store -- to later be replaced by a DB
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
    fn init() {}
    fn add_post(mut self, post: Post) -> Self {
        self.posts.insert(post.id.clone(), post);
        self
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

impl Post {
    fn new(id: PostID, author: Username, title: String, content: String, tags: Option<Vec<String>>) -> Post {
        Post {
            id,
            author,
            title,
            content,
            tags,
        }
    }
    // Instead of instantiating a new question instance, we could edit the existing instance,
    // but then we wade into the "lifetime" waters
    fn update_title(&self, new_title: String) -> Post {
        Post::new(self.id.clone(), self.author.clone(), new_title, self.content.clone(), self.tags.clone())
    }
}

// Newtype Idiom differentiates QuestionID types from normal strings
// https://doc.rust-lang.org/rust-by-example/generics/new_types.html
#[derive(Debug, Clone, Serialize, Hash, Eq, PartialEq)]
struct PostID(String); 

#[derive(Debug, Clone, Serialize)]
struct Username(String);

// Implement FromStr to allow easier creation of QuestionID (via a string slice)
impl FromStr for PostID {
    type Err = std::io::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(PostID(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}
// Custom warp reject type
#[derive(Debug)]
struct InvalidID;
impl Reject for InvalidID {}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    // r.find() allows us to search for specific rejections
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(_invalid_id) = r.find::<InvalidID>() {
        Ok(warp::reply::with_status(
            "No valid ID presented".to_string(),
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
    let post = Post::new(
        PostID::from_str("1").expect("No id provided"),
        Username("Anthony Fantano".to_string()),
        "First Question".to_string(),
        "Content of Question".to_string(),
        Some(vec!("faq".to_string())),
   );

    match post.id.0.parse::<i32>() {
        Err(_) => {
            Err(warp::reject::custom(InvalidID))
        },
        Ok(_) => 
            Ok(warp::reply::json(
                &post
            ))
    }
}

#[tokio::main]
async fn main() {
    // Cross-Origin Resource Sharing (https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS)
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("not-in-the-request")
        .allow_method(&Method::PUT)
        .allow_method(&Method::DELETE)
        .allow_method(&Method::GET)
        .allow_method(&Method::POST);

    // create a path Filter, chaining several filters
    let get_items = warp::get()
        .and(warp::path("posts"))
        .and(warp::path::end())
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
