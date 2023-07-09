
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5;
    println!("The value of x is {x}");
    // x = 6; // error
    // println!("The value of x is {x}"); // error
    let mut y = 5;
    println!("The value of y is {y}");
    y = 6;
    {
        let y = y * 100;
        println!("The value of y in inner scope is {y}");
    }
    println!("The value of y is {y}");

    println!("Three hours in seconds is {THREE_HOURS_IN_SECONDS}");
}

