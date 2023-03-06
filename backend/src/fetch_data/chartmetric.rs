use crate::error::Error as error;
use crate::types::artist::list::ArtistList;
use crate::types::artist::*;
use chrono::{Duration, NaiveDate};
use reqwest;
use reqwest::{header, Url};

// Read API token from file, make sure file is in gitignore to avoid leaking key
static API_TOKEN: &'static str = todo!();

// Needs to be async-awaited and spawn_blocking'ed to avoid blocking the event loop
// pull daily data from a provided range for a provided artist's metrics on a provided service
pub async fn service_from_api(
    since: NaiveDate,
    until: NaiveDate,
    id: ArtistID,
    services: Vec<Service>,
) -> Result<Vec<ArtistSnapshot>, error> {
    let duration: Duration = [since..until].days();
    let mut data: Vec<ArtistSnapshot> = Vec::<ArtistSnapshot>::with_capacity(duration);

    for service in services {
        let url: Url = Url::parse_with_params(
            format!("https://api.chartmetric.com/api/artist/{id}/stat/{service}/").as_str(),
            &[
                ("since", "{since}"),
                ("until", "{until}"),
                ("interpolated", "true"),
            ],
        );

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_static(format!("Bearer {API_TOKEN}").as_str()),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let api_call = client.get(url).send().await?.json().await?;
        data.push(api_call);
    }
}
