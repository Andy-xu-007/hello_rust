fn main() {
    let a = [1, 2, 3];
    let ap = format!("{:p}", &a);
    let a0p = format!("{:p}", &a[0]);

    println!("{ap}");
    println!("{a0p}");

    println!("{}", ap == a0p);

}
