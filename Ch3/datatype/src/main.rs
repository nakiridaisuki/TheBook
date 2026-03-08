use std::io;

fn main() {
    let x = 2.10;
    println!("x is {x}");
    let y: f32 = 3.0;
    println!("y is {y}");

    let sum = 5 + 10;
    println!("sum is {sum}");

    let sub = 9.29 - 4.42;
    println!("sub is {sub}");

    let prod = 200 * 39;
    println!("prod is {prod}");

    let quot = 399.2 / 45.0;
    println!("quot is {quot}");

    let rema = 420 % 39;
    println!("rema is {rema}");

    let tup = (500, 3.15, 1);
    let (t0, t1, t2) = tup;
    println!("tup is {t0}, {t1}, {t2}");
    println!("tup is {}, {}, {}", tup.0, tup.1, tup.2);

    let array = [1, 2, 3, 4, 5];
    for a in array {
        println!("array1: {a}");
    }

    let array: [i64; 5] = [1, 2, 3, 4, 5];
    for a in array {
        println!("array2: {a}");
    }

    let array = [3; 5];
    for a in array {
        println!("array3: {a}");
    }

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect_err("read line error");
    let index: usize = index.trim().parse().expect("Input is not a number");
    println!("Valid array index: {}", array[index]);
}
