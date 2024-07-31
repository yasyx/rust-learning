use std::collections::HashMap;
use std::mem::size_of_val;

fn main() {
    let c1 = || println!("hello closure");
    let c2 = |i: i32| println!("hello closure with parameters: {}", i);
    let name = String::from("Zero");
    let name1 = name.clone();
    let mut table = HashMap::new();
    table.insert("key", "value");
    let c3 = || println!("hello: {}", name);
    let c4 = move || println!("hello :{},{:?}", name1, table);
    let name2 = name.clone();
    let c5 = move || {
        let x = 1;
        let name3 = String::from("yasy");
        println!("hello:{}, {:?}, {:?}", x, name2, name3);
    };
    println!(
        "c1:{}, c2: {} ,c3:{}, c4:{}, c5:{}, main: {}",
        size_of_val(&c1),
        size_of_val(&c2),
        size_of_val(&c3),
        size_of_val(&c4),
        size_of_val(&c5),
        size_of_val(&main),
    );
}
