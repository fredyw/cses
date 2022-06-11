use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse::<i32>().unwrap();
    let mut all_sum = 0;
    let mut i = 1;
    while i <= n {
        all_sum += i;
        i += 1;
    }
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let sum: i32 = line
        .trim()
        .split(" ")
        .map(|e| e.parse::<i32>().unwrap())
        .sum();
    println!("{}", all_sum - sum);
}
