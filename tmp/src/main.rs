use core::num;

fn main() {
    let number_list = vec![34, 45, 56, 67];
    let mut largest = &number_list[0];
    
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("{}", largest);
    println!("{}", number_list[0]);


}
