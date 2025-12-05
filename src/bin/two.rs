// 11-22 has two invalid IDs, 11 and 22.
// 95-115 has one invalid ID, 99.
// 998-1012 has one invalid ID, 1010.
// 1188511880-1188511890 has one invalid ID, 1188511885.
// 222220-222224 has one invalid ID, 222222.
// 1698522-1698528 contains no invalid IDs.
// 446443-446449 has one invalid ID, 446446.
// 38593856-38593862 has one invalid ID, 38593859.
// The rest of the ranges contain no invalid IDs.

use std::fs::File;
use std::io::{self, BufRead, Write};
use std::env;

fn check_occurrence(s: &str) -> Option<(String, usize)> {
    let n = s.len();
    for size in (1..=n / 2).rev() {
        if n % size != 0 {
            continue;
        }

        let pat = &s[..size];
        let mut ok = true;

        for i in (0..n).step_by(size) {
            if &s[i..i + size] != pat {
                ok = false;
                break;
            }
        }

        if ok {
            return Some((pat.to_string(), n / size));
        }
    }

    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = match env::args().nth(1) {
        Some(p) => p,
        _ => unimplemented!(),
    };

    let mut invalid:i64 = 0;
    
    for line in io::BufReader::new(File::open(path)?).lines() {
        let line = line?;
        let ranges:  Vec<&str> = line.split(',').collect();
        for range in ranges {
            let r: Vec<&str> = range.split('-').collect();
            let s  = r[0].parse::<i64>()?;
            let e  = r[1].parse::<i64>()?;

            println!("Start {s},  end {e}");
            for i in s..e + 1 {
                // Check for repeated sequence
                let tmp_s = i.to_string();
                let t = format!("{}{}", tmp_s,tmp_s);
                let trimmed = &t[1..t.len() - 1];
                if trimmed.contains(&tmp_s) {
                    match check_occurrence(&tmp_s) {
                        Some((s, num)) => {
                            if num < 3 { println!("{i}, {num}"); invalid = invalid + i; }
                        }
                        None => (),
                    };
                }
            }
            
        }
        println!("{invalid}");
    }

    Ok(())
}
