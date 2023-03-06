pub mod genre;
pub mod list;

// TODO How do we insert artist profile pics TODO

use crate::fetch_data;
use crate::error::Error;
use chrono::offset::{Local, TimeZone};
use chrono::NaiveDate;
use parking_lot::RwLock;
use plotters::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::value::Value as json;
use std::sync::Arc;

use self::genre::Genre;
use self::list::ArtistList;

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
// using i32 for postgreSQL compatability and to ensure that Chartmetric IDs are within range
pub struct ArtistID(pub i32);

// Heap Allocate a vector of ArtistSnapshots parameterized by the timestamp
pub struct Artist {
    pub id: ArtistID,
    pub name: list::ArtistList,
    pub genre: genre::Genre,
    // socials.0 = spotify link
    // socials.1 = apple music link
    // socials.2 = instagram link
    // socials.3 = twitter link
    pub socials: Box<[String; 4]>,
    // background.0 = city
    // background.1 = state
    // background.2 = country
    // background.3 = description
    pub background: Box<[String; 4]>,
    // use VecDeque to pop_front and pop_back for ranges
    pub snapshots: Arc<RwLock<Vec<ArtistSnapshot>>>,
}

impl Artist {
    pub fn new(name: ArtistList) -> Self {
        let from = "2017-01-01";
        // how to do current day?
        let until = todo!();
        let duration = [from..until].days();
        let (id, genre) = name.get()?;
        let socials = Box::new(Artist::get_background(id));
        let background = Box::new(Artist::get_background(id));

        let mut snapshots = Arc::new(RwLock::new(Vec::<ArtistSnapshot>::new()));
        for timestamp in [duration] {
            snapshots.write().push(ArtistSnapshot::new(id, timestamp));
        }
        Artist{
            id,
            name,
            genre,
            socials,
            background,
            snapshots,
        }
    }

}

impl std::fmt::Debug for Artist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Artist")
            .field("id", &ArtistID)
            .field("name", &self.name)
            .field("from", &self.snapshots.0)
            .field("duration", &self.snapshots.len())
            .finish_non_exhaustive()
    }
}
// ------------------------------------------------------------------------------------------------
// SHOULD SNAPSHOTS BE IMPLEMENTED USING TRAIT OBJECTS/DYN DISPATCH?
// ------------------------------------------------------------------------------------------------
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArtistSnapshot {
    pub timestamp: NaiveDate,
    pub deezer_data: json,
    pub instagram_data: json,
    pub soundcloud_data: json,
    pub spotify_data: json,
    pub tiktok_data: json,
    pub twitter_data: json,
    pub yt_channel_data: json,
    pub yt_artist_data: json,
}

impl ArtistSnapshot {
    pub fn new(id: ArtistID, timestamp: NaiveDate) -> Self {
        ArtistSnapshot {
            timestamp,
            deezer_data: Self::get_service(id, timestamp, Service::Deezer),
            instagram_data: Self::get_service(id, timestamp, Service::Instagram),
            soundcloud_data: Self::get_service(id, timestamp, Service::Soundcloud),
            spotify_data: Self::get_service(id, timestamp, Service::Spotify),
            tiktok_data: Self::get_service(id, timestamp, Service::Tiktok),
            twitter_data: Self::get_service(id, timestamp, Service::Twitter),
            yt_channel_data: Self::get_service(id, timestamp, Service::YtChannel),
            yt_artist_data: Self::get_service(id, timestamp, Service::YtArtist),
        }
    }

    fn get_service(id: ArtistID, timestamp: NaiveDate, service: Service) -> json {
        // call db for Artist data at timestamp for service, if this call fails, call API instead
        // then cache results in DB before returning
        if let Some(data) = sqlx::query!(todo!()) {
            Ok(data)
        } else {
            chartmetric_api::service_from_api(timestamp, , id, service)
        }
    }
}

impl std::fmt::Debug for ArtistSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArtistSnapshot")
            .field("timestamp", &self.timestamp)
            .finish_non_exhaustive()
    }
}

// Common fn for graphing
fn parse_time(t: &str) -> NaiveDate<Local> {
    Local
        .datetime_from_str(&format!("{} 0:0", t), "%Y-%m-%d %H:%M")
        .unwrap()
        .naive_date()
}

/*
pub enum GraphFreq {
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}
*/

#[derive(PartialEq)]
pub enum Service {
    Deezer,
    Instagram,
    Soundcloud,
    Spotify,
    Tiktok,
    Twitter,
    YtChannel,
    YtArtist,
}

// implement display for parsing into url for api call in chartmetric_api/mod.rs
impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Service::Deezer => write!(f, "deezer"),
            Service::Instagram => write!(f, "instagram"),
            Service::Soundcloud => write!(f, "soundcloud"),
            Service::Spotify => write!(f, "spotify"),
            Service::Tiktok => write!(f, "tiktok"),
            Service::Twitter => write!(f, "twitter"),
            Service::YtChannel => write!(f, "youtube_channel"),
            Service::YtArtist => write!(f, "youtube_artist"),
        }
    }
}

#[derive(PartialEq)]
pub enum Metric {
    Price,
    Service(Service),
}

