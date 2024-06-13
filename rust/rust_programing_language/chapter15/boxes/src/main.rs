
use crate::List::{Cons, Nil};

fn main() {

    example_use();

    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

}

fn example_use() {
    let b = Box::new(5); // allocated on heap
    println!("b = {b}");
} // both box and the data(on heap) will be deallocated



enum List {
    Cons(i32, Box<List>),
    Nil,
}
