use std::{future, time::Duration};

pub struct Book {}
pub struct Music {}

async fn get_book() -> Book {
    println!("get book");

    Book {}
}
async fn get_music() -> Music {
    println!("get music");
    Music {}
}

pub async fn get_book_and_music() -> (Book, Music) {
    let book_fut = get_book();
    let music_fut = get_music();
    futures::join!(book_fut, music_fut)
}
