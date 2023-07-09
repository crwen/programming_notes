fn main() {
    let condition = true;
    let n = if condition { 5 } else { 6 };
    println!("The value of n is {n}");

    let mut counter = 0;
    let res = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The res is {res}");

    counter = 0;
    while counter != 10 {
        counter += 1
    }
    println!("The counter is {counter}");


    let arr = [10, 20, 30, 40];
    let mut idx = 0;
    while idx < arr.len() {
        println!("arr[{}] is {}", idx, arr[idx]);
        idx += 1;
    }

    for e in arr {
        println!("{} is in arr", e);
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");

}
