use std::io;

fn main() {
    println!("Guess the number");

    println!("Please input you guess number:");
    
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failled to read the line");

    println!("You guessed: {}", guess);
}
