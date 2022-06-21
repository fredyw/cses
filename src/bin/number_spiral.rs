// https://cses.fi/problemset/task/1071/
use std::{cmp, io};

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().unwrap();
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let coord = line
            .trim()
            .split(" ")
            .map(|e| e.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let y = &coord[0];
        let x = &coord[1];
        let largest = cmp::max(y, x);
        let min = i64::pow(*largest - 1, 2);
        let max = i64::pow(*largest, 2) + 1;
        let top = largest % 2 == 0;
        let value = if x == largest {
            if top {
                min + y
            } else {
                max - y
            }
        } else {
            if top {
                max - x
            } else {
                min + x
            }
        };
        println!("{}", value);
    }
}
