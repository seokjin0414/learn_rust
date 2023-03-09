fn p_3_3(x: u32, y: &str) {
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");

    // Statements 와 Expressions 의 차이점 반환값
    // let x = (let y = 6);

    let y = {
        let x= 3;
        x + 1
        //표현식은 종결을 나타내는 세미콜론을 사용 X
    };
    println!("y == {y}");


    let a = one();
    println!("a == {a}");

    let b = plus_one(8);
    println!("b == {b}");



}

fn one() -> i32 {
    1
}

fn plus_one(x: i32) -> i32 {
    x + 1
}