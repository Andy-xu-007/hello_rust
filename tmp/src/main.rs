
fn main() {
    let mut m = 1;
    for i in 12..14 {
        m += 1;
        let n = i - 12..i;
        println!("{:?}", n);
    }

    println!("end .. {m}");
}

