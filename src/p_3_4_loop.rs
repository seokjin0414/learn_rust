fn p_3_4_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count == {count}");
        let mut remaining = 10;

        loop {
            println!("remaining == {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count == {count}");


    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("end");


    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("value = {}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("the value = {element}");
    }

    for n in (1..4).rev() {
        println!("{n}");
    }
    println!("enddddd");
}