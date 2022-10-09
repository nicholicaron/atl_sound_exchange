use parking_lot::RwLock;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use crate::types::artist::genre;
use crate::types::artist::{Artist, ArtistID, Background, Origin};

// local store -- to later be replaced by a DB
#[derive(Clone, Debug)]
pub struct Store {
    // Using a hashmap here so that we can index an item given its ID w/o traversing the whole
    // collection
    pub artists: Arc<RwLock<HashMap<ArtistID, Artist>>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            artists: Arc::new(RwLock::new(Self::init())),
        }
    }
    fn init() -> HashMap<ArtistID, Artist> {
        let kanye_id = 1;

        let kanye_name = "Kanye West".to_string();

        let mut socials: Vec<String> = Vec::new();
        {
            let social_entries = include_str!("../artist_data/test_kanye/socials.txt").split('\n');
            for line in social_entries {
                socials.push(line.to_string());
            }
        }

        let mut origin_lines = include_str!("../artist_data/test_kanye/origin.txt").lines();
        let background = Background {
            description: "../../artist_data/test_kanye/background.txt".to_string(),
            origin: Arc::new(RwLock::new(Origin {
                city: origin_lines
                    .next()
                    .expect("error parsing artist origin [city]")
                    .to_string(),
                state: origin_lines
                    .next()
                    .expect("error parsing artist origin [state]")
                    .to_string(),
                country: origin_lines
                    .next()
                    .expect("error parsing artist origin [country]")
                    .to_string(),
            })),
        };

        // Is there a better way to do this? From_reader()????
        //
        // We should extract this functionality to a function, but it may be better to wait until we
        // replace our source with DB
        let deezer_file = BufReader::new(
            File::open("../artist_data/test_kanye/deezer.json")
                .expect("error opening deezer json file"),
        );
        let instagram_file = BufReader::new(
            File::open("../artist_data/test_kanye/instagram.json")
                .expect("error opening instagram json file"),
        );
        let soundcloud_file = BufReader::new(
            File::open("../artist_data/test_kanye/soundcloud.json")
                .expect("error opening soundcloudn json file"),
        );
        let spotify_file = BufReader::new(
            File::open("../artist_data/test_kanye/spotify.json")
                .expect("error opening spotify json file"),
        );
        let tiktok_file = BufReader::new(
            File::open("../artist_data/test_kanye/tiktok.json")
                .expect("error opening tiktok json file"),
        );
        let twitter_file = BufReader::new(
            File::open("../artist_data/test_kanye/twitter.json")
                .expect("error opening twitter json file"),
        );
        let yt_artist_file = BufReader::new(
            File::open("../artist_data/test_kanye/youtube_artist.json")
                .expect("error opening youtube_artist json file"),
        );
        let yt_channel_file = BufReader::new(
            File::open("../artist_data/test_kanye/youtube_channel.json")
                .expect("error opening youtube_channel json file"),
        );

        // serde_json::from_reader(file).expect("Can't read artist.json file")
        // serde_json::from_reader(file) vs json! macro???
        let id = ArtistID(kanye_id);
        let kanye = Artist {
            id: id.clone(),
            name: kanye_name,

            genre: genre::Genre::HipHop,
            socials,
            background: Arc::new(RwLock::new(background)),
            deezer_data: Arc::new(RwLock::new(
                serde_json::from_reader(deezer_file).expect("Unable to read JSON file."),
            )),
            instagram_data: Arc::new(RwLock::new(
                serde_json::from_reader(instagram_file).expect("Unable to read JSON file."),
            )),
            soundcloud_data: Arc::new(RwLock::new(
                serde_json::from_reader(soundcloud_file).expect("Unable to read JSON file."),
            )),
            spotify_data: Arc::new(RwLock::new(
                serde_json::from_reader(spotify_file).expect("Unable to read JSON file."),
            )),
            tiktok_data: Arc::new(RwLock::new(
                serde_json::from_reader(tiktok_file).expect("Unable to read JSON file."),
            )),
            twitter_data: Arc::new(RwLock::new(
                serde_json::from_reader(twitter_file).expect("Unable to read JSON file."),
            )),
            yt_channel_data: Arc::new(RwLock::new(
                serde_json::from_reader(yt_channel_file).expect("Unable to read JSON file."),
            )),
            yt_artist_data: Arc::new(RwLock::new(
                serde_json::from_reader(yt_artist_file).expect("Unable to read JSON file."),
            )),
        };

        let mut artist_profile: HashMap<ArtistID, Artist> = HashMap::new();
        artist_profile.insert(id, kanye);
        artist_profile
    }
}
