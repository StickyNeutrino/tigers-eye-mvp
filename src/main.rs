mod spotify;

use spotify::{ Spotify, PlaylistItem, Playlist };

fn main() {
    let token = "Bearer ..."
        .to_string();

    let profiles = vec![""];

    let client = reqwest::blocking::Client::builder()
                .https_only(true)
                .build()
                .unwrap();

    let api_client = Spotify { client, token };

    let scrape: Vec<(Playlist, Vec<PlaylistItem>)> = profiles
        .iter()
        .flat_map(|profile_id| {
            let ret = api_client
                .list_playlists(profile_id)
                .unwrap_or_default();
            println!("{}, {}", profile_id, ret.len());
            ret
        })
        .map(|playlist| {
            let items = api_client
                .items(&playlist.id)
                .unwrap_or_default();
                
                (playlist, items)
        })
        .collect();

    let mut entries = scrape
    .into_iter()
    .flat_map(|(playlist, items)|
    	items
    	.into_iter()
    	.map( move|item|(item, playlist.owner.display_name.clone(), playlist.name.clone())))
    .collect::<Vec<(PlaylistItem, String, String)>>();
    
    entries.sort_by(|a, b| a.0.added_at.cmp(&b.0.added_at));
    
    entries.into_iter().for_each(|(track, owner, playlist)|
    	println!("{}|{}|{}| {}", track.added_at, owner, playlist, track.track.name));
    
}
