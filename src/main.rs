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
    println!("The area of the rectangle is {} square pixels.", rect1.area());


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
}