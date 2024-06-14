use std::future::Future;

#[allow(unused)]
async fn foo(x: &u8) -> u8 {
    *x
}

// 根据生命周期规则改写后：

// Is equivalent to this function:
#[allow(unused)]
fn foo_expanded<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
    async move { *x }
}

// 说明返回值 `Future` 受到生命周期参数的约束

// async fn bad() -> impl Future<Output = u8> {
//     let x = 5;
//     foo(&x) // ERROR: `x` does not live long enough
// }

// async fn bad() -> u8 {
//     let x = 5;
//     foo(&x) // ERROR: `x` does not live long enough
// }
#[allow(unused)]
fn good() -> impl Future<Output = u8> {
    async {
        let x = 5;
        foo(&x).await
    }
}

fn main() {
    println!("Hello, world!");
}
