// Artist Struct
//      Background (Struct)
//          Name
//          Place of Origin (Location(String))
//          Genre(s)
//          description (String)
//          Top songs [Song; 10]
//      deezer (JSON)
//      instagram (JSON)
//      soundcloud (JSON)
//      spotify (JSON)
//      tiktok (JSON)
//      twitter (JSON)
//      yt_artist (JSON)
//      yt_channel (JSON)
// 
// Song Struct
//      Title
//      Link
// Genre Enum
// Origin Enum
//
// CSV vs JSON? 
//      CSV is more bandwidth friendly as you don't have
// to parse out the syntax, just separate by character
//
//      JSON has more ergonomic Rust support with serde
//
// Cannot nest structs because size must be known at compile time
// Instead look into RC/ARC/Box<T>
//      Rc and Arc treat their contents as immutable, if you need mutation,
//      you need to combine them with something (Rc: RefCell/ Arc: RwLock)
//
// RwLock: Allows many readers simultaneously but just one writer at a time
//      - Need async implementation of RwLock
//          + parking_lot library
//



use std::sync::Arc;
use parking_lot::RwLock;
use serde::{JSON,URL}

// Circular references here using Arc and RwLock
#[derive(Clone)]
struct Artist {
    // top_songs todo!() 
    genre: Genre,
    socials: Vec<URL>,
    background: Option<Arc<RwLock<Background>>>,
    deezer_data: Option<Arc<RwLock<JSON>>>,
    instagram_data: Option<Arc<RwLock<JSON>>>,
    soundcloud_data: Option<Arc<RwLock<JSON>>>,
    spotify_data: Option<Arc<RwLock<JSON>>>,
    tiktok_data: Option<Arc<RwLock<JSON>>>,
    twitter_data: Option<Arc<RwLock<JSON>>>,
    yt_artist_data: Option<Arc<RwLock<JSON>>>,
    yt_channel_data: Option<Arc<RwLock<JSON>>>,
};

struct Background {
    origin: Option<Arc<RwLock<Origin>>>,
    description: String,
    backstory: String,

};

struct Song {};

enum Genre {};

struct Origin{
    city: String,
    country: String,
};

struct URL{String}



