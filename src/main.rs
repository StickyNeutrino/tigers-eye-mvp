use serde::Deserialize;
use reqwest::header::AUTHORIZATION;

#[derive(Deserialize, Debug)]
struct Playlist {
    id: String,
    name: String
}

fn main() {
    let token = "Bearer ...";

    let profiles = vec!["al5c0jlr79drnbkatho8ev5j2"];

    let client = reqwest::blocking::Client::builder()
                .https_only(true)
                .build()
                .unwrap();

    let mut playlists: Vec<String> = vec![];

    profiles.iter().for_each(|profile_id|{
        let url = format!("https://api.spotify.com/v1/users/{}/playlists", profile_id);

        let resp = client
            .get(&url)
            .header(AUTHORIZATION, token)
            .send().expect("Request failed");

        let json = resp.text().expect("Failed to load json");


        #[derive(Deserialize, Debug)]
        struct PlaylistResponse {
            items: Vec<Playlist>
        }

        let parsed: PlaylistResponse = serde_json::from_str(&json).unwrap();

        for item in parsed.items {
            println!("{}, {}",item.name, item.id);
        }
    });



}
