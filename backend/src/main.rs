#![warn(clippy::all)]

use warp::{http::Method, Filter};

use crate::routes::artist_routes::get_artists;
use sqlx;
use tracing_subscriber::fmt::format::FmtSpan;

mod error;
mod fetch_data;
mod routes;
mod store;
mod types;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    // TODO: Revisit tracing, need to make sure logs are outputted to debug.log file
    // Console logs work, need to work on logging to file
    // Also, we should probably avoid logging whole artist json file, makes logs much less readable

    // https://docs.rs/tracing-subscriber/0.3.1/tracing_subscriber/layer/idnex.html
    // https://stackoverflow.com/questions/70013172/how-to-use-the-tracing-library

    // console logs
    // let stdout_log = tracing_subscriber::fmt::layer().pretty();

    /*
    // logs events to a file
    let log_file = match File::open("debug.log") {
        Ok(log_file) => log_file,
        Err(err) => panic!("Error: {:?}", err),
    };
    let debug_log = tracing_subscriber::fmt::layer().with_writer(Arc::new(log_file));

    // A layer that collects metrics using specific events
    let metrics_layer = /* ... */ filter::LevelFilter::INFO;

    tracing_subscriber::registry()
        .with(
            stdout_log
                // Add an 'INFO' filter to the stdout logging layer
                .with_filter(filter::LevelFilter::INFO)
                // Combine the filtered 'stdout_log' layer with the 'debug_log' layer, producing a new 'Layered' layer
                .and_then(debug_log)
                // Add a filter to *both* layers that rejects spans and events whose targets start with 'metrics'
                .with_filter(filter::filter_fn(|metadata| {
                    !metadata.target().starts_with("metrics")
                })),
        )
        .with(
            // Add a filter to the metrics label that *only* enables events whose targets start with 'metrics'
            metrics_layer.with_filter(filter::filter_fn(|metadata| {
                metadata.target().starts_with("metrics")
            })),
        )
        .init();
    */

    // global log collector configured by RUST_LOG environmental variable
    let log_filter = std::env::var("RUST_LOG")
        // default log level
        .unwrap_or_else(|_| "atl_sound_exchange=info,warp=error".to_owned());

    // If we need to add a username and password, the link takes the structure:
    // "postgres://username:password@localhost:port/db_name"
    let store = store::Store::new("postgres://localhost:5432/atlse").await;
    sqlx::migrate!("./src/migrations")
        .run(&store.clone().connection)
        .await
        .expect("Cannot run migration");
    let store_filter = warp::any().map(move || store.clone());

    // Subscriber: receives all internal log and tracing events and decides what to do with them
    tracing_subscriber::fmt()
        // Use the log filter we built above to determine which traces to record
        .with_env_filter(log_filter)
        // Record an event when each span closes. This can be used to time our routes' durations
        .with_span_events(FmtSpan::CLOSE)
        .init();

    // Cross-Origin Resource Sharing (https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS)
    // "an HTTP-header based mechanism that allows a server to indicate any origins other than its
    // own from which a browser should permit loading resources"
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    // What is a filter?
    // Each HTTP requeset runs through the filters we setup and adds or modifies the data along the
    // way
    //
    // Filters allow us to pass around state and return copies of the object we pass around to more than one route handler

    let registration = warp::post()
        .and(warp::path("registration"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::register);

    // to do: allow requesting a single artist via an id
    // parse url as in update_artist but instead of updating the artist in the hashmap, return it
    let get_artists = warp::get()
        .and(warp::path("artists"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(get_artists)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "get_artists request",
                // % sigil indicates the value should be recorded using its fmt::Display implementation
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    // It seems accepting all POST requests for artists would be a vulnerability
    // We will want to add them internally, instead of allowing users to make POST requests
    // and add random artists via cURL. Here's the code anyways, but it'll remain commented
    // out for now, until I figure out how to only accept with authentication
    /*
    let add_artist = warp::post()
        .and(warp::path("artists"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(add_artist);
    */

    // PUT requests get same status as POST requests ^^^ for now
    /* let update_artist = warp::put()
        .and(warp::path("artists"))
        // add parameters here so filter gets triggered for a particular artist (e.g. artist/000001)
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        // add store to this route so we can pass it to route handler later
        .and(store_filter.clone())
        // extract JSON body
        .and(warp::body::json())
        .and_then(update_artist);
    */

    // DELETE requests get the same status as POST & PUT requests ^^^ for now
    /* let delete_artist = warp::delete()
        .and(warp::path("artists"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_artist);
    */

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::login);

    // defining http routes to try
    // recover = error handling filter, fetches every prev rejection and check
    // which HTTP message we need to send back
    let routes = get_artists
        // .or(update_artist)
        // .or(add_artist)
        // .or(delete_artist)
        .or(registration)
        .or(login)
        .with(cors)
        // log incoming requests as well
        .with(warp::trace::request())
        .recover(error::return_error);

    // start the server and pass the route filter to it
    Ok(warp::serve(routes).run(([127, 0, 0, 1], 3030)).await)
}
