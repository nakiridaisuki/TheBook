use core::num;

fn main() {
    let x = 5;
    if x > 3 {
        println!("x > 3");
    } else {
        println!("x <= 3");
    }

    // Can't auto transform x to bool
    // if x {
    //     println!("x > 0");
    // }
    if x != 0 {
        println!("x > 0");
    }

    let x = 12;
    if x % 4 == 0 {
        println!("x is 4k");
    } else if x % 3 == 0 {
        println!("x is 3k");
    } else if x % 2 == 0 {
        println!("x is 2k");
    } else {
        println!("wtf is x");
    }

    let n = if x > 4 { 0 } else { 1 };
    println!("n is {n}");

    let mut cnt = 0;
    let result = loop {
        cnt += 1;
        if cnt == 10 {
            break cnt * 2;
        }
    };
    println!("result is {result}");

    let mut cnt = 0;
    'cnt_up: loop {
        println!("count = {cnt}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if cnt == 2 {
                break 'cnt_up;
            }
            remaining -= 1;
        }
        cnt += 1;
    }
    println!("End count = {cnt}");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("Fly~~~");

    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("element {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("Fly~~");
}
