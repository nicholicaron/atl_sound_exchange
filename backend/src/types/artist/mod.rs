pub mod genre;

use parking_lot::RwLock;
use serde::{
    // ser::{SerializeStruct, Serializer},
    Deserialize,
    Serialize,
};
use serde_json::value::Value as json;
use std::sync::Arc;

// Newtype Idiom differentiates ArtistID types from normal u16's
// https://doc.rust-lang.org/rust-by-example/generics/new_types.html
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
pub struct ArtistID(pub u16);

// May bring this newtype for socials vec back eventually, for now it seems unnecessary
// #[derive(Clone, Serialize, Deserialize, Debug)]
// struct Url(String);

// Circular references here using Arc and RwLock
// Had to use the cargo.toml flags "derive" and rc for serde and "serde" for parking_lot
//
// --------------------------------------------------------------------------------------------------------------------------------------------------------
// https://stackoverflow.com/questions/56156876/how-to-deserialize-a-parking-lotmutex-with-serde
// https://stackoverflow.com/questions/49312600/how-do-i-serialize-or-deserialize-an-arct-in-serde
// --------------------------------------------------------------------------------------------------------------------------------------------------------
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artist {
    pub id: ArtistID,
    pub name: String,
    pub genre: genre::Genre,
    pub socials: Vec<String>,
    pub background: Arc<RwLock<Background>>,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Background {
    pub origin: Arc<RwLock<Origin>>,
    pub description: String,
    // top_songs todo!()
}

// struct Song {};

// should this be a string tuple or a struct?
// I wonder if there will be an issue with nested Arc<RwLock<>>'s
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Origin {
    pub city: String,
    pub state: String,
    pub country: String,
}
