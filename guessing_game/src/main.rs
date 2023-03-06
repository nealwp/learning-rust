use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {

    const MAX_NUMBER: u32 = 300;

    println!("Guess the number! It will be between 1 and {MAX_NUMBER}!");

    let secret_number = rand::thread_rng().gen_range(1..=MAX_NUMBER);

    let mut player_1 = String::new();
    let mut player_2 = String::new();
    

    println!("Enter player 1 name:");

    io::stdin()
        .read_line(&mut player_1)
        .expect("Enter a name");

    let player_1 = player_1.trim();
    let mut active_player = &player_1;

    println!("Enter player 2 name:");

    io::stdin()
        .read_line(&mut player_2)
        .expect("Enter a name");

    let player_2 = player_2.trim();

    loop {
        
        if active_player == &player_2 {
            active_player = &player_1;
        } else {
            active_player = &player_2;
        }

        println!("{active_player}'s turn. Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{active_player} guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small..."),
            Ordering::Greater => println!("too big..."),
            Ordering::Equal => {
                println!("{active_player} wins!!!");
                break;
            }
        }
    }
}
