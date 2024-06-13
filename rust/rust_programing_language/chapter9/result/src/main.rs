use std::{fs::File, io::{Write, Read}, panic};
use std::fs;

fn main() {

}

fn _open_file() {

    let file_result = File::open("hello.txt");
    let mut f = match file_result {
        Ok(file) => file,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => panic!("Problem creating the file: {:?}", e)
            },
            other_err => {
                panic!("Problem opening the file: {:?}", other_err)
            }
        }
    };
    let w_result = f.write("hello".as_bytes());
    if let Err(err) = w_result {
        panic!("Problem writting the file {:?}", err);
    } 
}

fn _open_file_fail() {
    // Ok => value in Ok
    // Err => panic!
    let _f = File::open("hello.txt").unwrap();

    let _f2 = File::open("helo.txt")
        .expect("hello.txt should be included in this project");

}

fn _propagating_errors() -> Result<String, std::io::Error> {

    let file_result = File::open("hello.txt");

    let mut f = match file_result {
        Ok(file) => file,
        // if file not exist, or other error, return err
        Err(e) => return Err(e),
    };

    let mut content = String::new();

    match f.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        // if file can not be read, return err
        Err(e) => Err(e),
    }

}

// ? return a value from function.
fn _propagating_errors_simplify() -> Result<String, std::io::Error> {

    // Ok ==>  the value in Ok
    // Err ==> return the error(convert to the type of return type)
    let mut f = File::open("hello.txt")?;

    let mut content = String::new();

    f.read_to_string(&mut content)?;
    Ok(content)
}

fn _return_simplify() -> Option<i32> {

    let op = Some(5);
    // None ==> return None
    let res = op?;
    Some(res)
}


fn _read_string_from_file() -> Result<String, std::io::Error> {
    fs::read_to_string("hello.txt")
}
