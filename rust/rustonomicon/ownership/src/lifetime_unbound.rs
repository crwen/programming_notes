#[allow(unused)]
fn main() {
    unbound();
}

fn get_str<'a>(s: *const String) -> &'a str {
    unsafe { &*s }
}

fn get_str_safe(s: &str) -> &str {
    s
}

#[allow(unused)]
fn unbound() {
    println!("---------------------------------------");
    let soon_dropped = String::from("hello");
    let dangling = get_str(&soon_dropped);
    drop(soon_dropped); // drop soon_dropped 后，dangling 仍然存活(unbound)
    println!("Invalid str: {}", dangling); // Invalid str: gӚ_`
    println!("---------------------------------------");

    let soon_dropped = String::from("hello");
    let safe = get_str_safe(&soon_dropped);
    drop(soon_dropped);
    // println!("Invalid str: {}", safe); // Error 因为 safe 的生命周期不应该比 soon_dropped 长
}

#[cfg(test)]
mod test {
    use super::unbound;

    #[test]
    fn unbound_test() {
        unbound();
    }
}
