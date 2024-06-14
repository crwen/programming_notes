use std::{future::Future, rc::Rc};

use futures::join;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct Data {
    data: u32,
}

// 多个不同的 `async` 能够访问相同的局部变量
#[allow(unused)]
async fn blocks() {
    let my_string = String::from("foo");

    let future_one = async {
        // ...
        println!("{my_string}");
    };
    let future_two = async {
        // ...
        println!("{my_string}");
    };

    let ((), ()) = futures::join!(future_one, future_two);
}

#[allow(unused)]
fn move_block() -> impl Future<Output = ()> {
    let my_string = String::from("foo");
    let my_data = Data { data: 1 };
    async move {
        // ...
        println!("{my_string}"); // my_string 被捕获, Move
        println!("{:?}", my_data); // my_data 被捕获，Copy
    };
    async move {
        // ...
        // println!("{my_string}"); // Error, my_string 已经被移动了
        println!("{:?}", my_data); // my_data 被捕获，Copy
    }
}
