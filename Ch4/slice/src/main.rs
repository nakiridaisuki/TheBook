fn main() {
    let s = String::from("Hello world!");

    let word = first_word(&s);
    println!("The first word is {word}");

    let word = first_word(&s[6..]);
    println!("The second word is {word}");
}

fn first_word(some_str: &str) -> &str {
    let bytes = some_str.as_bytes();

    for (i, &char) in bytes.iter().enumerate() {
        if char == b' ' {
            return &some_str[..i];
        }
    }
    some_str
}
