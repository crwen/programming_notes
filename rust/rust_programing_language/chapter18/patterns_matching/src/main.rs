use std::vec;

fn main() {
    println!("Hello, world!");
}


fn _match_pattern() {
    let x = Some(5);
    let _x = match x {
        Some(i) => println!("match {}", i),
        None => {}
    };

    let y = 5;
    match y {
        1 | 2 => (),
        3 => (),
        _ => (),
    }
    // match y {
    //     1..10 => println!("{y}"),
    //     _ => (),
    // }
}


fn _if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn _while_let() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn _for() {
    let v = vec!['a', 'b', 'c'];

    for (idx, val) in v.iter().enumerate() {
        println!("{} is at index {}", val, idx);
    }
}



struct Point {
    x: i32,
    y: i32,
}


fn _match_struct() {
    let p = Point {x: 0, y: 7};

    match p {
        Point {x, y: 0} => println!("On the x axis at {x}"),
        Point {x: 0, y} => println!("On the y axis at {y}"),
        Point {x , y} => println!("On neither axis: ({x}, {y})"),
    }
    
}

