use serde::Deserialize;
use reqwest::header::AUTHORIZATION;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub owner: PlaylistOwner
}

#[derive(Deserialize, Debug)]
pub struct PlaylistOwner {
    pub display_name: String
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ItemType {
    Item(PlaylistItem),
    None(DummyItem)
}

#[derive(Deserialize, Debug)]
pub struct PlaylistItem {
    pub added_at: DateTime<Utc>,
    pub track: Track
}

#[derive(Deserialize, Debug)]
struct DummyItem {
}

#[derive(Deserialize, Debug)]
pub struct Track {
    pub name: String
}

pub struct Spotify {
    pub client: reqwest::blocking::Client,
    pub token: String
}



impl Spotify {
    fn json(&self, url: &str) -> Option<String>{
        self.client
            .get(url)
            .header(AUTHORIZATION, &self.token)
            .send()
            .ok()?
            .text()
            .ok()
    }

    pub fn list_playlists(& self, profile_id: &str) -> Option<Vec<Playlist>>{
        let url = format!("https://api.spotify.com/v1/users/{}/playlists?limit=50", profile_id);

        let json = self.json(&url)?;

        #[derive(Deserialize, Debug)]
        struct PlaylistsResponse {
            items: Vec<Playlist>
        }

        let parsed: PlaylistsResponse = serde_json::from_str(&json).ok()?;

        Some(parsed.items)
    }

    pub fn items(& self, playlist_id: &str) -> Option<Vec<PlaylistItem>>{
        let url = format!("https://api.spotify.com/v1/playlists/{}/tracks?market=es&fields=total", playlist_id);

        let json = self.json(&url)?;

        #[derive(Deserialize)]
        struct SizeResponse {
            total: u32
        }
        let parsed: SizeResponse = serde_json::from_str(&json).expect("fail");

        let offset = if parsed.total < 100 { 0 } else { parsed.total - 100 };

        let url = format!("https://api.spotify.com/v1/playlists/{}/tracks?market=es&offset={}&fields=items(added_at.id%2Ctrack(name))", playlist_id,offset);

        let json = self.json(&url)?;

        #[derive(Deserialize)]
        struct PlaylistResponse {
            items: Vec<ItemType>,
        }

        let parsed: PlaylistResponse = serde_json::from_str(&json).expect("fail");

        Some(parsed.items.into_iter().filter_map(|i| match i { ItemType::Item(pi) => Some(pi), ItemType::None(_) => None}).collect())
    }
}
