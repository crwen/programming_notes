use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
struct Foo;

#[allow(unused)]
impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }
    fn share(&self) {}
}

#[allow(warnings)]
fn main() {
    let mut foo = Foo;
    let loan = foo.mutate_and_share();
    // let loan = Foo::mutate_and_share(&mut foo);
    // foo.share(); // Error
    println!("{:?}", loan);
}

// impl Foo {
//     fn mutate_and_share<'a>(&'a mut self) -> &'a Self { &'a *self }
//     fn share<'a>(&'a self) {}
// }
//
// fn main() {
//     'b: {
//         let mut foo: Foo = Foo;
//         'c: {
//             let loan: &'c Foo = Foo::mutate_and_share::<'c>(&'c mut foo);
//             'd: {
//                 Foo::share::<'d>(&'d foo);
//             }
//             println!("{:?}", loan);
//         }
//     }
// }
//

// 不正确地缩减借用
// fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
// where
//     K: Clone + Eq + Hash,
//     V: Default,
// {
//     match map.get_mut(&key) {
//         Some(value) => value,
//         None => {
//             map.insert(key.clone(), V::default());
//             // HashMap::insert(&mut map)
//             map.get_mut(&key).unwrap()
//         }
//     }
// }
//
