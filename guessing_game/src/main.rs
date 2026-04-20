use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Welcome to the Guessing Game!");
    println!("Please input your guess");
    let secret_number=rand::thread_rng().gen_range(1..=100);
    let mut attempts=5;
    println!("You have {attempts} attempts to guess the number (1-100)");
    loop{
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse(){
            Ok(num)=> num,
            Err(_)=>{
                println!("Please enter a valid number");
                continue;
            }
        };
        attempts -=1;
    println!("You guessed: {guess}");
    match guess.cmp(&secret_number){
        Ordering::Less=>{
            println!("Too small!!");
            if secret_number-guess<=5{
                println!("Very close");
            }
            else{
                println!("Far away bro")
            }
        }
        Ordering::Greater=>{
            println!("Too big!");
            if guess-secret_number<=5{
                println!("Very close");
            }
            else{
                println!("Far Away Bro");
            }
        }
        Ordering::Equal=>{
            println!("You win!");
            println!("Score{}", attempts*20);
            break;
        }
    }
    if attempts==0 {
        println!("Game Over! The number was {secret_number}");
        break;
    }
    else{
        println!("Attempts left: {attempts}");
    }
    }   
}
