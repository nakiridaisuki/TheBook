#[derive(Debug)]
struct Rectangle {
    width: u32,
    high: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.high
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.high > other.high
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            high: size,
        }
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 130,
        high: 70,
    };
    let rect2 = Rectangle {
        width: 200,
        high: 30,
    };
    let rect3 = Rectangle {
        width: 100,
        high: 50,
    };
    let squ1 = Rectangle::square(5);

    println!("Rectangle's area is {} pixel", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("Rectangle is {:?}", squ1);
    println!("Formated rectangle is {:#?}", squ1);
    dbg!(&squ1);
}
