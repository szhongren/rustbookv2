extern crate rand;

use std::io;

#[allow(dead_code)]
pub fn guessing_game_2() {
    use self::rand::Rng;
    use std::cmp::Ordering;

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_number);
    loop {
        println!("Please input your guess:");

        let mut guess = String::new(); // static method

        io::stdin() // static method
            .read_line(&mut guess) // read into mut ref
            .expect("Failed to read line"); // expect a Result, print string on error
        // a Result is an enum that can be either a Ok<type> or Err<type>

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        }; // match on the result of parse

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
