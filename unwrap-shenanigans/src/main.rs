use rand::Rng;
use std::env;
use std::io;

fn main() {
    let mut mode = 1;
    if env::args().len() >= 2 {
        //Let's user define the number to guess. Thanks to DeavidSedice!
        mode = match env::args().nth(1).unwrap().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "Type a number between 1-6 to determine the Error Management.
                    
                    Mode 1 = gift.expect (Default)
                    Mode 2 = gift.unwrap
                    Mode 3 = gift.unwrap_or
                    Mode 4 = gift.unwrap_or_default
                    Mode 5 = gift.unwrap_or_else
                    Mode 6 = gift.unwrap_unchecked (Let's not.)",
                    //                    env::args().next().unwrap()
                );
                return;
            }
        }
    }
    let console = "xbox".to_string();
    let mut gift = Some(console);
    println!(
        "Rumors have it that your parents bought a console for you!\n\
        But you'll only get it if you were well behaved this year.\n\
        Have you been nice?"
    );

    let mut nice = String::new();
    io::stdin().read_line(&mut nice).unwrap();
    if nice.trim().to_lowercase().replace(
        &['(', ')', ',', '\"', '.', ';', ':', '!', '?', '\''][..],
        "",
    ) == "no"
    {
        gift.take();
    };
    //    let mut gift = None;
    println!(
        "Hooray, you get an {} for christmas!",
        match mode {
            1 => gift.expect("No console for you, kiddo."),
            2 => gift.unwrap(),
            3 => gift.unwrap_or("PlayStation".to_string()),
            4 => gift.unwrap_or_default(),
            5 => gift.unwrap_or_else(|| get_console()),
            // 6 => gift.unwrap_unchecked(),
            _ => todo!(),
        }
    );
}

fn get_console() -> String {
    let giftnumber = rand::thread_rng().gen_range(1..=3);
    match giftnumber {
        1 => "Vii".to_string(),
        2 => "Game Toy".to_string(),
        _ => "Lame Cube".to_string(),
    }
}

/* gift.unwrap

Rumors have it that your parents bought a console for you!
But you'll only get it if you were well behaved this year.
Have you been nice?
No.
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/main.rs:22:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

gift.expect

Rumors have it that your parents bought a console for you!
But you'll only get it if you were well behaved this year.
Have you been nice?
no
thread 'main' panicked at 'No console for you, kiddo.', src/main.rs:22:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace*/
