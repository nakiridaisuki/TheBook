fn main() {
    let mut s = String::from("Reference test");

    test_ref(&s);
    println!("Welcome back {s}");

    let r2 = &mut s;
    println!("{}", r2);
    let r1 = &mut s;
    println!("{} {}", r1, r1);

    test_mut_ref(&mut s);
    println!("Welcome back {s}");
}

fn test_ref(ref_str: &String) {
    println!("Hi {ref_str}");
}

fn test_mut_ref(ref_str: &mut String) {
    println!("Hi {ref_str}");
    ref_str.push_str(" with new data");
}
