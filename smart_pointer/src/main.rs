use std::ops::Deref;

use crate::List::{Cons, Nil};
fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;
    let z = Box::new(x);

    // 解引用
    assert_eq!(*y, 5);
    assert_eq!(x, 5);
    assert_eq!(*z, 5);

    let m = MyBox::new(x);
    assert_eq!(5, *m);

    let n = MyBox::new(String::from("rust"));
    hello(&n);

    let c: CustomSmartPointer = CustomSmartPointer {data: String::from("my stuff")};
    let d = CustomSmartPointer {data: String::from("other stuff")};

    drop(c);

    println!("CustomSmartPointer created.");

}
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>  {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(item: &str) {
    println!("hello {item}");
}

struct CustomSmartPointer {
    data: String,
} 

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}
