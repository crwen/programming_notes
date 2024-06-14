// 每个 `let` 语句都隐含地引入了一个作用域，这对相互引用的的变量来说很重要
#[allow(dead_code, warnings)]
fn lifetimes() {
    let x = 0;
    let z;
    let y = &x;
    z = y;
}

// 'a: {
//     let x: i32 = 0;
//     'b: {
//         let z: &'b i32;
//         'c: {
//             // y 的生命周期一定为 'b，因为对 x 的引用被传递到了 'b 这个作用域
//             let y: &'b i32 = &'b x;
//             z = y;
//         }
//     }
// }
//

// fn outlive_referents(data: &u32) -> &str {
//     let s = format!("{}", data);
//     &s
// }
//           ||
//           \/
//
// fn outlive_referents(data: &u32) -> &str {
//     'b: {
//         let s = format!("{}", data);
//         return &'a s;
//     }
// }
//
//
//
//
//
//
//
#[allow(dead_code, warnings)]
fn aliasing_mutable_referent() {
    let mut data = vec![1, 2, 3];
    let x = &data[0];
    data.push(4);
    // println!("{}", x);
}
// fn aliasing_mutable_referent() {
//     let mut data = vec![1, 2, 3];
//     let x = &data[0];
//     data.push(4);
//     println!("{}", x);
// }
//
// 'a: {
//     let mut data: Vec<i32> = vec![1, 2, 3];
//     'b: {
//         // 'b 这个生命周期范围如我们所愿地小（刚好够 println!）
//         let x: &'b i32 = Index::index::<'b>(&'b data, 0);
//         'c: {
//             // 这里有一个临时作用域，我们不需要更长时间的 &mut 借用
//             Vec::push(&'c mut data, 4); // &'b data 依然存活
//         }
//         println!("{}", x);
//     }
// }

#[allow(dead_code)]
#[derive(Debug)]
struct X<'a>(&'a i32);

impl Drop for X<'_> {
    fn drop(&mut self) {}
}
#[allow(dead_code, warnings)]
fn area_covered_lifetimes() {
    let mut data = vec![1, 2, 3];
    let x = &data[0];
    println!("{}", x);
    data.push(4);

    let mut data2 = vec![1, 2, 3];
    let y = X(&data2[0]);
    println!("{:?}", y);
    // data2.push(4); // error, &data2[0] 依然存活
    // Vec::push(& mut data, 4); // error, &data2[0] 依然存活
    // 编译器会在这里自动插入 drop 函数，也就意味着我们会访问 y 中引用的变量，因此编译失败
}

#[allow(dead_code)]
fn area_covered_lifetimes2() {
    let mut data = vec![1, 2, 3];
    // x 是可变的（通过 mut 声明），因此我们可以修改 x 指向的内容
    let mut x = &data[0];

    println!("{}", x); // 最后一次使用这个引用
    data.push(4); // OK
    x = &data[3]; // x 在这里借用了新的变量
    println!("{}", x);
}
