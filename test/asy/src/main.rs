#![allow(dead_code, unused_variables)]

fn main() {
    println!("Hello, world!");

    let x = foo2();
}

async fn foo1() -> usize {
    println!("hehe");
    0
}

async fn foo2() -> usize {
    println!("1");
    let x = foo1().await;
    println!("2");
    0
}
