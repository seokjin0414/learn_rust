fn p_3_1() {
    let mut x = 5;
    println!("value of x is : {x}");
    x = 6;
    println!("value of x is : {x}");

    const THREE_HOURS_IN_SECONDS:  u32 = 60 * 60 * 3;

    println!("value of THREE_HOURS_IN_SECONDS is : {THREE_HOURS_IN_SECONDS}");


    let y = 5;

    let y = y + 4;

    {
        let y = y * 2;
        println!("value of x is : {y}");
    }

    println!("value of x is : {y}");


    let spaces = "       ";
    let spaces = spaces.len();

    // 타입 오류
    // let mut spaces = "   ";
    // spaces = spaces.len();
}