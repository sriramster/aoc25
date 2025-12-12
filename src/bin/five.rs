use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return vec![];
    }

    ranges.sort_by_key(|r| r.0);

    let mut merged = vec![];
    let mut current = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= current.1 {
            current.1 = current.1.max(end);
        } else {
            merged.push(current);
            current = (start, end);
        }
    }

    merged.push(current);
    merged
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).expect("usage: prog <input-file>");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut ranges: Vec<(u64, u64)> = vec!();
    let mut ids: Vec<u64> = vec!();
    let mut process: bool = false;
    let mut accept:u64 = 0;
    
    for line in reader.lines() {
        let line = line?;
        if line.len() != 0 && process == false {
            let r: Vec<&str> = line.split('-').collect();
            let s = r[0].parse::<u64>()?;
            let e = r[1].parse::<u64>()?;
            ranges.push((s, e));
        } else if line.len() == 0 {
            // skip this line and continue
            process = true;
            continue;
        }

        if process {
            ids.push(line.parse::<u64>()?);
        }
    }

    let merged = merge_ranges(ranges);
    for r  in merged {
        accept += r.1 - r.0 + 1;
    }
    
    println!("Acceptable ids = {accept}");
    
    Ok(())
}
