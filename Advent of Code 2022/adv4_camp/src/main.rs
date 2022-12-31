#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy)]

struct Sections {
    u_min: u32,
    u_MAX: u32,
    d_min: u32,
    d_MAX: u32,
}

impl Sections {
    fn from_vec(x: Vec<&str>) -> Self {
        Sections {
            u_min: x[0].parse().unwrap(),
            u_MAX: x[1].parse().unwrap(),
            d_min: x[2].parse().unwrap(),
            d_MAX: x[3].parse().unwrap(),
        }
    }
}
fn main() {
    let itemlist = std::fs::read_to_string("Pairs.txt")
        .unwrap()
        .replace(",", "-");
    let pairs: Vec<_> = itemlist
        .split('\n')
        .map(|x| x.split("-").collect::<Vec<_>>())
        .map(Sections::from_vec)
        .collect();
    // _part_one()
    let mut totals_one = 0;
    let mut totals_two = 0;
    println!("{:?}\n{:?}\n{:?}", pairs[0], pairs[1], pairs[2]);
    for pair in pairs.clone() {
        if pair.u_min >= pair.d_min && pair.u_MAX <= pair.d_MAX {
            totals_one += 1
        } else if pair.d_min >= pair.u_min && pair.d_MAX <= pair.u_MAX {
            totals_one += 1
        }
    }
    for p in pairs {
        if p.u_min <= p.d_min && p.u_MAX >= p.d_min {
            totals_two += 1
        } else if p.d_min <= p.u_min && p.d_MAX >= p.u_min {
            totals_two += 1
        } else if p.u_min >= p.d_min && p.u_MAX <= p.d_MAX {
            totals_two += 1
        } else if p.d_min >= p.u_min && p.d_MAX <= p.u_MAX {
            totals_two += 1
        }
    }
    println!("{totals_one} // {totals_two}")

    // 123.......
    // .....789
}