impl Artist {
    // Goal is to create a candlestick graph for a given artist's streaming metrics (for a particular streaming service)
    // Pull data from PostgreSQL DB and send graph to frontend
    // Should also input frequency of data -- e.g. daily, weekly, monthly, yearly, etc
    pub fn graph_metric(
        &self,
        metrics: Vec<Metric>,
        duration: (NaiveDate, NaiveDate),
        resolution: (u32, u32),
        frequency: GraphFreq,
    ) -> Result<(), Error> {
        let artist_name = self.name.clone();

        // want f_name to depend on self.name & streaming service, should this even be a file
        // though? What is the interface to frontend
        const OUTPUT_F_NAME: &'static str = "./output/artist/platform.png";

        let root = BitMapBackend::new(OUTPUT_F_NAME, resolution).into_drawing_area();
        root.fill(&BLACK);

        let num_graphs = metrics.len();
        // decide graph panel layout:
        //      If no service is selected, go with default of spotify and single panel layout
        //      If one is selected,
        //      If there is an even number of services, create two columns and divide rows evenly
        //      If there is a multiple of 3, create two columns and divide rows evenly
        //      Else, create 3 columns and num_graphs/3 + 1 rows. But split first row into
        //      num_graphs mod 3 columns
        let mut service: String = "Spotify";
        let (col, row): (u16, u16) = match num_graphs {
            0 => (1, 1),
            1 => (1, 1),
            num if num % 2 == 0 => (2, num / 2),
            _ => (3, num_graphs / 3 + 1),
        };

        let rows = root.split_vertically(resolution / row);

        let areas = for row in rows {
            let mut uneven: bool = num_graphs % col == 0;
            match uneven {
                // if there is an uneven multiple of panels
                // Split first row into num_graphs mod col columns
                // Then split rest of rows into col columns
                // E.g. if 5 entries:
                //      XXX   |  XXX
                //      XX | XX | XX
                true => {
                    row.split_horizontally(num_graphs % col);
                    uneven = false;
                }
                false => root.split_horizontally(col),
            }
        };

        let (to_date, from_date) = (duration.1, duration.0);

        for (id, area) in areas.into_iter().enumerate() {
            let metric = metrics[id - 1];

            let data = match metric {
                Metric::Price => self.get_data(Metric::Price, duration),
                Metric::Deezer => self.get_data(Metric::Deezer, duration),
                Metric::Instagram => self.get_data(Metric::Instagram, duration),
                Metric::Soundcloud => self.get_data(Metric::Soundcloud, duration),
                Metric::Spotify => self.get_data(Metric::Spotify, duration),
                Metric::Tiktok => self.get_data(Metric::Tiktok, duration),
                Metric::Twitter => self.get_data(Metric::Twitter, duration),
                Metric::YtArtist => self.get_data(Metric::YtArtist, duration),
                Metric::YtChannel => self.get_data(Metric::YtChannel, duration),
            };

            let data_range: (u32, u32) = (data.into_iter().min(), data.into_iter().max());

            let caption = artist_name + " " + metric.to_string() + " data";

            // If metric is price, do candlestick chart
            // Goal is to create candlestick graph for a given artists stock price
            // Initially calculate price based on derivative of streaming metrics
            // Eventually incorporate buying/selling demand & sentiment analysis
            if let Metric::Price = metric {
                let mut chart = ChartBuilder::on(&area)
                    // Label_area_size is a factor of resolution -- varying label sizes
                    .x_label_area_size(resolution.0 / (col * 20))
                    .y_label_area_size(resolution.1 / (row * 20))
                    // Consider Century font instead
                    .caption(
                        caption.as_str(),
                        ("sans-serif", resolution.0 / 10).into_font(),
                    )
                    // build cartesian (x range, y range) what variables make sense?
                    .build_cartesian_2d(from_date..to_date, data_range.0..data_range.1)?;

                chart.configure_mesh().light_line_style(&BLACK).draw()?;

                // Figure how to do a variable amount of data points
                // How to display key?
                chart.draw_series(data.iter().map(|x| {
                    // x is a data point, change to fit with our data format
                    // ----------------------------------------------------
                    todo!();
                    CandleStick::new(
                        parse_time(x.0),
                        x.1,
                        x.2,
                        x.3,
                        x.4,
                        GREEN.filled(),
                        RED.filled(),
                        15,
                    )
                }))?;
            } else {
                // Histogram
                let mut chart = ChartBuilder::on(&area)
                    .x_label_area_size(resolution.0 / (col * 20))
                    .y_label_area_size(resolution.1 / (col * 20))
                    .caption(
                        caption.as_str(),
                        ("sans-serif", resolution.0 / 10).into_font(),
                    )
                    .build_cartesian_2d(from_date..to_date, data_range.0..data_range.1)?;
                chart.configure_mesh().disable_x_mesh().disable_y_mesh();

                let histogram = Histogram::vertical(&chart)
                    .style(GREEN.filled())
                    .margin(0)
                    .data(data.iter().map(|x| {
                        todo!();
                    }));
                chart.draw_series(histogram);
            }

            // To avoid the IO failure being ignored silently, we manually call the present function
            root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
            println!("Reuslt has been saved to {}", OUTPUT_F_NAME);
        }
    }
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

/*
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewArtist {
    // id is i32 for postgresql compatibility -- may change after considering db alternatives
    pub name: String,
    pub genre: String,
    pub socials: [String; 4],
    pub background: [String; 4],
    pub deezer_data: Arc<RwLock<json>>,
    pub instagram_data: Arc<RwLock<json>>,
    pub soundcloud_data: Arc<RwLock<json>>,
    pub spotify_data: Arc<RwLock<json>>,
    pub tiktok_data: Arc<RwLock<json>>,
    pub twitter_data: Arc<RwLock<json>>,
    pub yt_channel_data: Arc<RwLock<json>>,
    pub yt_artist_data: Arc<RwLock<json>>,
}
*/
