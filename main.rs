use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Word guessing game");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{}", secret_number);
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the File");

    println!("You gues the game {}", guess);
    let x = 5;
    let y = 10;

    if guess > 12.to_string() {
        println!("Hello Wolrd I am Comming with the new technology");
    } else {
        println!("I am Comming but late but hard");
    }

    println!("x = {x} and y + 2 = {}", y + 2);

    match guess.cmp(&secret_number.to_string()) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    loop { // loop ititerate infinite times;

        println!("Word guessing game");
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("{}", secret_number);
        let mut guess = String::new();
 

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the File");
    
        println!("You gues the game {}", guess);
        match guess.cmp(&secret_number.to_string()) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => { 
                println!("Too big!");
                break;
                // break trow out of the loop
        }

            Ordering::Equal => println!("You win!"),
        }
    }
    let mut again_guess  = String::new();
loop {
 
    io::stdin()
    .read_line(&mut again_guess)
    .expect("Faled Error OcCurs");

    let guess_here: u32 = match again_guess.trim().parse() {
        Ok(num) => num, 
        Err(_) =>   continue,
        
    };
}
 
}
