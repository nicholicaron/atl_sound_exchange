use parking_lot::RwLock;
use serde::{
    // ser::{SerializeStruct, Serializer},
    Deserialize,
    Serialize,
};
use serde_json::value::Value as json;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

pub mod genre;

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
    name: String,
    genre: genre::Genre,
    socials: Vec<String>,
    background: Arc<RwLock<Background>>,
    // At first I thought we could wrap json values with Results, but calls to the chartmetric API still creates files
    // even if no data is present. We may have to match on file contents to check
    deezer_data: Arc<RwLock<json>>,
    instagram_data: Arc<RwLock<json>>,
    soundcloud_data: Arc<RwLock<json>>,
    spotify_data: Arc<RwLock<json>>,
    tiktok_data: Arc<RwLock<json>>,
    twitter_data: Arc<RwLock<json>>,
    yt_channel_data: Arc<RwLock<json>>,
    yt_artist_data: Arc<RwLock<json>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Background {
    origin: Arc<RwLock<Origin>>,
    description: String,
    // top_songs todo!()
}

// struct Song {};

// should this be a string tuple or a struct?
// I wonder if there will be an issue with nested Arc<RwLock<>>'s
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Origin {
    city: String,
    state: String,
    country: String,
}
