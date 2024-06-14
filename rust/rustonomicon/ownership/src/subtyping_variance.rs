// 注意：debug 需要两个具有相同生命周期的参数
fn debug<'a>(a: &'a str, b: &'a str) {
    println!("a = {a:?} b = {b:?}");
}

#[allow(unused)]
fn main() {}

#[allow(unused)]
fn subtyping() {
    let hello: &'static str = "hello";
    {
        let world = String::from("world");
        let world = &world; // 'world 的生命周期比 'static 短
        debug(hello, world); // hello 从 &'static str 降级为 &'world str
    }
    println!("{hello}");
}

#[cfg(test)]
mod subtyping_variance_test {

    use super::*;

    #[test]
    fn example_test() {
        subtyping();
        panic!("abc")
    }
}
