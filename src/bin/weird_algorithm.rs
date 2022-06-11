use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse::<i64>().unwrap();
    let mut m = n;
    while m != 1 {
        print!("{} ", m);
        if m % 2 == 0 {
            m /= 2;
        } else {
            m = (m * 3) + 1;
        }
    }
    print!("{}", 1)
}
