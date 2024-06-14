use std::time::Duration;

use futures::{executor::block_on, Future};

struct Song {}
async fn learn_song() -> Song {
    println!("learning song");
    std::thread::sleep(Duration::from_secs(3));
    Song {}
}

#[allow(unused)]
async fn sing_song(song: Song) {
    println!("sing song");
}

async fn dance() {
    println!("dance");
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1, f2);
}

pub fn async_start() {
    block_on(async_main());
}

pub trait Common {}

impl<T> Common for T {}
