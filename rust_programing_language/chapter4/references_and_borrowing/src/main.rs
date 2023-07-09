fn main() {

    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("{}, {s1}, {len}", &s1);

    change(&mut s1);
    change(&mut s1); // ok
    println!("{}, {s1}, {len}", &s1);

    let _r1 = &mut s1; 
    let r2 = &mut s1;
    // let r3 = &mut s1; // error
    // let r4 = &s1; // error

    println!("{}", r2);
}

fn calculate_length(s: &String) -> usize {
    // s.push_str(", world!"); // error
   s.len()
}

fn change(s: &mut String) -> usize {
    s.push_str(", world!");
   s.len()
}
