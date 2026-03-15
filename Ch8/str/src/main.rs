fn main() {
    let mut s = String::new();
    s = "Hello world".to_string();

    println!("{s}");

    let mut s = "你好".to_string();
    println!("{s}");

    s.push_str(" world ");
    println!("{s}");

    s.push('!');
    println!("{s}");

    let s1 = "nakiri".to_string();
    let s2 = "daisuki".to_string();
    let s3 = s1 + &s2;
    println!("{s3}");

    let s1 = "emiria".to_string();
    let s2 = "maji".to_string();
    let s3 = "tenshi".to_string();
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let hello = "नमЗрауйе";
    let s = &hello[0..6];
    println!("{s}");

    for c in hello.chars() {
        println!("{c}")
    }

    for c in hello.bytes() {
        println!("{c}")
    }
}
