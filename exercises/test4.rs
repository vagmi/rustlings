// test4.rs
// This test covers the sections:
// - Modules
// - Macros
use std::ops::Add;
// Write a macro that passes the test! No hints this time, you can do it!

macro_rules! my_macro {
    ($val: expr) => {
        String::from("Hello ").add($val)
    };
}

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
