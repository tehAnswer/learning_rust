use std::io;

fn main() {
    println!("Guess the number I got in mind.");
    println!("-----------");

    let mut guess = String::new();
    let mut console_input = io::stdin();
    console_input.read_line(&mut guess).ok().expect("OMFG guy, you force a panic. OMFG.");

    println!("You said {}", guess);
}
