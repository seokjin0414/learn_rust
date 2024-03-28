fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
#[should_panic]
fn add_works() {
    assert_eq!(add(2,3), 3);
}

fn main() {

}

