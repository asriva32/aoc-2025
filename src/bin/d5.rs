use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::BTreeMap;

fn p2(ids: &Vec<(i128, i128)>) {
    let mut mp = BTreeMap::new();
    for (l, r) in ids {
        mp.entry(*l).and_modify(|e| *e += 1).or_insert(1);
        mp.entry(*r + 1).and_modify(|e| *e -= 1).or_insert(-1);
    }
    mp.insert(0, 0);
    let mut ans = 0;
    let mut sum: i128 = 0;
    for (_, v) in &mut mp {
        *v += sum;
        sum = *v;
    }
    let mut mp_iter = mp.into_iter();
    let mut cur_iter = mp_iter.next();
    loop {
        let Some((k, v)) = cur_iter else {
            break;
        };

        let next_iter = mp_iter.next();
        let Some((k2, _)) = next_iter else {
            break;
        };

        ans += if v > 0 {k2 - k} else {0};
        


        cur_iter = next_iter;

    }

    println!("{}", ans);

    
}

fn p1(ids: &Vec<(i128, i128)>, ingredients: &Vec<i128>) {
    let mut ans = 0;
    for id in ingredients {
        for (l, r) in ids {
            if r >= id && id >= l {
                ans += 1;
                break;
            } 
        }
    }
    println!("{}", ans);
}



fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut ids: Vec<(i128, i128)> = vec![];
    let mut ingredients = vec![];
    
    let re1 = Regex::new(r"(\d+)-(\d+)").unwrap();
    let re2 = Regex::new(r"(\d+)").unwrap();

    for line in lines.map_while(Result::ok) {
        
        if let Some(range_cap) = re1.captures(&line) {
            let (x, y) = (range_cap.get(1).unwrap().as_str().parse::<i128>().unwrap(), range_cap.get(2).unwrap().as_str().parse::<i128>().unwrap());
            ids.push((x, y));
        }else if let Some(digit_cap) = re2.captures(&line) {
            ingredients.push(digit_cap.get(0).unwrap().as_str().parse::<i128>().unwrap());
        }
    }

    p1(&ids, &ingredients);
    p2(&ids);
    Ok(())

}