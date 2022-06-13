// https://cses.fi/problemset/task/1071/
use std::{cmp, io};

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().unwrap();
    for _i in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let coord = line
            .trim()
            .split(" ")
            .map(|e| e.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let y = &coord[0];
        let x = &coord[1];
        let largest = cmp::max(y, x);
        let min = i32::pow(*largest - 1, 2) + 1;
        let max = i32::pow(*largest, 2);
        let top = largest % 2 == 0;
        let value = if top { todo!() } else { todo!() };
        // println!("{}", value);
    }
}
