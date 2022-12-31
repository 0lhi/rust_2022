const MARKER: usize = 14; // 4;
const ADD: usize = MARKER - 1;

fn main() {
    use std::collections::HashSet;
    let itemlist = std::fs::read_to_string("Input.txt").unwrap();
    let sequence: Vec<char> = itemlist.chars().collect();
    let mut number = 0;
    for letters in sequence.windows(MARKER) {
        number += 1;
        println!("{:?}", letters);
        let map: HashSet<char> = letters.iter().copied().collect();
        println!("{:?}, {}", number + ADD, map.len());
        if map.len() == MARKER {
            println!("The amount of characters needed is {}", number + ADD);
            break;
        }
    }
}

// fn main() {
//     // Version wish HashMap.
//     use std::collections::HashMap;
//     let itemlist = std::fs::read_to_string("Input.txt").unwrap();
//     let sequence: Vec<char> = itemlist.chars().collect();
//     let mut number = 0;
//     for letters in sequence.windows(MARKER) {
//         number += 1;
//         println!("{:?}", letters);
//         let mut map: HashMap<char, u8> = HashMap::new();
//         for letter in letters.iter().copied() {
//             *map.entry(letter).or_insert(0) += 1;
//         }
//         println!("{:?}, {}", number + ADD, map.len());
//         if map.len() == MARKER {
//             println!("The amount of characters needed is {}", number + ADD);
//             break;
//         }
//     }
// }
