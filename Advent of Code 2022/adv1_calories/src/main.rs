// 1. Make Program read TXT file
// 2. Create Vector collecting Strings from number-rows split by two Newlines
// 3. Create Elf Struct with Elf-ID and Calory-Count (both i32) that
//    are converted from Tuple-Strings
// 4. Find Maximum Calory-Count
// 5. Turn Calory-Count and corresponding Elf-ID into output

// use std::env;
use std::fs;

struct Elf {
    id: i32,
    calories: i32,
}

fn main() {
    // let content = fs::read_to_string("Item.txt").unwrap();
    let content = fs::read_to_string("Items.txt").unwrap();
    let lines: Vec<_> = content.split("\n\n").collect();
    let mut totals: Vec<i32> = vec![];
    for foods in lines {
        // println!("\n{}", foods.replace("\n", "+"));
        let meals: Vec<_> = foods.split("\n").collect();
        let mut sum = 0;
        for calories in meals {
            sum = sum + calories.clone().parse::<i32>().unwrap();
            // println!("t:{} c:{}", sum, calories);
        }
        totals.push(sum);
    }
    // println!("{:?}", totals);
    let mut id = 0;
    let mut highestsum = 0;
    let mut highestid = 0;
    for elves in totals {
        id += 1;
        let elf = Elf {
            id: (id),
            calories: (elves),
        };
        if elf.calories > highestsum {
            highestsum = elf.calories;
            highestid = elf.id
        }
    }
    println!(
        "The Elf with ID {} has the most Calories with {}.",
        highestid, highestsum
    )
}
