#[derive(Debug)]
struct Rectangle {
    width: u32,
    high: u32,
}

fn main() {
    let canva = Rectangle {
        width: 130,
        high: 70,
    };

    println!("Rectangle's area is {} pixel", get_area(&canva));

    println!("Rectangle is {:?}", canva);
    println!("Formated rectangle is {:#?}", canva);
    dbg!(&canva);
}

fn get_area(rect: &Rectangle) -> u32 {
    rect.width * rect.high
}
