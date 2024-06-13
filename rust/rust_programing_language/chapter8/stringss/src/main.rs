fn main() {
    new_string();
    update_string();
}


fn new_string() {
    let s = String::new();

    let data = "initial contents";

    let s2 = data.to_string();

    let s3 = String::from("new strign");

    println!("{}, {}, {}", s, s2, s3);
}

fn update_string() {
    let mut s = String::from("foo");
    println!("original string {}", s);
    s.push(' ');
    println!("after append ' ' {}", s);
    s.push_str("bar");
    println!("after append bar {}", s);
}
