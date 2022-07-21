use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
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
    posts: Arc<RwLock<HashMap<ArtistID, Artist>>>,
}

impl Store {
    fn new() -> Self {
        Store {
            artists: Arc::New(RwLock::new(Self::init())),
        }
    }
    fn init() -> HashMap<ArtistID, Artist> {
        let ArtistID: HashMap<String, u16> = HashMap::from[("Kanye West", 00001)]; 

        let socials = vec::new(); 
        {   
            let social_entries = include_str!("../artist_data/test_kanye/socials.txt").split("\n");
            for line in social_entries {
                socials.push(line);
            }
        }

        let background = Background {
            description: include_str!("../artist_data/test_kanye/background.txt");
            origin: Some<RwLock::new(Origin{
                let origin_lines = include_str!("../artist_data/test_kanye/origin.txt").lines();
                city: origin_lines.next();
                state: origin_lines.next();
                country: origin_lines.next();
            })>;
        }

        // Is there a better way to do this? From_reader()????
        // Box allocation???
        // We could extract this functionality to a function, but it may be better to wait until we
        // replace our source with DB
        let deezer_file = BufReader::new(File::open("../artist_data/test_kanye/deezer.json"));
        let instagram_file = Bufreader::new(File::open("../artist_data/test_kanye/instagram.json"));
        let soundcloud_file = Bufreader::new(File::open("../artist_data/test_kanye/soundcloud.json"));
        let spotify_file = Bufreader::new(File::open("../artist_data/test_kanye/spotify.json"));
        let tiktok_file = Bufreader::new(File::open("../artist_data/test_kanye/tiktok.json"));
        let twitter_file = Bufreader::new(File::open("../artist_data/test_kanye/twitter.json"));
        let yt_artist_file = Bufreader::new(File::open("../artist_data/test_kanye/youtube_artist.json"));
        let yt_channel_file = Bufreader::new(File::open("../artist_data/test_kanye/youtube_channel.json"));

        // serde_json::from_reader(file).expect("Can't read artist.json file")
        // serde_json::from_reader(file) vs json! macro???
        let kanye = Artist{
            genre: genre.HipHopSubgenre.HipHop,
            socials,
            background: Some<Arc::new(RwLock::new(background))>,
            deezer_data: serde_json::from_reader(deezer_file).expect("Unable to read JSON file."),
            instagram_data: serde_json::from_reader(instagram_file).expect("Unable to read JSON file."),
            soundcloud_data: serde_json::from_reader(soundcloud_file).expect("Unable to read JSON file."),
            spotify_data: serde_json::from_reader(spotify_file).expect("Unable to read JSON file."),
            tiktok_data: serde_json::from_reader(tiktok_file).expect("Unable to read JSON file."),
            twitter_data: serde_json::from_reader(twitter_file).expect("Unable to read JSON file."),
            yt_artist_data: serde_json::from_reader(yt_artist_file).expect("Unable to read JSON file."),
            yt_channel_data: serde_json::from_reader(yt_channel_data).expect("Unable to read JSON file."),
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




