fn p_4_2_references_and_borrowing() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("length of '{s1}' is {len}.");


    let mut s = String::from("wa");
    change(&mut s);
    println!("{s}");


    let mut s = String::from("hello");
    let r1 = &mut s;
    // 동일 데이터에 2개의 가변 참조자 불가능 data race 방지
    // let r2 = &mut s;
    // 우회하기
    {
        let r2 = &mut s;
    } // r2 goes out of scope here, so we can make a new reference with no problems.

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM 불변 참조자가 있는 동안 가변 참조자를 만드는 것 또한 불가능

    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");
}

fn calculate_length(s: &String) -> usize {  // s는 String의 참조자
    s.len()
}                                           // 여기서 s는 스코프 밖으로 벗어남. 하지만 가리키고 있는 값에 대한 소유권이 없기
// 때문에, 아무런 일도 발생하지 않습니다.

fn change(some_s: &mut String) {
    some_s.push_str(", wo");
}