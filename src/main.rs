use std::io;

fn main() {
    println!("Hello, world!");
    p_2();
}

fn p_2() {
    println!("번호 추측하기!");
    println!("예측 번호를 입력하세요.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("your guessed : {}", guess);
}

