use std::time::Duration;

use futures::executor::block_on;
use start::start;

fn download(url: &str) {
    println!("thread {:?} download {}", std::thread::current().id(), url);
}

async fn download_asyc(url: &str) {
    if url == "https:://www.foo.com" {
        std::thread::sleep(Duration::from_secs(3));
    }
    println!(
        "thread {:?} async download {}",
        std::thread::current().id(),
        url
    );
}

fn thread_download() {
    let t1 = std::thread::spawn(|| download("https:://www.foo.com"));
    let t2 = std::thread::spawn(|| download("https:://www.bar.com"));

    t1.join().expect("thread one panicked");
    t2.join().expect("thread one panicked");
}

async fn async_download() {
    // let future_one = download_asyc("https:://www.foo.com");
    // let future_two = download_asyc("https:://www.bar.com");
    download_asyc("https:://www.foo.com").await;
    download_asyc("https:://www.bar.com").await;
    // futures::join!(future_one, future_two);
    // futures::join!(future_two, future_one);
}

fn main() {
    thread_download();
    block_on(async_download());
    start::async_start();
}
