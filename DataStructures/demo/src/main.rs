use rand::Rng;
use rand::thread_rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = thread_rng().gen_range(0..10);
    
    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        println!("You guessed: {}", guess);
    
    
        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("To big!"),
        }
    }
}
