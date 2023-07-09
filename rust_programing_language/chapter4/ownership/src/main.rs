fn main() {
    // literals is immutable
    // String allocat at heap
    let mut s: String = String::from("Hello");
    s.push(',');
    s.push_str(" world!");
    println!("{s}");


    let s2 = s; // move, s can not used any more
    // println!("{s}"); // error
    println!("{s2}");

    takes_ownership(s2); // s2 move into the function
    // s2 can not be used any more
    // println!("{}", s2); // error

    let x = 5;
    makes_copy(x); // x move into the function
    // x can be used

    println!("{x}");

    func_return();
}

fn makes_copy(_n: i32) {}

fn takes_ownership(str: String) {
    println!("{}", str);
}

fn func_return() {

    let s1 = gives_ownership(); // move its return value into s1
    
    let s2 = String::from("hello world");     

    let s3 = takes_and_gives_back(s2); // s2 is moved into the function; return value is moved into s3

    println!("{}, {}", s1, s3)
}

fn gives_ownership() -> String {
    let str = String::from("hello");
    str
}

fn takes_and_gives_back(str: String) -> String {
    str // return str, and move out to the calling function
}
