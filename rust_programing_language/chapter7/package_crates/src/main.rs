use crate::garden::vegetables::Asparagus;

pub mod garden;
// crate root file
fn main() {
    let plant = Asparagus{};
    println!("I'm growing {:?}!", plant);
}
