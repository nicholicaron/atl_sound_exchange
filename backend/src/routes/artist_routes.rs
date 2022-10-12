use std::collections::HashMap;
use tracing::{info, instrument};
use warp::{http::StatusCode, Rejection, Reply};

use crate::error::Error;
use crate::store::Store;
use crate::types::{artist, pagination};

// First route handler, returns either a reply or rejection
// tracing macro to open/close span (and assign tracing events to this span) for us since this is an async fn
#[instrument]
pub async fn get_artists(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl Reply, Rejection> {
    info!("querying artists");
    if !params.is_empty() {
        let pagination = pagination::extract_pagination(params)?;
        info!(pagination = true);
        let result: Vec<artist::Artist> = store.artists.read().values().cloned().collect();
        let result = &result[pagination.start..pagination.end];
        // Was warp::reply::json, need to implement serialize and deserialize for Artist Struct
        Ok(warp::reply::json(&result))
    } else {
        info!(pagination = false);
        let result: Vec<artist::Artist> = store.artists.read().values().cloned().collect();
        // Was warp::reply::json, need to implement serialize and deserialize for Artist Struct
        Ok(warp::reply::json(&result))
    }
}

// Fn for processing HTTP POST requests to the /artists path
// SHOULD ONLY BE ACCESSIBLE INTERNALLY / WITH AUTHENTICATION
#[instrument]
pub async fn add_artist(
    store: Store,
    artist: artist::Artist,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("inserting artist: {:?}", artist.id.clone());
    store.artists.write().insert(artist.id.clone(), artist);

    Ok(warp::reply::with_status("Artist added", StatusCode::OK))
}

// Fn for processing HTTP PUT requests to the /artists path
// SHOULD ONLY BE ACCESSIBLE INTERNALLY / WITH AUTHENTICATION
#[instrument]
pub async fn update_artist(
    id: String,
    store: Store,
    artist: artist::Artist,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store
        .artists
        .write()
        .get_mut(&artist::ArtistID(id.parse().unwrap()))
    {
        Some(a) => {
            info!("updating artist: {:?}", artist.id.clone());
            *a = artist;
        }
        None => {
            info!("failed to find and update artist: {:?}", artist.id.clone());
            return Err(warp::reject::custom(Error::ArtistNotFound));
        }
    }

    Ok(warp::reply::with_status("Artist updated", StatusCode::OK))
}

// Fn for processing HTTP DELETE requests to the /artist path
// SHOULD ONLY BE ACCESSIBLE INTERNALLY / WITH AUTHENTICATION

//  and_then filter expects id to be string, so we pass it as a String, then parse to u16 while matching on accessing the hashmap via its keys (id: ArtistID(u16))
#[instrument]
pub async fn delete_artist(id: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store
        .artists
        .write()
        .remove(&artist::ArtistID(id.parse().unwrap()))
    {
        Some(_) => {
            info!("deleting artist id: {}", id.clone());
            return Ok(warp::reply::with_status("Artist deleted", StatusCode::OK));
        }
        None => Err(warp::reject::custom(Error::ArtistNotFound)),
    }
}
