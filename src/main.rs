extern crate rand;

use std::io;
use std::cmp::Ordering;
use std::path::Component::ParentDir;
use rand::Rng;
use aggregator::{Summary, Tweet};

fn main() {

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;


    let address = 0x012345usize;
    let r = address as *const i32;


    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }


    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }


    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);



}