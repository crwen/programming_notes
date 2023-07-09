
mod front_of_house;

pub use crate::front_of_house::hosting;

fn deliver_order() {

}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

}
pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    // front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}
