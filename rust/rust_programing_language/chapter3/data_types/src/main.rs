fn main() {
    // scalar types:
    //   - intergers: i8(16/32/64/128), u8(16/32/64/128), isize, usize
    //   - float-point: f32(64)
    //   - bool: true/false
    //   - character: char
    //
    // Compound Types
    //   - tuple: let tup: (i32, f64, u8) = (500, 6,4, 1). size can not change
    //   - array: let arr: [i32: 4] = [1, 2, 3, 4];
    //            let arr = [3; 5] // every element in arr is 3
    let tup = (0xff, 0b1111_0000, 32u8, 3.14159265, "hello world");
    let (_, _, _, _, str) = tup;
    let bin = tup.1;
    println!("{bin}, {}", str);

    let months = ["", "January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    println!("{}", months[1]);

}
