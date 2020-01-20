// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)

macro_rules! my_macro {
    () => {
        println!("This is vagmi's macro");
    };
}
fn main() {
    my_macro!();
}
