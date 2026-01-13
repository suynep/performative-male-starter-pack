use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrimaryArtist {
    id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hit {
    pub result: Reslt,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reslt {
    primary_artist: PrimaryArtist,
    artist_names: String,
    pub header_image_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseWithinSearch {
    pub hits: Vec<Hit>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub response: ResponseWithinSearch,
}
