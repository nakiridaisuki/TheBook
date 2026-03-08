fn main() {
    let s1 = String::from("Hello s1");
    let mut s2 = String::from("Hello s2");

    takes_ownershio(s1);
    s2 = takes_ownershio_and_return(s2);

    let x = 2;
    makes_copy(x);

    // println!("{} {} {}", s1, s2, x);
    println!("{} {}", s2, x);
}

fn takes_ownershio(some_string: String) {
    println!("{}", some_string);
}

fn takes_ownershio_and_return(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}

fn makes_copy(some_integer: u32) {
    println!("{}", some_integer);
}
