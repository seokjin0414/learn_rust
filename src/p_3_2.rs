fn p_3_2() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32


    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;


    let t = true;
    let f: bool = false; // with explicit type annotation


    let x = 'z';
    let y = 'ℤ';
    let heart_eyed_cat = '😻';


    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("value of b is : {b}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;


    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    // 추가 수정 삭제가 없는경우
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];


    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}