use std::ops::Deref;

fn main() {
    normal_reference();
    box_like_normal_reference();

    mybox_like_normal_reference();
}

fn normal_reference() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}


fn box_like_normal_reference() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn mybox_like_normal_reference() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
    
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {

    fn drop(&mut self) {
        println!("Drop MyBox...");
    }
    
}
