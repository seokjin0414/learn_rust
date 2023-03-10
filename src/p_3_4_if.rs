fn p_3_4_if() {
    let number = 10;

    //  코드의 조건은 반드시 bool
    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }

    if number != 0 {
        println!("not zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("number == {number}");

    // type 다르다
    // let number = if condition {
    //     5
    // } else {
    //     "six"
    // };
    // println!("The value of number is: {number}");
}