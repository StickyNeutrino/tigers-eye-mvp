mod spotify;
use rayon::prelude::*;

use std::fs;
use spotify::{ Spotify, PlaylistItem, Playlist };


fn main() {
    let token = fs::read_to_string("token.txt")
    .expect("Token File Missing");

    let profiles = vec![""];

    let client = reqwest::blocking::Client::builder()
                .https_only(true)
                .build()
                .unwrap();

    let api_client = Spotify { client, token };

    let scrape: Vec<(Playlist,Vec<PlaylistItem>)> = profiles
        .par_iter()
        .map(|profile_id| {
            api_client.list_playlists(profile_id)
        })
        .flat_map(|playlists: Option<Vec<Playlist>>| { playlists.unwrap_or_default() })
        .map(|playlist : Playlist| {
            
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
