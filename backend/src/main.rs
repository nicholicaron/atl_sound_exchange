use crate::artist::Store;
use std::collections::HashMap;
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::{Method, StatusCode},
    reject::Reject,
    Filter, Rejection, Reply,
};

mod artist;

//#[derive(Debug, Serialize, Deserialize, Clone)]
// struct Post {
//    id: PostID,
//    author: Username,
//    title: String,
//    content: String,
//    tags: Option<Vec<String>>,
//}

// impl Post {
// Instead of instantiating a new artist instance, we could edit the existing instance,
// but then we wade into the "lifetime" waters
// fn update_title(&self, new_title: String) -> Post {
//    Post::new(self.id.clone(), self.author.clone(), new_title, self.content.clone(), self.tags.clone())
//}
//}

// Newtype Idiom differentiates QuestionID types from normal strings
// https://doc.rust-lang.org/rust-by-example/generics/new_types.html
// #[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
// struct PostID(String);

// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct Username(String);

#[derive(Debug)]
enum Error {
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

// Pagination struct to add structure to our receiving query params
#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

// TODO:
// NEED TO REFURBISH ERROR HANDLING
// What if params > Store.size
// What if end < start
// etc...
fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
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

// TODO:
// Cache error and return more user friendly error message
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

// First route handler, returns either a reply or rejection
async fn get_artists(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl Reply, Rejection> {
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        let result: Vec<artist::Artist> = store.artists.read().values().cloned().collect();
        let result = &result[pagination.start..pagination.end];
        // Was warp::reply::json, need to implement serialize and deserialize for Artist Struct
        Ok(warp::reply::json(&result))
    } else {
        let result: Vec<artist::Artist> = store.artists.read().values().cloned().collect();
        // Was warp::reply::json, need to implement serialize and deserialize for Artist Struct
        Ok(warp::reply::json(&result))
    }
}

// Fn for processing HTTP POST requests to the /artists path
// SHOULD ONLY BE ACCESSIBLE INTERNALLY / WITH AUTHENTICATION
async fn add_artist(
    store: Store,
    artist: artist::Artist,
) -> Result<impl warp::Reply, warp::Rejection> {
    store.artists.write().insert(artist.id.clone(), artist);

    Ok(warp::reply::with_status("Artist added", StatusCode::OK))
}

// Fn for processing HTTP PUT requests to the /artists path
// SHOULD ONLY BE ACCESSIBLE INTERNALLY / WITH AUTHENTICATION

async fn update_artist(
    id: String,
    store: Store,
    artist: artist::Artist,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.artists.write().get_mut(&artist::ArtistID {
        number: id.parse().unwrap(),
    }) {
        Some(a) => *a = artist,
        None => return Err(warp::reject::custom(Error::ArtistNotFound)),
    }

    Ok(warp::reply::with_status("Artist updated", StatusCode::OK))
}

// Fn for processing HTTP DELETE requests to the /artist path
// SHOULD ONLY BE ACCESSIBLE INTERNALLY / WITH AUTHENTICATION

//  and_then filter expects id to be string, so we pass it as a String, then parse to u16 while matching on accessing the hashmap via its keys (id: ArtistID(u16))
async fn delete_artist(id: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.artists.write().remove(&artist::ArtistID {
        number: id.parse().unwrap(),
    }) {
        Some(_) => return Ok(warp::reply::with_status("Artist deleted", StatusCode::OK)),
        None => return Err(warp::reject::custom(Error::ArtistNotFound)),
    }
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    // The any filter matches any request, so this statement evaluates for any and all requests
    // Map (w/ move closure) passes the store by value (following cloning) into the filter  so that
    // each route handler has access to the store

    // TODO:
    // seems like we clone the store A LOT, let's eventually optimize this and try to pass around references where possible
    let store_filter = warp::any().map(move || store.clone());

    // Cross-Origin Resource Sharing (https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS)
    // own from which a browser should permit loading resources"
    // "an HTTP-header based mechanism that allows a server to indicate any origins other than its
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("not-in-the-request")
        .allow_method(&Method::PUT)
        .allow_method(&Method::DELETE)
        .allow_method(&Method::GET)
        // DOUBLE CHECK WHETHER WE NEED POST REQUESTS, users will not be updating artist structs
        .allow_method(&Method::POST);

    // What is a filter?
    // Each HTTP requeset runs through the filters we setup and adds or modifies the data along the
    // way
    //
    // Filters allow us to pass around state and return copies of the object we pass around to more than one route handler

    // to do: allow requesting a single artist via an id
    // parse url as in update_artist but instead of updating the artist in the hashmap, return it
    let get_artists = warp::get()
        .and(warp::path("artists"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(get_artists);

    // It seems accepting all POST requests for artists would be a vulnerability
    // We will want to add them internally, instead of allowing users to make POST requests
    // and add random artists via cURL. Here's the code anyways, but it'll remain commented
    // out for now, until I figure out how to only accept with authentication
    let add_artist = warp::post()
        .and(warp::path("artists"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(add_artist);

    // PUT requests get same status as POST requests ^^^ for now
    let update_artist = warp::put()
        .and(warp::path("artists"))
        // add parameters here so filter gets triggered for a particular artist (e.g. artist/000001)
        // since ArtistID is a struct with both name and number, make sure we're addressing ArtistID.number
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        // add store to this route so we can pass it to route handler later
        .and(store_filter.clone())
        // extract JSON body
        .and(warp::body::json())
        .and_then(update_artist);

    // DELETE requests get the same status as POST & PUT requests ^^^ for now
    let delete_artist = warp::delete()
        .and(warp::path("artists"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_artist);

    // defining http routes to try
    // recover = error handling filter, fetches every prev rejection and check
    // which HTTP message we need to send back
    let routes = get_artists
        .or(update_artist)
        .or(add_artist)
        .or(delete_artist)
        .with(cors)
        .recover(return_error);

    // start the server and pass the route filter to it
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
