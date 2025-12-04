use std::fs::{self};
use std::io::{self};
use std::str;

fn check(v: &String, d: usize) -> bool {
    let first = &v[..d];
    return &first.repeat(v.len() / d) == v;
}

fn main() -> io::Result<()>{
    let file = fs::read_to_string("input.txt")?;
    let inputs:Vec<&str> = file.trim().split(",").collect();
    let mut ans: i64 = 0;
    for input in inputs{
        let ranges: Vec<&str> = input.split("-").collect();
        let left = ranges[0].parse::<i64>().unwrap();
        let right = ranges[1].parse::<i64>().unwrap();
        for idx in left..=right{
            let v = idx.to_string();
            let limit = v.len();
            for d in 1..v.len() {
                if !limit.is_multiple_of(d) {
                    continue;
                }
                
                if check(&v, d) {
                    ans += v.parse::<i64>().unwrap();
                    break;
                }
            }
            
        }
    }
    println!("{}", ans);
    Ok(())
}