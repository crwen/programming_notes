use std::mem::size_of;

// A 与 B 的内存分布可能不同。但是同一类型的实例的内存分布相同
//
#[allow(dead_code)]
struct A {
    a: u8,
    b: u32,
    c: u16,
}

#[allow(dead_code)]
struct B {
    a: u8,
    b: u32,
    c: u16,
}

#[allow(dead_code)]
struct C {
    b: u64,
}

#[allow(dead_code)]
struct Foo<T, U> {
    count: u16,
    data1: T,
    data2: U,
}

// enum
// data: u64    tag: u8
#[allow(dead_code)]
enum Enum {
    A(u32),
    B(u64),
    C(u8),
    D(u64),
    E(A),
    F(B),
    G(C),
}

enum EnumNone {
    OK(()),
}

#[allow(dead_code)]
pub fn size_of_foo<T, U>() -> usize {
    size_of::<Foo<T, U>>()
}

#[allow(dead_code)]
fn main() {
    println!("size of () {}", size_of::<()>()); // 0
    println!("size of A {}", size_of::<A>()); // 8
    println!("size of B {}", size_of::<B>()); // 8
    println!("size of C {}", size_of::<B>()); // 8
    println!("size of Foo<u16,u32> {}", size_of_foo::<u16, u32>()); // 8
    println!("size of Foo<u32,u16> {}", size_of_foo::<u32, u16>()); // 8
    println!("size of Foo<A,B> {}", size_of_foo::<A, B>()); // 20
    println!("size of Foo<A,C> {}", size_of_foo::<A, C>()); // 24
    println!("size of Enum {}", size_of::<Enum>()); // 16
    println!("size of EnumNone {}", size_of::<Enum>()); // 16
}
