use std::io;

// https://cses.fi/problemset/task/1094
fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().parse::<i32>().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut v: Vec<i64> = line
        .trim()
        .split(" ")
        .map(|e| e.parse::<i64>().unwrap())
        .collect();
    let mut min = 0;
    let mut i = 1;
    while i < v.len() {
        if v[i - 1] > v[i] {
            min += v[i - 1] - v[i];
            v[i] = v[i - 1];
        }
        i += 1;
    }
    println!("{}", min);
}
