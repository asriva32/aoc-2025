use std::fs::File;
use std::io::{self, BufRead};
use std::collections::VecDeque;

fn p1(a: &Vec<Vec<char>>) {
    let dirs: Vec<(i32, i32)> = vec![(1, 1), (1, -1), (-1, -1), (-1, 1), (0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut ans = 0;
    let is_valid = |x, y| x >= 0 && x < a.len() as i32 && y >= 0 && y < a[0].len() as i32;
    for i in 0..a.len() {
        for j in 0..a.len() {
            if a[i][j] != '@' {
                continue;
            }
            let mut bad = 0;
            for (x, y) in &dirs {
                let dx = i as i32 + x;
                let dy = j as i32 + y;
                if is_valid(dx, dy) && a[dx as usize][dy as usize] == '@'{
                    bad += 1;
                }
            }
            if bad < 4 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

fn p2(a: &mut Vec<Vec<char>>) {
    let dirs: Vec<(i32, i32)> = vec![(1, 1), (1, -1), (-1, -1), (-1, 1), (0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut vis = vec![vec![false; a[0].len()]; a.len()];
    let mut q = VecDeque::new();
    let is_valid = |x, y| x >= 0 && x < a.len() as i32 && y >= 0 && y < a[0].len() as i32;
    let mut deg: Vec<Vec<i32>> = vec![vec![0; a[0].len()]; a.len()];
    let mut ans: i32 = 0;
    for i in 0..a.len() {
        for j in 0..a.len() {
            if a[i][j] != '@' {
                continue;
            }
            let mut bad = 0;
            for (x, y) in &dirs {
                let dx = (i as i32 + x) as i32;
                let dy = (j as i32 + y) as i32;
                if is_valid(dx, dy) && a[dx as usize][dy as usize] == '@'{
                    bad += 1;
                }
            }
            deg[i][j] = bad;
            if deg[i][j] < 4 {
                q.push_back((i, j));
                vis[i][j] = true;
            }
        }
    }

    while !q.is_empty() {
        let (u, v) = q.pop_back().unwrap();
        ans += 1;
        for (x, y) in &dirs {
            let dx = (u as i32 + x) as i32;
            let dy = (v as i32 + y) as i32;
            if !is_valid(dx, dy) || vis[dx as usize][dy as usize] || a[dx as usize][dy as usize] != '@' {
                continue;
            }
            deg[dx as usize][dy as usize] -= 1;
            if deg[dx as usize][dy as usize] < 4 {
                q.push_back((dx as usize, dy as usize));
                vis[dx as usize][dy as usize] = true;
            }
        }
    }
    println!("{}", ans);
}


fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let lines = io::BufReader::new(f).lines();
    let mut a: Vec<Vec<char>> = Vec::new();
    for line in lines.map_while(Result::ok) {
        a.push(line.chars().collect());
    }
    p1(&a);
    p2(&mut a);
    Ok(())

}