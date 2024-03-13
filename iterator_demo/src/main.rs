fn main() {
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();
    let some_v1 = v1_iter.next();
    match some_v1 {
        Some(v) => println!("{v}"),
        None => {},
    }

    // for ele in v1_iter {
    //     println!("go: {ele}");
    // }

    let total: i32 = v1_iter.sum();
    println!("{:?}", total);

    let v2: Vec<i32> = v1.iter().map(|r| r + 1).collect();
    println!("{:?}", v2);

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        }
    ];
    let shoes_match = shoes_in_size(shoes, 10);
    println!("{:?}", shoes_match);

}


#[derive(PartialEq, Debug)]
struct Shoe {
    size: i32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
    shoes.into_iter().filter(|r| r.size == shoe_size).collect()
}