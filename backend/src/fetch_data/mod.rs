pub mod chartmetric;
pub mod local_db;

use crate::types::artist::{list::*, *, genre::Genre};
use chrono::{Duration, NaiveDate};
use reqwest::Url;

pub fn get_id(name: ArtistList) -> ArtistID {
    name.get()
}

pub fn get_genre(id: ArtistID) -> Genre {
    local_db::get_genre(id}.or(chartmetric::get_genre(id))

pub fn get_socials(id: ArtistID) -> Box<[Url; 4]> {
    Box::new()
}

pub fn get_background(id: ArtistID) -> Box<[String; 4]> {}

pub fn get_snapshot(
    since: NaiveDate,
    until: NaiveDate,
    id: ArtistID,
    services: Vec<Service>,
) -> ArtistSnapshot {
}
