extern crate rand;

use std::io;
use std::cmp::Ordering;
use std::path::Component::ParentDir;
use rand::Rng;

fn main() {
    p_5_3_method();
}

fn p_5_3_method() {
    let rect1 = Rectangle {
        width: 60,
        height: 70,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 65,
        height: 60,
    };

    println!("The area of the rectangle is {} square pixels.", rect1.area());

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    // 같다
    // p1.distance(&p2); 더 명료
    // (&p1).distance(&p2);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(9);
    dbg!(&sq);

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}