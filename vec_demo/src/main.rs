fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1,2,3];

    let mut v3 = Vec::new();
    v3.push(1);
    v3.push(2);
    v3.push(3);

    let third1 = &v3[2];
    println!("third element is {third1}");

    let third2 = v3.get(2);
    match third2 {
        Some(third) => println!("this third is {}", third),
        None => {},
    }

    println!("third new is {}", v3[1]);
}
