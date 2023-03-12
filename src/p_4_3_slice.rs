fn p_4_3_slice() {
    let mut s = String::from("haha hoho");
    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!


    let s = String::from("haha hoho");
    let ha = &s[0..4];
    let ha = &s[..4]; // 위와 동일함

    let len = s.len();
    let ho = &s[5..9];
    let ho = &s[5..len]; // 위와 동일함
    let ho = &s[5..]; // 위와 동일함

    let slice = &s[0..len];
    let slice = &s[..]; // 위와 동일함


    let mut s = String::from("haha hoho");
    let word = first_word_2(&s);
    s.clear();
    // error! println!("the first word is: {word}");


    let my_string = String::from("haha hoho");
    // first_word_2가 `String`의 슬라이스로 동작합니다.
    let word = first_word_2(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "haha hoho";
    // first_word_2가 스트링 리터럴의 슬라이스로 동작합니다.
    let word = first_word_2(&my_string_literal[..]);
    // 스트링 리터럴은 *또한* 스트링 슬라이스이기 때문에,
    // 아래 코드도 슬라이스 문법 없이 동작합니다!
    let word = first_word_2(my_string_literal);


    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

}

// 반환값 만으로는 의미가 없다
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// 개선
// fn first_word_2(s: &String) -> &str {
fn first_word_2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}