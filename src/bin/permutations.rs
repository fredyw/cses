use std::io;

// https://cses.fi/problemset/task/1070
fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse::<i32>().unwrap();
    if n == 1 {
        println!("1");
    } else if n == 2 || n == 3 {
        println!("NO SOLUTION");
    } else {
        let mut j = 2;
        for _i in 0..n {
            if j == 2 {
                print!("{}", j);
            } else {
                print!(" {}", j);
            }
            j += 2;
            if j > n {
                j = 1;
            }
        }
    }
}
