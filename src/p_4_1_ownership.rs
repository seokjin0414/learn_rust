fn p_4_1_ownership() {
    let mut s = String::from("hhhhh");
    s.push_str(", world!");
    println!("string = {s}");

    let x =5;
    let y =5;
    println!("x = {x}, y = {y}");
    // 둘은 다르다
    let s1 = String::from("asd");
    let s2 = s1;
    // s1이 더이상 유효하지 않다고 간주
    // println!("{}, world!", s1);

    let s1 = String::from("asd");
    let s2 = s1.clone();
    println!("s1 == {s1}, s2 == {s2}");


    let s = String::from("hello");  // s가 스코프 안으로 들어왔습니다.

    takes_ownership(s);             // s의 값이 함수 안으로 이동했습니다...
    // ... 그리고 이제 더이상 유효하지 않습니다.
    let x = 5;                      // x가 스코프 안으로 들어왔습니다.

    makes_copy(x);                  // x가 함수 안으로 이동했습니다만,
    // i32는 Copy가 되므로, x를 이후에 계속
    // 사용해도 됩니다.
}   // 여기서 x는 스코프 밖으로 나가고, s도 그 후 나갑니다. 하지만 s는 이미 이동되었으므로,
// 별다른 일이 발생하지 않습니다.

fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어왔습니다.
    println!("{}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출됩니다. 메모리는
// 해제되었습니다.

fn makes_copy(some_integer: i32) { // some_integer이 스코프 안으로 들어왔습니다.
    println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어났습니다. 별다른 일은 발생하지 않습니다.
