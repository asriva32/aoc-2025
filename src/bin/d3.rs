use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::cmp::max;

fn p2(file: &File) {
    let lines = BufReader::new(file).lines();
    let mut ans: i64 = 0;
    for line in lines.map_while(Result::ok) {
        let mut cur_idx = 0;
        let mut j = 0;
        let bstr = line.as_bytes();
        let n = bstr.len();
        while cur_idx < 12 {
            let need = 12 - cur_idx;
            let mut mx: i64 = 0;
            let mut next_j = j;
            for k in j..=n-need {
                let val = (bstr[k] - ('0' as u8)) as i64;
                if val > mx {
                    mx = val;
                    next_j = k;
                }
            }
            ans += mx * 10_i64.pow((need - 1) as u32);
            cur_idx += 1;
            j = next_j + 1;
        }
    }
    println!("{}", ans);
}

fn p1(file: &File) {
    let lines = BufReader::new(file).lines();
    let mut ans: i64 = 0;
    for line in lines.map_while(Result::ok) {
        let mut best: i64 = 0;
        let bstr = line.as_bytes();
        for i in 0..bstr.len() {
            for j in (i+1)..line.len() {
                let cur: i64 = 10 * (bstr[i] - '0' as u8) as i64 + (bstr[j] - '0' as u8) as i64;
                best = max(best, cur);
            }
        }
        ans += best;
    }
    println!("{}", ans);
}

fn main() ->io::Result<()> {
    let file = File::open("input.txt")?;
    // p1(&file);
    p2(&file);
    Ok(())
}