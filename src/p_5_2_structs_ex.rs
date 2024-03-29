fn p_5_2_structs_ex() {
    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectangle is {} square pixels.", area(width1, height1));

    let rect1 = (30, 40);
    println!("The area of the rectangle is {} square pixels.", area_2(rect1));

    let rect2 = Rectengle {
        width: 30,
        height: 30,
    };
    println!("The area of the rectangle is {} square pixels.", area_3(&rect2));

    println!("rect1 is {:?}", rect2);
    println!("rect1 is {:#?}", rect2);

    let scale = 2;
    let rect3 = Rectengle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect3);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_3(rectangle: &Rectengle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectengle {
    width: u32,
    height: u32,

}