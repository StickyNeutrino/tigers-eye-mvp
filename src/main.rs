mod spotify;

use std::fs;
use spotify::{ Spotify, PlaylistItem, Playlist };
use chrono::{ DateTime, Duration };
use tokio::time::{ sleep };
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use std::collections::BinaryHeap;


#[tokio::main]
async fn main() {
    let NUM_WORKERS = 8;

    let token = fs::read_to_string("token.txt")
    .expect("Token File Missing");

    let profiles = vec![""];

    let client = reqwest::Client::builder()
                .https_only(true)
                .build()
                .unwrap();

    let api_client = Spotify { client, token };

        let playlist_queue: BinaryHeap<(DateTime<Utc>, Playlist)>  = BinaryHeap::new();

        //Sorted by date_added
        let latest_tracks: BinaryHeap<PlaylistItem> = BinaryHeap::new();



        let worker_channel: Vec<Sender<Playlist>> = (0..NUM_WORKERS)
        .map( |_| {
            let (tx, mut rx) = mpsc::channel(100);
        
            let worker = async move { 
                while let Some(playlist) = rx.recv().await {
            
    
                    let playlist = api_client.get_playlist_items(playlist).await;
                }
            };

            tokio::spawn(worker);

            tx
        } )
        .collect();

        loop {

            for i in 0..NUM_WORKERS {
                let (last_fetch, playlist) = playlist_queue.peek();

                if (last_fetch  < (Utc::now() - Duration::minutes(10))){
                    break;
                } else {
                    worker_channel.send(playlist);
                }
            }
            
        }
        
        



    
    
}
