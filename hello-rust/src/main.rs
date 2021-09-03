use ferris_says::say;
use std::io;
use std::cmp::Ordering;
use rand::Rng;


mod flow_control;
use crate::flow_control::loop_demo1;


fn main() {
    // ferris say first.
    ferris_say();

    loop_demo1(3);

    // Sum
    let s = sum(12, 24);
    println!("sum(12, 24) is {}", s);

    // println!("Hello, world!");
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // Err(_) => continue,
            Err(_) => break,
        };

        println!("Your guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn ferris_say() {
    let stdout = io::stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = io::BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

fn sum(x: i32, y: i32) -> i64 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let z = {
        x + y
    };

    // return (z).into();
    z.into()
}
