use futures::executor::block_on;
use multiple_futures::join;

fn main() {
    let f = join::get_book_and_music();

    block_on(f);
}
