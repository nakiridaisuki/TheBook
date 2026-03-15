fn main() {
    let v: Vec<i32> = Vec::new();
    println!("{}", v.len());

    let v = vec![1, 2, 3];
    println!("{}", v[0]);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{}", v[0]);

    let mut v = vec![1, 2, 3, 4];
    let third = &v[2];
    println!("Third number is {}", third);

    let fifth = v.get(4);
    match fifth {
        None => println!("Vector don't has fifth number"),
        Some(x) => println!("Fifth number is {}", x),
    }

    // let first = &v[0];
    // v.push(5);
    // println!("Third number is {}", first);

    for x in &v {
        print!("{} ", x);
    }
    println!();

    for x in &mut v {
        *x *= 3;
        print!("{} ", x);
    }
    println!();
}
