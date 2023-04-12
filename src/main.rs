extern crate rand;

use std::io;
use std::cmp::Ordering;
use std::path::Component::ParentDir;
use rand::Rng;
use aggregator::{Summary, Tweet};

fn main() {

}

enum Coin {
    찬원,
    오천원,
    만원,
    오만원,
}

fn value_in_number(coin: Coin) -> u8 {
    match coin {
        Coin::찬원 => 1000,
        Coin::오천원 => 5000,
        Coin::만원 => 10000,
        Coin::오만원 => 50000,
    }
}

