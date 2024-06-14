use std::{thread, time::Duration};

use futures::executor::block_on;

fn main() {
    // block_on 会阻塞当前线程直到指定的 Future 执行完成
   block_on(do_something());
}

// async 返回一个 Future
async fn do_something() {
    // await 不会阻塞当前线程，而是异步等待 Future 完成
    // do_something_other().await;
    // do_something_else().await;
    futures::join!(do_something_other(), do_something_else());
    println!("do_something!");
}

async fn do_something_other() {
    println!("do_something_other!");
}

async fn do_something_else() {
    thread::sleep(Duration::from_millis(1000));
    println!("do_something_else!");
}
