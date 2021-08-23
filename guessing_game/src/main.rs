use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    // infinite loop
    loop {
        println!("Please input your guess.");
    
        // get user input
        let mut guess = String::new(); // mutable string

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // convert user input to an integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,            
        };
        println!("You guessed: {}", guess);
    
        // check if guess is correct
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    

}