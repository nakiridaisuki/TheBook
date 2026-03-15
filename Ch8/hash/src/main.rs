use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Yellow".to_string(), 10);
    scores.insert("Blue".to_string(), 20);

    let team_name = "Yellow".to_string();
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.entry("Black".to_string()).or_insert(30);
    scores.entry("Yellow".to_string()).or_insert(50);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
