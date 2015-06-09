extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  println!("Guess the number I got in mind.");
  println!("-----------");
  loop {
    let number = rand::thread_rng().gen_range(1, 6);
    let mut guess = String::new();
    let mut console_input = io::stdin();
    console_input.read_line(&mut guess).ok().expect("OMFG guy, you force a panic. OMFG.");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("WTF.");
        continue
      }
    }; 

    println!("You said {} and it was {}", guess, number);
    match guess.cmp(&number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too much!"),
      Ordering::Equal => {
        println!("You got this!");
        break;
      }
    }
  }
}
