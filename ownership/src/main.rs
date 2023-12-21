fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    
    let x = 5;
    let _y = x;
    println!("x = {x}");

    let s1 = String::from("hello");
    let _s2 = s1.clone(); // 不用clone只是移动，会导致s1失效
    println!("{}, world!", s1);

    let s = String::from("hello");
    takes_ownership(s.clone()); 
    println!("{}", s);  // 上一步s如果不使用clone会报错，s是存在堆上已经被释放失效了
    takes_ownership_ref(&s); // 借用，原参数依然存在，不会被释放
    println!("{}", s);

    let a = 5;
    makes_copy(a); // 不需要使用，该数据存在栈上，被复制了一份
    println!("{}", a);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string 移出作用域并调用 `drop` 方法，占用的内存被释放

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer 移出作用域。没有特殊之处

fn takes_ownership_ref(some_string: &String) {
    // some_string.push_str("world"); // 借用没有所有权，不能修改
    println!("{}", some_string);
}

fn takes_ownership_mut_ref(some_string: &mut String) {
    some_string.push_str("world"); // 可变引用，可以改变值
}
