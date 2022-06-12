use std::{cmp, io};

// https://cses.fi/problemset/task/1069
fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut longest = 1;
    let chars: Vec<char> = line.trim().chars().collect();
    let mut i = 0;
    let mut count = 1;
    while i < chars.len() - 1 {
        if chars[i] != chars[i + 1] {
            count = 1;
        } else {
            count += 1;
        }
        longest = cmp::max(longest, count);
        i += 1;
    }
    println!("{}", longest);
}
