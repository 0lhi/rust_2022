// fn main() {println!("Hello, world!");}
use std::io;

fn add_numbers(number_one: i64, number_two: i64) -> i64 {
    number_one + number_two
}

#[rustfmt::skip]
fn main() {
    println!(
        "Type a number for which Mode you want to choose:

    1 = Example Integer/Decimals
    2 = Count up to 100
    3 = Chess Board
    4 = Graph for a Function
    9999 = Display Source Code
    … = Exit\n"
    );
    let mut mode = String::new();

    io::stdin().read_line(&mut mode).unwrap();
    let mode: u32 = mode.trim().parse().unwrap_or(0);
    match mode {
        1 => {boxes_with_things();} //
        2 => {looping_around();}    //
        3 => {chess_board();}       //
        4 => {loops_again();}       //
//        5 => {dbg!(add_numbers(6, 9));}
        5 => {let_mut_const();}
        9999 => {source_code();}    //
        _ => {println!("See ya!");} //
    }

    // let x = 4;
    // x = 5;
    // println!("x = {}", x);
}

fn boxes_with_things() {
    #[allow(clippy::identity_op)] //Makes cargo clippy ignore the unnecessary " + 3 / 7".
    {
        println!(
            "(2 * (1 + 5) + 3 / 7) / 2) = {}", //integers
            (2 * (1 + 5) + 3 / 7) / 2
        );
    }
    println!(
        "(2.0 * (1.0 + 5.0) + 3.0 / 7.0) = {}", //decimals
        (2.0 * (1.0 + 5.0) + 3.0 / 7.0) / 2.0
    );
}

fn looping_around() {
    println!("\nLet's count to 100! \n");
    for number in 1..=100 {
        println!("Number {}", number)
    }
}

fn chess_board() {
    for row in 1..=8 {
        for column in 1..=8 {
            println!("Row {}, Column {}", row, column);
        }
    }
}

#[rustfmt::skip]
fn loops_again() {
    for y in (-10..10).rev() {
        for x in -30..30_i32 //Force value to be integer;
        {                        //needed for .pow to accept variable.
            let value = x.pow(2) / 20 - 9;
            if value >= y {
                print!(" ");
            } else {
                print!("#");
            }
        }
        println!();
    }
}

fn let_mut_const() {}

fn source_code() {
    println!("\nSource Code about to go here.\n");
}

// fn divide(a: f64, b: f64) -> Result<f64> {
//     if b == 0 {
//         return Err(DivideByZeroError);
//     }
//     return Ok(a / b);
// }
