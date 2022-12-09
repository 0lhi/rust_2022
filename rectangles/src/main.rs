// #[warn(dead_code)]
// use std::sync::mpsc::Receiver;

// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// // fn area(width: u32, height: u32) -> u32 {
// //     width * height
// // }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // fn area(&self) -> u32 {
    //     self.width * self.height
    // }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
        // fn width(&self) -> bool {
        //     self.width > 0
        // }
    }
}

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     if rect1.width() {
//         println!("THe rectangle has a nonzero width; it is {}", rect1.width);
//     } else {
//         println!(
//             "The area of the rectangle is square pixels.",
//             //rect1.area() //area(&rect1)
//         );
//     }
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    let sq = Rectangle::square(3);
    dbg!(sq);
}
