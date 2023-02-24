
use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number");

    let secrete_number = rand::thread_rng().gen_range(1, 101);

    println!("The secrete number is: {}", secrete_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed read line");       

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };           

        println!("You have guessed: {}", guess);

        match guess.cmp(&secrete_number) {
            Ordering::Less => println!("{}", "To small".red()),
            Ordering::Greater => println!("{}", "To big".red()),
            Ordering::Equal => {
                println!("{}", "You win".green());
                break;
            },
        }
    }
    
}
