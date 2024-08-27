fn main() {
    println!("{}", return_function());
    println!("{}", receive_function());
}

fn return_function() -> i32 {
    #![allow(unused_variables)]
    let x = 1;
    let f = move |y| {
        let x = y + 1;
        move |z| x + y + z
    };

    let x = 3; // irrelevant

    let g = f(4); // return a function that adds 8 to its argument

    let y = 5; // irrelevant

    g(6) // return 15
}

fn receive_function() -> i32 {
    #![allow(unused_variables)]
    let x = 4;
    fn f(g: impl Fn(i32) -> i32) -> i32 {
        let x = 3; // irrelevant
        g(2)
    }

    let h = move |y: i32| x + y; // add 4 to its argument

    f(h) // return 6
}
