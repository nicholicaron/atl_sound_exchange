use hyper::client::{Client, };
use serde_json::value::Value as json; 

// TODO: Still prototyping, unable to try without api_key

fn api_call(http_client: &Client, artist_name: String, streaming_service: String) -> Result<json, Error> {
    let data: json = http_client
        // TODO: How to include headers?
        .get(&format!($"CHARTMETRIC_API_LINK/{}/{}", artist_name, streaming_service))
        .send()?
        .json?
}

fn main() {
    let artist_name = "temp_placeholder".to_string();
    let streaming_service = "temp_placeholder".to_string();

    api_call(artist_name, streaming_service);
}
