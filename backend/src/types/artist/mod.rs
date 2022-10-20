// pub mod genre;

// TODO How do we insert artist profile pics TODO

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use serde_json::value::Value as json;
use std::sync::Arc;

// Newtype Idiom differentiates ArtistID types from normal u16's
// https://doc.rust-lang.org/rust-by-example/generics/new_types.html
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
pub struct ArtistID(pub i32);

// May bring this newtype for socials vec back eventually, for now it seems unnecessary
// #[derive(Clone, Serialize, Deserialize, Debug)]
// struct Url(String);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artist {
    // id is i32 for postgresql compatibility -- may change after considering db alternatives
    pub id: ArtistID,
    pub name: String,
    pub genre: String,
    // socials.0 = spotify link
    // socials.1 = apple music link
    // socials.2 = instagram link
    // socials.3 = twitter link
    pub socials: [String; 4],
    // background.0 = city
    // background.1 = state
    // background.2 = country
    // background.3 = description
    pub background: [String; 4],
    // At first I thought we could wrap json values with Results, but calls to the chartmetric API still creates files
    // even if no data is present. We may have to match on file contents to check
    pub deezer_data: Arc<RwLock<json>>,
    pub instagram_data: Arc<RwLock<json>>,
    pub soundcloud_data: Arc<RwLock<json>>,
    pub spotify_data: Arc<RwLock<json>>,
    pub tiktok_data: Arc<RwLock<json>>,
    pub twitter_data: Arc<RwLock<json>>,
    pub yt_channel_data: Arc<RwLock<json>>,
    pub yt_artist_data: Arc<RwLock<json>>,
}

/* We should eventually try to more strongly type background, socials, and genre structs. For now I'm simplifying them to be more easily compatible with PostgreSQL mapping (store.rs)
To decode a custom type (T) we have to implement Type<Postgres> for T, which doesn't seem that hard, but I don't feel like fiddling with it right now


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Background {
    pub origin: (),
    pub description: String,
    // TODO: top_songs
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Origin {
    pub city: String,
    pub state: String,
    pub country: String,
}
 */
