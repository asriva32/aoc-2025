use std::fs::{self};
use std::io::{self};
use std::str;



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
            if !v.len().is_multiple_of(2) {
                continue;
            }
            let (l, r) = v.split_at(v.len() / 2);
            if l == r {
                ans += v.parse::<i64>().unwrap();
            }
        }
    }
    println!("{}", ans);
    Ok(())
}