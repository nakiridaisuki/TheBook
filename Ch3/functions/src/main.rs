fn main() {
    println!("Hello, world!");

    println!("Hi another_func!");
    another_func();

    say_name_and_age('L', 18);

    statement_and_expression();

    let x = five();
    println!("x is {x}");

    let x = plus_one(x);
    println!("x is {x}");
}

fn another_func() {
    println!("Hi main function!");
}

fn say_name_and_age(name: char, age: u32) {
    println!("My name is {name}, {age} years old");
}

fn statement_and_expression() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("y is {y}");
}

fn five() -> u32 {
    5
}

fn plus_one(x: u32) -> u32 {
    x + 1
}
