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
use crate::types::artist::{Artist, ArtistID};

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
                id: ArtistID(row.try_get("id").expect("error fetching id from database")),
                name: row
                    .try_get("name")
                    .expect("error fetching name from database"),
                genre: row
                    .try_get("genre")
                    .expect("error fetching genre from database"),
                socials: row
                    .try_get("socials")
                    .expect("error fetching socials from database"),
                background: row
                    .try_get("background")
                    .expect("error fetching background from database"),
                deezer_data: Arc::new(RwLock::new(
                    row.try_get("deezer")
                        .expect("error fetching deezer data from database"),
                )),
                instagram_data: Arc::new(RwLock::new(
                    row.try_get("instagram")
                        .expect("error fetching instagram data from database"),
                )),
                soundcloud_data: Arc::new(RwLock::new(
                    row.try_get("soundcloud")
                        .expect("error fetching soundcloud data from database"),
                )),
                spotify_data: Arc::new(RwLock::new(
                    row.try_get("spotify")
                        .expect("error fetching spotify data from database"),
                )),
                tiktok_data: Arc::new(RwLock::new(
                    row.try_get("tiktok")
                        .expect("error fetching tiktok data from database"),
                )),
                twitter_data: Arc::new(RwLock::new(
                    row.try_get("twitter")
                        .expect("error fetching twitter data from database"),
                )),
                yt_channel_data: Arc::new(RwLock::new(
                    row.try_get("yt_channel")
                        .expect("error fetching youtube channel data from database"),
                )),
                yt_artist_data: Arc::new(RwLock::new(
                    row.try_get("yt_artist")
                        .expect("error fetching youtube artist data from database"),
                )),
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

/* Artist DB schema prototype:

Artists
    - Artist 1
         + id: int4
         + name: varchar
         + genre: varchar
         + socials (varchar, varchar, varchar, varchar)
         + Background
             * Origin
                 - city: varchar
                 - state: char[2]
                 - country: char[2]
             * Description: varchar
         + deezer: jsonb
         + instagram: jsonb
         + soundcloud: jsonb
         + spotify: jsonb
         + tiktok: jsonb
         + twitter: jsonb
         + yt_channel: jsonb
         + yt_artist: jsonb

Why Jsonb?
Jsonb is stored in a decomposed binary format
    - Slightly slower input due to conversion overhead
    - Significantly faster to process, since no reparsing needed
    - Also supports indexing
*/
