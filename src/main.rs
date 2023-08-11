use std::io;
use sheepit::sheep_test;

fn main() {
    sheep_test();

    println!("enter some text");
    let input = rpassword::read_password().unwrap();
    println!("input: {input}");
}
