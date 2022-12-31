use std::env;
use std::fs;
// use std::io;

// Predict the Outcome of Games when following Strategy Guide
// Calculate Score. Things to consider:
//
// 1. A,B,C are Opponent Moves. X,Y,Z are my Moves.
// 2. A/X = Rock | B/Y = Paper | C/Z = Scissors
// 3. Shape adds to Score: Rock=1, Paper=2, Scissors=3
// 4. Outcome adds to Score: Loss=0, Draw=3, Win=6
#[derive(Debug, Clone, Copy)]

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn get_move(var: &str) -> Move {
        match var {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!(),
        }
    }
}
#[derive(Debug)]
struct Round {
    opponent: Move,
    response: Move,
}

impl Round {
    fn get_score(&self) -> i32 {
        self.get_mscore() + self.get_oscore()
    }

    fn get_mscore(&self) -> i32 {
        match self.response {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn get_oscore(&self) -> i32 {
        match (self.opponent, self.response) {
            (Move::Rock, Move::Paper) => 6,
            (Move::Rock, Move::Scissors) => 0,
            (Move::Scissors, Move::Paper) => 0,
            (Move::Scissors, Move::Rock) => 6,
            (Move::Paper, Move::Rock) => 0,
            (Move::Paper, Move::Scissors) => 6,
            _ => 3, //Thanks to Deavid for this amazing solution :-)
        }
    }
}

// #[derive(Debug)]
// struct Game {
//     r1: Round,
//     r2: Round,
//     r3: Round,
// }

#[derive(Debug)]
struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn total_score(&self) -> i32 {
        let mut total = 0;
        for round in &self.rounds {
            total += round.get_score()
        }
        total
    }
}

fn main() {
    let argument = env::args().nth(1).unwrap_or_default();
    // println!("{}", argument);
    if argument == "-c" {
        convert()
    } else {
        guide()
    }
}

fn convert() {
    // let guide = fs::read_to_string("Example.txt").unwrap();
    // println!("{}", guide);
    // let moves: Vec<_> = guide.split('\n').collect();

    let content = fs::read_to_string("Guide.txt").unwrap();
    let moves: Vec<_> = content.split('\n').collect();
    // println!("{:?}", moves);

    let mut game: Game = Game { rounds: vec![] };
    for rounds in moves {
        let opponent = Move::get_move(&rounds[0..1]);
        let t_response = &rounds[2..3];
        let response = match (opponent, t_response) {
            (Move::Rock, "X") => Move::Scissors,
            (Move::Rock, "Z") => Move::Paper,
            (Move::Scissors, "X") => Move::Paper,
            (Move::Scissors, "Z") => Move::Rock,
            (Move::Paper, "X") => Move::Rock,
            (Move::Paper, "Z") => Move::Scissors,
            _ => opponent, //Thanks to Deavid for this amazing solution :-)
        };
        let round = Round { opponent, response };
        game.rounds.push(round)
    }
    println!("Total Score is {}.", game.total_score());
}

fn guide() {
    let content = fs::read_to_string("Example.txt").unwrap();
    let moves: Vec<_> = content.split('\n').collect();
    // println!("{:?}", moves);

    let mut game: Game = Game { rounds: vec![] };
    for rounds in moves {
        let round = Round {
            opponent: Move::get_move(&rounds[0..1]),
            response: Move::get_move(&rounds[2..3]),
        };
        game.rounds.push(round)
    }
    // let game: Game = Game {
    //     r1: {
    //         Round {
    //             opponent: Move::get_move(moves[0]),
    //             response: Move::get_move(moves[1]),
    //         }
    //     },
    //     r2: {
    //         Round {
    //             opponent: Move::get_move(moves[2]),
    //             response: Move::get_move(moves[3]),
    //         }
    //     },
    //     r3: {
    //         Round {
    //             opponent: Move::get_move(moves[4]),
    //             response: Move::get_move(moves[5]),
    //         }
    //     },
    // };
    // println!("{:?}", game);
    println!("Total Score is {}.", game.total_score());
    // println!("{}", moves.len() / 2)
}

// let rounds: Vec<_> = content.split("\n").collect();
// let mut round: Vec<i32> = vec![];
// for game in rounds {
//     let games: Vec<_> = game.split(" ").collect().replace("";
//     let mut sum = 0;
//     for result in games {
//         sum = sum + result.clone().parse::<i32>().unwrap();
//     }
//     round.push(sum);
// }
// println!("{:?}", round);
