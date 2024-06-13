fn main() {
    let rect1 = Rectangle::new(30, 40);
    println!("{}", rect1.area());
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
}


