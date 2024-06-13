use std::thread;

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Read, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Read);
    let giveaway = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway);

    let user_pref2: Option<ShirtColor> = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);


    capturing_reference();
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Read,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // let expensive_closure = ||  {
        //     self.most_stocked()
        // };
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Read => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Read
        } else {
            ShirtColor::Blue
        }
    }
}


fn _example_closure() {
    let return_self = |x| x;

    let _s = return_self(String::from("hello")); // ok
    // let n = return_self(5); // error
    let _n = return_self("world".to_string()); // ok
}


fn capturing_reference() {

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    // let mut borrows_mutable = || list.push(7); // error

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    let mut borrows_mutable = || list.push(7);
    // println!("After calling closure: {:?}", list); // error, because list mut reference hold by closure
    borrows_mutable();
    println!("After calling closure: {:?}", list);


    // move 
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    // println!("After calling closure: {:?}", list); // error. ownership has been moved
}




