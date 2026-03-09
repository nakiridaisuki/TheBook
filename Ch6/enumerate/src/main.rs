enum People {
    Student { id: String, name: String },
    Teacher(String),
    NPC,
}

impl People {
    fn call(&self) {
        println!("Hello");
    }
}

fn main() {
    let nakiri = People::Student {
        id: String::from("1234567"),
        name: String::from("nakiri"),
    };

    nakiri.call();

    let some_num = Some(5);
    let num = 10;
    // println!("some_num add num: {}", some_num + num);
}
