use std::fs::File;
use std::io::{self, BufRead};
fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();
    let m: i32 = 100;
    let mut cur: i32 = 50;
    let mut password: i32 = 0;
    for line in lines.map_while(Result::ok){
        let num = &line[1..].parse::<i32>().unwrap();
        if line.starts_with('L') {
            let dist: i32 = if cur == 0 {num - m} else {num - cur};
            if dist >= 0 {
                password += dist / m + 1;
            }
            cur = (cur - (num % m) + m) % m;
        } else {
            let dist: i32 = num - (m - cur);
            if dist >= 0 {
                password += dist / m + 1;
            }
            cur = (cur + num) % m;
        }
    }
    println!("{}", password);
    Ok(())
}

