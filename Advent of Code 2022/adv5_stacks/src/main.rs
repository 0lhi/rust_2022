#[derive(Debug, Clone, Copy)]
struct Move {
    m0ve: usize,
    from: usize,
    to: usize,
}

fn main() {
    let itemlist = std::fs::read_to_string("Input.txt").unwrap();
    // println!("{:?}", itemlist);
    let mut iter = itemlist.split("\n\n");
    let input: [String; 2] = [
        iter.next()
            .unwrap()
            .to_string()
            .split('\n')
            .rev()
            .collect::<Vec<_>>()
            .join("\n"),
        iter.next().unwrap().to_string(),
    ];
    // println!("{}\n\n{}", input[0], input[1]);
    let mut column = 1;
    let mut id = 0;
    let mut stacks: Vec<String> = vec![input[0]
        .clone()
        .split('\n')
        .next()
        .unwrap()
        .replace(' ', "")];
    for _ in 1..=(stacks[0].len()) {
        // println!("=\n");
        for line in input[0].split('\n') {
            let variable = line.chars().nth(column).unwrap_or(' ');
            if variable.is_ascii_digit() {
                id += 1;
                stacks.push("".to_string());
            } else if variable.is_alphabetic() {
                stacks[id].push(variable)
            }
            // println!("{}", variable);
        }
        column += 4;
    }

    // println!("{:?}", stacks);
    // println!("={}=", input[1].lines().count());
    let mut moves: Vec<Move> = vec![];
    for line in input[1].lines() {
        let vc = line.split(' ').collect::<Vec<_>>();
        let m: Move = Move {
            m0ve: vc[1].parse::<usize>().unwrap(),
            from: vc[3].parse::<usize>().unwrap(),
            to: vc[5].parse::<usize>().unwrap(),
        };
        moves.push(m);
    }
    for m in &moves {
        // println!("{}", &stacks[m.from]);
        let invert = format!("{}", &stacks[m.from][stacks[m.from].len() - m.m0ve..]);
        stacks[m.to] = format!(
            "{}{}",
            stacks[m.to], invert /* .chars().rev().collect::<String>() */
        );

        stacks[m.from] = stacks[m.from][..stacks[m.from].len() - m.m0ve].to_string();
        println!("{stacks:?}")
    }
}

// println!("{moves:?}");
// let mut example_from = "HIJABCDEFG".to_owned();
// let mut example_to = "KLMNOP".to_owned();
// let example_move = 2;
// println!("{}", &example_from[0..=example_move]);
// example_to = format!("{}{}", &example_from[0..=example_move], example_to);
// example_from = example_from.replace(&example_from[0..=example_move], "");
// println!("{example_to} + {example_from}")
