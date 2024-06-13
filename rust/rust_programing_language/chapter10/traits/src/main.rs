use std::fmt::Display;

use traits::{Tweet, Summary};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };

    notify(&tweet);
    notify2(&tweet);

    notify3(&tweet);

    returns_summarizable(&tweet);


}

fn notify(item: &(impl Summary + Display)) {
    println!("{}", item.summarize_more());
    println!("1 new tweet: {}", item.summarize());
}

fn notify2<T: Summary + Display>(item: &T) {
    println!("{}", item.summarize_more());
    println!("1 new tweet: {}", item.summarize());
}

fn notify3<T>(item: &T) -> i32
where 
    T: Display + Summary{

    println!("{}", item.summarize_more());
    println!("1 new tweet: {}", item.summarize());
    0
}


fn returns_summarizable(tw: &Tweet) -> impl Summary {
    Tweet {
        username: tw.username.clone(),
        content: tw.content.clone(),
        reply: tw.reply,
        retweet: tw.retweet
    }
}


struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x: x, y: y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
