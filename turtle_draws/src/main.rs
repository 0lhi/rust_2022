use turtle::Turtle;

fn is_not_digit(d: char) -> bool {
    !matches!(d, '0'..='9')
}

fn main() {
    let mut turtle = Turtle::new();
    let mut speed: i32 = (format!("{}", turtle.speed()))
        .replace(is_not_digit, "")
        .parse()
        .unwrap();

    //    let mut speed = turtle.speed().parse::<i32>.unwrap();
    // {
    //     println!("{}", speed.0); //Private field, cannot be printed.
    // };
    //    turtle.drawing_mut().set_background_color("black");
    turtle.drawing_mut().set_background_color("#ff75ff");
    turtle.set_pen_color("white");
    turtle.forward(10.0);

    let distance = 200.0;
    let angle = 142.0;
    let loops = 30;
    if loops > 10 {
        speed *= loops / 10;
    };

    if speed > 25 {
        turtle.set_speed("instant");
        println!("Speed: Instant.");
    } else {
        turtle.set_speed(speed);
        println!("Speed: {}", speed);
    }
    {
        println!("Distance: {}", distance);
    };
    {
        println!("Angle: {}", angle);
    };
    {
        println!("Loops: {}", loops);
    };

    //    let mut loopcount = 0;

    // loop {
    //     turtle.forward(distance);
    //     turtle.right(angle);
    //     if loopcount == loops {
    //         break;
    //     } else {
    //         loopcount += 1;
    //     }
    // }

    for _ in 0..=loops {
        turtle.forward(distance);
        turtle.right(angle);
    }
}

// fn is_not_numeric(c: char) -> bool {
//     match c {
//         '0'..='9' => false,
//         _ => true,
//     }
// }

// fn is_not_numeric(c: char) -> bool {
//     if c >= '0' && c <= '9' {
//         return false;
//     } else {
//         return true;
//     }
