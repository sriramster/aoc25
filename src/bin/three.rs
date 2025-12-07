use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn largest_subsequence_of_len(digits: &[char], k: usize) -> Option<String> {
    let n = digits.len();
    if k == 0 || n < k {
        return None;
    }

    let mut start = 0;
    let mut result = String::with_capacity(k);

    for pos in 0..k {
        let end = n - (k - pos);
        let mut best = digits[start];
        let mut best_idx = start;

        for i in start..=end {
            let d = digits[i];
            if d > best {
                best = d;
                best_idx = i;
                if best == '9' { break; }
            }
        }

        result.push(best);
        start = best_idx + 1;
    }

    Some(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).expect("usage: prog <input-file>");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let joltlen = 12_usize;
    let mut sum: u128 = 0;

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }

        let chars: Vec<char> = trimmed.chars().collect();

        match largest_subsequence_of_len(&chars, joltlen) {
            Some(num_str) => {
                let value: u64 = num_str.parse().expect("failed to parse digits to u64");
                println!("line: {}, best: {}, value: {}", trimmed, num_str, value);
                sum += value as u128;
            }
            None => {
                println!("line too short for length {}: {}", joltlen, trimmed);
            }
        }
    }

    println!("sum = {}", sum);
    Ok(())
}
