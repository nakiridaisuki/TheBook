use std::{collections::HashMap, io};

fn main() {
    let mut nums = String::new();
    io::stdin().read_line(&mut nums).expect("Read line error");

    let mut cnts = HashMap::new();
    let mut lst = Vec::new();
    for num in nums.split_whitespace() {
        let num: i32 = match num.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        lst.push(num);
        let tmp = cnts.entry(num).or_insert(0);
        *tmp += 1;
    }

    lst.sort();

    let length = lst.len();

    if length % 2 == 0 {
        let ans = lst[length / 2] + lst[length / 2 - 1];
        let ans: f64 = f64::from(ans) / 2.0;
        println!("{ans}");
    } else {
        let ans = lst[length / 2];
        println!("{ans}");
    }

    let mut cnt = 0;
    let mut num = 0;
    for (key, val) in cnts {
        if val > cnt {
            num = key;
            cnt = val;
        }
    }
    println!("{num}");
}
