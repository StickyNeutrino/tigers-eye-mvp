use serde::Deserialize;
use reqwest::header::AUTHORIZATION;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
struct Playlist {
    id: String,
    name: String,
    owner: PlaylistOwner
}

#[derive(Deserialize, Debug)]
struct PlaylistOwner {
    display_name: String
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ItemType {
    Item(PlaylistItem),
    None(DummyItem)
}

#[derive(Deserialize, Debug)]
struct PlaylistItem {
    added_at: DateTime<Utc>,
    track: Track
}

#[derive(Deserialize, Debug)]
struct DummyItem {
}

#[derive(Deserialize, Debug)]
struct Track {
    name: String
}

struct Spotify {
    client: reqwest::blocking::Client,
    token: String
}

fn main() {
    let token = "Bearer ..."
        .to_string();

    let profiles = vec![""];

    let client = reqwest::blocking::Client::builder()
                .https_only(true)
                .build()
                .unwrap();

    let api_client = Spotify { client, token };

    let mut tracks: Vec<PlaylistItem> = profiles
        .iter()
        .flat_map(|profile_id| {
            let ret = api_client
                .list_playlists(profile_id)
                .unwrap_or_default();
            println!("{}, {}", profile_id, ret.len());
            ret
        })
        .flat_map(|playlist| {
            api_client
                .items(&playlist.id)
                .unwrap_or_default()
        })
        .collect();

    tracks.sort_by(|a, b| a.added_at.cmp(&b.added_at));

    for track in tracks {
        println!("{}, {}", track.added_at, track.track.name);
    }
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

    fn list_playlists(& self, profile_id: &str) -> Option<Vec<Playlist>>{
        let url = format!("https://api.spotify.com/v1/users/{}/playlists?limit=50", profile_id);

        let json = self.json(&url)?;

        #[derive(Deserialize, Debug)]
        struct PlaylistsResponse {
            items: Vec<Playlist>
        }

        let parsed: PlaylistsResponse = serde_json::from_str(&json).ok()?;

        Some(parsed.items)
    }

    fn items(& self, playlist_id: &str) -> Option<Vec<PlaylistItem>>{
        let url = format!("	https://api.spotify.com/v1/playlists/{}/tracks?market=es&fields=total", playlist_id);

        let json = self.json(&url)?;

        #[derive(Deserialize)]
        struct SizeResponse {
            total: u32
        }
        let parsed: SizeResponse = serde_json::from_str(&json).expect("fail");

        let offset = if parsed.total < 100 { 0 } else { parsed.total - 100 };

        let url = format!("	https://api.spotify.com/v1/playlists/{}/tracks?market=es&offset={}&fields=items(added_at.id%2Ctrack(name))", playlist_id,offset);

        let json = self.json(&url)?;

        #[derive(Deserialize)]
        struct PlaylistResponse {
            items: Vec<ItemType>,
        }

        let parsed: PlaylistResponse = serde_json::from_str(&json).expect("fail");

        Some(parsed.items.into_iter().filter_map(|i| match i { ItemType::Item(pi) => Some(pi), ItemType::None(_) => None}).collect())
    }
}
