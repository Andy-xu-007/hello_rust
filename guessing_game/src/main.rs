use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_numner = rand::thread_rng().gen_range(1..=100);
    println!("This secret number is: {secret_numner}");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()  // 如果没有用use引入，也可以使用 std::io::Stdin
        .read_line(&mut guess)
        .expect("failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Plesase type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,  // 不能漏掉逗号
        };
        match guess.cmp(&secret_numner) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            // Ordering::Equal => println!("You Win"), // 最后一行必须有逗号
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }

        println!("You guessed: {guess}");
    }

}

