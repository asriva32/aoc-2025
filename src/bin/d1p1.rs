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
            cur = (cur - (num % m) + m) % m;
        } else {
            cur = (cur + num) % m;
        }
        
        if cur == 0 {
            password += 1;
        }
    }
    println!("{}", password);
    Ok(())
}

