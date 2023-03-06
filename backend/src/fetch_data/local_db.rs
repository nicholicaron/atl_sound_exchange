use reqwest::Url;

use crate::types::artist::{list::*, *};
use chrono::{Duration, NaiveDate};

pub fn get_id(name: ArtistList) -> ArtistID {
    name.get()
}

pub fn get_genre(id: ArtistID) -> ArtistID {}

pub fn get_socials(id: ArtistID) -> Box<[Url; 4]> {}

pub fn get_background(id: ArtistID) -> Box<[String; 4]> {}

pub fn get_snapshot(
    since: NaiveDate,
    until: NaiveDate,
    id: ArtistID,
    services: Vec<Service>,
) -> ArtistSnapshot {
}
