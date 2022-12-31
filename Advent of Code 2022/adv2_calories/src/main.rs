// 1. Make Program read TXT file
// 2. Create Vector collecting Strings from number-rows split by two Newlines
// 3. Create Elf Struct with Elf-ID and Calory-Count (both i32) that
//    are converted from Tuple-Strings
// 4. Find Maximum Calory-Count
// 5. Turn Calory-Count and corresponding Elf-ID into output

// use std::env;
use std::fs;

// Not needed yet.
// struct Elf {
//     id: i32,
//     calories: i32,
// }

fn main() {
    // let content = fs::read_to_string("Item.txt").unwrap();
    let content = fs::read_to_string("Items.txt").unwrap();
    let lines: Vec<_> = content.split("\n\n").collect();
    let mut totals: Vec<i32> = vec![];
    for foods in lines {
        // println!("\n{}", foods.replace("\n", "+"));
        let meals: Vec<_> = foods.split('\n').collect();
        let mut sum = 0;
        for calories in meals {
            sum = sum + calories.clone().parse::<i32>().unwrap();
            // println!("t:{} c:{}", sum, calories);
        }
        totals.push(sum);
    }

    // This approach sorts the integers of the Vectors from
    // largest to smallest. Printing out the sum of the first
    // 3 will deliver the answer to the second task.
    //
    // totals.sort();
    // totals.reverse();
    // println!("{:?}", totals[0] + totals[1] + totals[2]);

    // Problem: The Vectors do not include the Elf IDs.
    // Will likely be relevant in future tasks.
    // println!("{:?}", totals);
    // let mut id = 0;

    // This algorhythm will provide the correct solution for
    // the second task, whether the Vector is sorted or not.
    // Less computationally expensive according to Deavid.
    let mut sum0 = 0;
    let mut sum1 = -1;
    let mut sum2 = -2;
    let mut sum3 = -3;
    for elves in totals {
        // id += 1;
        let calories = elves;
        if calories > sum0 {
            sum3 = sum2;
            sum2 = sum1;
            sum1 = sum0;
            sum0 = calories
        } else if calories > sum1 {
            sum3 = sum2;
            sum2 = sum1;
            sum1 = calories
        } else if calories > sum2 {
            sum3 = sum2;
            sum2 = calories
        } else if calories > sum3 {
            sum3 = calories
        }
    }
    println!("Total Calories are {}.", sum0 + sum1 + sum2)
}
