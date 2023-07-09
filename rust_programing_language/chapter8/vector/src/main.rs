fn main() {
    let _v: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];
    let mut v3 = Vec::new(); // can bee inferred by line 5

    v2.push(4);
    v2.push(5);
    v3.push(1);


    let third: &i32 = &v2[2];
    // v2.push(6); // error. vector may copy old data to new memory
    println!("{}", third);

    v2.push(6); // ok
    
    let third: Option<&i32> = v2.get(2);
    // v2.push(7); // error
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    v2.push(7); // ok

    for i in &v2 {
        // can not delete here
        println!("{i}")
    }


    enum_vec();

    println!("{}", v3.len());
    v3.pop();
    println!("{}", v3.len());

    
}


enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

fn enum_vec() {

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];
    match &row[0] {
        SpreadsheetCell::Int(i) => println!("cell number is {i}"),
        SpreadsheetCell::Float(i) => println!("cell float number is {i}"),
        SpreadsheetCell::Text(s) => println!("cell contents is {s}"),
    };

    if let SpreadsheetCell::Text(s) = &row[1] {
        println!("cell contents is {s}");
    }
}
