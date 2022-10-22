use anyhow::Result;
use parking_lot::RwLock;
use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgRow},
    Row,
};
use std::sync::Arc;
use tracing::*;

// use crate::types::artist::genre;
use crate::error::Error;
use crate::types::{
    account::Account,
    artist::{Artist, ArtistID, NewArtist},
};

// Store holds the database connection and is passed to the route handlers
#[derive(Clone, Debug)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(_) => panic!("Couldn't establish DB connection!"),
        };

        Store {
            connection: db_pool,
        }
    }

    pub async fn add_account(self, account: Account) -> Result<bool, Error> {
        match sqlx::query("INSERT INTO accounts (email, password) VALUES ($1, $2)")
            .bind(account.email)
            .bind(account.password)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(error) => {
                tracing::event!(
                    tracing::Level::ERROR,
                    code = error
                        .as_database_error()
                        .unwrap()
                        .code()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                    db_message = error.as_database_error().unwrap().message(),
                    constraint = error.as_database_error().unwrap().constraint().unwrap()
                );
                Err(Error::DatabaseQueryError)
            }
        }
    }

    pub async fn add_artists(self, new_artist: NewArtist) -> Result<Artist, sqlx::Error> {
        // Can we acquire the locks here before passing them to the query?
        let unlocked_deezer = new_artist.deezer_data.read().read();
        let unlocked_instagram = new_artist.instagram_data.read().read();
        let unlocked_soundcloud = new_artist.soundcloud_data.read().read();
        let unlocked_spotify = new_artist.spotify_data.read().read();
        let unlocked_tiktok = new_artist.tiktok_data.read().read();
        let unlocked_twitter = new_artist.twitter_data.read().read();
        let unlocked_yt_channel = new_artist.yt_channel_data.read().read();
        let unlocked_yt_artist = new_artist.yt_channel_data.read().read();
        match sqlx::query("INSERT INTO artists (id, name, genre, socials, background, deezer, instagram, soundcloud, spotify, tiktok, twitter, yt_channel, yt_artist) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13) RETURNING id, name, genre, socials, background, deezer, instagram, soundcloud, spotify, tiktok, twitter, yt_channel, yt_artist")
            .bind(new_artist.name)
            .bind(new_artist.genre)
            .bind(new_artist.socials)
            .bind(new_artist.background)
            .bind(unlocked_instagram)
            .bind(unlocked_soundcloud)
            .bind(unlocked_spotify)
            .bind(unlocked_tiktok)
            .bind(unlocked_twitter)
            .bind(unlocked_yt_channel)
            .bind(unlocked_yt_artist)
            .map(|row: PgRow| Artist {
                id: ArtistID(row.get("id")), 
                name: row.get("artist_name"),
                genre: row.get("genre"),
                socials: row.get("socials"),
                background: row.get("background"),
                deezer_data: Arc::new(RwLock::new(row.get("deezer"))),
                instagram_data: Arc::new(RwLock::new(row.get("instagram"))),
                soundcloud_data: Arc::new(RwLock::new(row.get("soundcloud"))),
                spotify_data: Arc::new(RwLock::new(row.get("spotify"))),
                tiktok_data: Arc::new(RwLock::new(row.get("tiktok"))),
                twitter_data: Arc::new(RwLock::new(row.get("twitter"))),
                yt_channel_data: Arc::new(RwLock::new(row.get("yt_channel"))),
                yt_artist_data: Arc::new(RwLock::new(row.get("yt_artist"))),
            })
            .fetch_one(&self.connection)
            .await{
                Ok(artist) => Ok(artist),
                Err(e) => Err(e),
            }
    }

    // Pass limit and offset params to indicate if pagination is wanted by the client
    pub async fn get_artists(self, limit: Option<i32>, offset: i32) -> Result<Vec<Artist>, Error> {
        match sqlx::query("SELECT * from artists LIMIT $1 OFFSET $2")
            // bind method substitutes variables (e.g. $1 -> limit)
            // limit & offset = pagination params in postgreSQL
            .bind(limit)
            .bind(offset)
            // using map to aggregate each row returned from  postgreSQL query into an Artist
            // TODO: Buff up error handling here
            .map(|row: PgRow| Artist {
                id: ArtistID(row.get("id")),
                name: row.get("artist_name"),
                genre: row.get("genre"),
                socials: row.get("socials"),
                background: row.get("background"),
                deezer_data: ArcJson {
                    data: Arc::new(RwLock::new(row.get("deezer"))),
                },
                instagram_data: ArcJson {
                    data: Arc::new(RwLock::new(row.get("instagram"))),
                },
                soundcloud_data: ArcJson {
                    data: Arc::new(RwLock::new(row.get("soundcloud"))),
                },
                spotify_data: ArcJson {
                    data: Arc::new(RwLock::new(row.get("spotify"))),
                },
                tiktok_data: ArcJson {
                    data: Arc::new(RwLock::new(row.get("tiktok"))),
                },
                twitter_data: ArcJson {
                    data: Arc::new(RwLock::new(row.get("twitter"))),
                },
                yt_channel_data: ArcJson {
                    data: Arc::new(RwLock::new(row.get("yt_channel"))),
                },
                yt_artist_data: ArcJson {
                    data: Arc::new(RwLock::new(row.get("yt_artist"))),
                },
            })
            // Returns all artists found
            .fetch_all(&self.connection)
            .await
        {
            Ok(artists) => Ok(artists),
            Err(e) => {
                event!(Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }
}
