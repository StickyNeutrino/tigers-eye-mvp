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

#[derive(Deserialize, Debug)]
struct PlaylistItem {
    added_at: DateTime<Utc>,
    track: Track
}

#[derive(Deserialize, Debug)]
struct Track {
    name: String
}


fn main() {
    let token = "Bearer ...";

    let profiles = vec!["al5c0jlr79drnbkatho8ev5j2"];

    let client = reqwest::blocking::Client::builder()
                .https_only(true)
                .build()
                .unwrap();

    let mut tracks: Vec<PlaylistItem> = profiles.iter().flat_map(|profile_id| {
        let url = format!("https://api.spotify.com/v1/users/{}/playlists", profile_id);

        let resp = client
            .get(&url)
            .header(AUTHORIZATION, token)
            .send().expect("Request failed");

        let json = resp.text().expect("Failed to load json");


        #[derive(Deserialize, Debug)]
        struct PlaylistsResponse {
            items: Vec<Playlist>
        }

        let parsed: PlaylistsResponse = serde_json::from_str(&json).unwrap();

        parsed.items

    }).flat_map(|playlist| {
        let url = format!("	https://api.spotify.com/v1/playlists/{}/tracks", playlist.id);

        let resp = client
            .get(&url)
            .header(AUTHORIZATION, token)
            .send().expect("Request failed");

        let json = resp.text().expect("Failed to load json");


        #[derive(Deserialize, Debug)]
        struct PlaylistResponse {
            items: Vec<PlaylistItem>
        }

        let parsed: PlaylistResponse = serde_json::from_str(&json).unwrap();

        parsed.items
    })
    .collect();

    tracks.sort_by(|a, b| a.added_at.cmp(&b.added_at));

    for track in tracks {
        println!("{}| name: {}", track.added_at, track.track.name);
    }
}
