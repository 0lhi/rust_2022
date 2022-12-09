use rand::Rng;
use std::env;
use std::io;
// use std::cmp::Ordering;

fn is_not_alphabetic(c: char) -> bool {
    !c.is_alphabetic()
}

fn main() {
    println!("\nGuess the number!");

    let mut secret_number: i16 = rand::thread_rng().gen_range(1..=500);

    let mut failcounter = 0;
    let mut cheater = false;

    if env::args().len() >= 2 {
        //Let's user define the number to guess. Thanks to DeavidSedice!
        secret_number = match env::args().nth(1).unwrap().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "To use the debug feature, type \"{} NUMBER\" in the Terminal.",
                    env::args().next().unwrap()
                );
                return;
            }
        }
    }

    //println!("The secret number is: {}", secret_number);
    loop {
        println!("\nPlease input your guess (or type \"quit\" to leave):");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).unwrap(); //.expect("Failed to read line");

        let guess: i16 = match guess.trim().parse() {
            //.expect("Please type a number!");
            Ok(num) => num,
            Err(_) => {
                if guess.trim().to_lowercase().replace(
                    is_not_alphabetic, //To be replaced with not_latin.
                    "",
                ) == "quit"
                {
                    println!();
                    println!("Thanks for playing!");
                    println!();
                    break;
                } else if guess.trim() == "\x6f\x6c\x68\x69\x69\x73\x63\x6f\x6f\x6c" {
                    // Cheat Code.
                    println!("The secret number is: {}", secret_number);
                    cheater = true;
                    continue;
                } else {
                    println!("No, no! You have to type a number!");
                    continue;
                }
            }
        };

        println!("You guessed: {}", guess);

        // match guess.cmp(&secret_number) {
        //     Ordering::Less => println!("Too small."),
        //     Ordering::Greater => println!("Too big!"),
        //     Ordering::Equal => {
        //         println!("You win!");
        //         break;
        //     }
        // }
        if guess < secret_number {
            println!("Too small.");
            failcounter += 1;
        }
        if guess > secret_number {
            println!("Too big!");
            failcounter += 1;
        }
        if guess == secret_number {
            println!();
            print!("You win! ");
            if failcounter > 0 {
                print!("You have failed {} times. ", failcounter);
            }
            if cheater && failcounter > 0 {
                print!("And you cheated.");
            }
            if cheater && failcounter == 0 {
                print!("But you cheated.");
            }
            println!();
            println!();
            break;
        }
    }
}
