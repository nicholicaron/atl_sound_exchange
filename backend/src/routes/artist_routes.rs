use std::collections::HashMap;
use tracing::{instrument, *};
use warp::{Rejection, Reply};

use crate::store::Store;
use crate::types::{artist, pagination};

// First route handler, returns either a reply or rejection
// tracing macro to open/close span (and assign tracing events to this span) for us since this is an async fn
#[instrument]
pub async fn get_artists(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl Reply, Rejection> {
    event!(target: "atl_sound_exchange", Level::INFO, "querying artists");
    let mut pagination = pagination::Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, "pagination = true");
        pagination = pagination::extract_pagination(params)?;
    }

    let result: Vec<artist::Artist> =
        match store.get_artists(pagination.limit, pagination.offset).await {
            Ok(result) => result,
            Err(e) => return Err(warp::reject::custom(e)),
        };

    Ok(warp::reply::json(&result))
}

// TODO: Update other route handlers for DB compatibility if deemed appropriate later

// Fn for processing HTTP POST requests to the /artists path
// SHOULD ONLY BE ACCESSIBLE INTERNALLY / WITH AUTHENTICATION
/* #[instrument]
pub async fn add_artist(
    store: Store,
    artist: artist::Artist,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(
        Level::INFO,
        "inserting artist: {:?}",
        artist.id.clone()
    );
    store.artists.write().insert(artist.id.clone(), artist);

    Ok(warp::reply::with_status("Artist added", StatusCode::OK))
}

// Fn for processing HTTP PUT requests to the /artists path
// SHOULD ONLY BE ACCESSIBLE INTERNALLY / WITH AUTHENTICATION
//
// TODO: is there a way to update with only the diff? & if so, is it worth the effort of
// implementing
#[instrument]
pub async fn update_artist(
    id: i32,
    store: Store,
    artist: artist::Artist,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.artists.write().get_mut(&artist::ArtistID(id)) {
        Some(a) => {
            event!(
                Level::INFO,
                "updating artist: {:?}",
                artist.id.clone()
            );
            *a = artist;
        }
        None => {
            event!(
                Level::INFO,
                "failed to find and update artist: {:?}",
                artist.id.clone()
            );
            return Err(warp::reject::custom(Error::ArtistNotFound));
        }
    }

    Ok(warp::reply::with_status("Artist updated", StatusCode::OK))
}

// Fn for processing HTTP DELETE requests to the /artist path
// SHOULD ONLY BE ACCESSIBLE INTERNALLY / WITH AUTHENTICATION

//  and_then filter expects id to be string, so we pass it as a String, then parse to u16 while matching on accessing the hashmap via its keys (id: ArtistID(i32))
#[instrument]
pub async fn delete_artist(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.artists.write().remove(&artist::ArtistID(id)) {
        Some(_) => {
            event!(Level::INFO, "deleting artist id: {}", id.clone());
            return Ok(warp::reply::with_status("Artist deleted", StatusCode::OK));
        }
        None => Err(warp::reject::custom(Error::ArtistNotFound)),
    }
}
*/
