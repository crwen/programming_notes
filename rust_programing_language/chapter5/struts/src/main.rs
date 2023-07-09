fn main() {
    let user1 = build_user(String::from("rust@email.com"), String::from("rust"));
    let user2 = User {
        email: String::from("abc@gmail.com"),
        username: dbg!(String::from("abc")),
        ..user1
    };
    println!("user1: username:{}, active:{}", user1.username, user1.active);
    println!("user2: username:{}, active:{}", user2.username, user2.active);

    let user3 = User {
        email: String::from("abc@gmail.com"),
        ..user1
    };
    println!("{}", user1.email);
    // println!("{}", user1.username); // error
    println!("user3: {}, {}", user3.email, user3.sign_in_count);
    println!("use3: {:?}", user3);
    println!("use3: {:#?}", user3);
    dbg!(user3); // receive ownership of an expression, and return ownership

    let p = Point(1, 2, 4);
    println!("Point: x = {}, y = {}, y = {}", p.0, p.1, p.2);

    let _subject = AlwaysEqual;
}

struct Point(i32, i32, i32);

struct AlwaysEqual;

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}
