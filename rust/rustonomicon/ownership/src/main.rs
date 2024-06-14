#[derive(Debug)]
struct CopyStruct {
    x: u32,
    y: u64,
}

impl Clone for CopyStruct {
    fn clone(&self) -> Self {
        println!("clone...");
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

impl Copy for CopyStruct {}

fn main() {
    let x = CopyStruct { x: 0, y: 1 };
    #[allow(unused)]
    let y = x;
    println!("{:?}", y);
    println!("{:?}", x);
    println!("{:?}", x.clone());
}

#[allow(dead_code)]
fn compute(input: &u32, output: &mut u32) {
    let cached_input = *input;
    if cached_input > 10 {
        *output = 1;
    } else if cached_input > 5 {
        *output = 2;
    }
}
