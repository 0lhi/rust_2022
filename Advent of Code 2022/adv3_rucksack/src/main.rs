use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
struct Group {
    r1: HashSet<char>,
    r2: HashSet<char>,
    r3: HashSet<char>,
}
fn main() {
    let itemlist = fs::read_to_string("List.txt").unwrap();
    let rucksacks: Vec<_> = itemlist.split('\n').collect();
    // _part_one(rucksacks)
    _part_two(rucksacks)
}

fn _part_one(rucksacks: Vec<&str>) {
    // println!("{:?}", rucksacks[2].len());
    // let
    // for rucksack in rucksacks{
    // line_process(rucksack)};
    let mut total_prio = 0;
    for rucksack in rucksacks {
        total_prio += _line_process(rucksack);
    }
    println!("{}", total_prio)
}

fn _part_two(rucksacks: Vec<&str>) {
    let groups: Vec<_> = rucksacks.chunks(3).collect();
    // println!("{:?}", groups);
    let mut total_prio = 0;
    for rucksack in groups {
        let mut g: Group = Group {
            r1: HashSet::new(),
            r2: HashSet::new(),
            r3: HashSet::new(),
        };
        // println!("\n\n{:?}", g);
        for item in rucksack[0].chars() {
            g.r1.insert(item);
        }
        for item in rucksack[1].chars() {
            g.r2.insert(item);
        }
        for item in rucksack[2].chars() {
            g.r3.insert(item);
        }
        // println!("{:?}", g);
        let intersection: HashSet<_> = g.r1.intersection(&g.r2).copied().collect();
        for x in intersection.intersection(&g.r3).copied()
        /* .intersection(&group.r3) */
        {
            let prio = get_priority(x);
            total_prio += get_priority(x) as u32;
            println!("Group Prio is {x} = {prio}. Total Prio is {total_prio}.");
        }
    }
}

fn _line_process(rucksack: &str) -> u32 {
    let halfway = rucksack.len() / 2;
    let left = &rucksack[..halfway];
    let right = &rucksack[halfway..];
    // println!("{} {}", left, right);
    let mut array_zero = [0_u32; 53];
    for c in left.chars() {
        let p = get_priority(c);
        array_zero[p as usize] += 1;
    }
    for c in right.chars() {
        let p = get_priority(c);
        if array_zero[p as usize] > 0 {
            // println!("Something wrong about {}", p);
            return p.into();
        }
    }
    0
}

fn get_priority(c: char) -> u8 {
    let s: u8 = if c.is_uppercase() { 38 } else { 96 };
    c as u8 - s
}
