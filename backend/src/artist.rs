use std::sync::Arc;
use parking_lot::RwLock;
use serde_json;
use serde_qs;
use backend::artist::genre;

pub mod genre;

// local store -- to later be replaced by a DB
#[derive(Clone)]
struct Store {
    // Using a hashmap here so that we can index an item given its ID w/o traversing the whole
    // collection
    posts: Arc<RwLock<HashMap<PostID, Post>>>,
}

impl Store {
    fn new() -> Self {
        Store {
            artists: Arc::New(RwLock::new(Self::init())),
        }
    }
    fn init() -> HashMap<ArtistID, Artist> {
       let id: HashMap<String, u16> = HashMap::from[("Kanye West", 00001)]; 

        let socials = vec::new(); 
        let social_entries = include_str!("../artist_data/test_kanye/socials.txt").split("\n");
        for line in social_entries {
            socials.push(line);
        }

        let background = 

        // from_reader()
        let background_file = include_str!("../artist_data/test_kanye/background.txt");
        let origin_file = include_str!("../artist_data/test_kanye/origin.txt");
        let deezer_file = include_str!("../artist_data/test_kanye/deezer.json");
        let instagram_file = include_str!("../artist_data/test_kanye/instagram.json");
        let soundcloud_file = include_str!("../artist_data/test_kanye/soundcloud.json");
        let spotify_file = include_str!("../artist_data/test_kanye/spotify.json");
        let tiktok_file = include_str!("../artist_data/test_kanye/tiktok.json");
        let twitter_file = include_str!("../artist_data/test_kanye/twitter.json");
        let yt_artist_file = include_str!("../artist_data/test_kanye/youtube_artist.json");
        let yt_channel_file = include_str!("../artist_data/test_kanye/youtube_channel.json");



        let kanye = Artist{
            genre: genre.HipHopSubgenre.HipHop,
            socials,
            Some<Arc::new(RwLock::new())>
            serde_json::from_reader(file).expect("Can't read artist.json file")
        }
    }
}


// Circular references here using Arc and RwLock
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Artist {
    genre: genre,
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
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Background {
    origin: Option<Arc<RwLock<Origin>>>,
    description: String,
    // top_songs todo!() 
}

// struct Song {};

// should this be a string tuple or a struct?
// I wonder if there will be an issue with nested Arc<RwLock<>>'s
#[derive(Clone, Serialize, Deserialize, Debug)]
struct Origin{
    city: String,
    state: String,
    country: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct URL{String};



