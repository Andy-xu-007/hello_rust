use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();

    let mut  some_number = Some(5);
    let a = some_number.take();

    match some_number {
        Some(a) => println!("{a}"),
        None => println!("Error")
    }
}