use warp::{http::Method, Filter};

use crate::routes::artist_routes::{add_artist, delete_artist, get_artists, update_artist};
use crate::store::Store;

mod error;
mod routes;
mod store;
mod types;

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
    // "an HTTP-header based mechanism that allows a server to indicate any origins other than its
    // own from which a browser should permit loading resources"
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
        .recover(error::return_error);

    // start the server and pass the route filter to it
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
