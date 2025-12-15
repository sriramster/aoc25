use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).expect("usage: prog <input-file>");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut operands: Vec<String> = vec!();
    let mut lastrowidx = 0; // 0-index
    let mut cols = 0; // 0-index
    
    for line in reader.lines() {
        let line = line?;
        lastrowidx += 1;
        cols = line.split_whitespace().count();
        for c in line.split_whitespace() {
            operands.push(c.to_string());
        }
    }
    lastrowidx = lastrowidx - 1; // kludge
    let mut total = 0;
    
    for i in 0..cols {
        let op: &str = &operands[lastrowidx * cols + i];
        let mut sum: u64 = 0;
        let mut prod: u64 = 1;
        if op == "+" {
            for j in 0..lastrowidx {
                sum += operands[j * cols + i].trim().parse::<u64>().unwrap();
            }
        } else if op == "*" {
            for j in 0..lastrowidx {
                prod *= operands[j * cols + i].trim().parse::<u64>().unwrap();
            }
        }
        
        total += if op == "+" { sum } else { prod }
    }
    println!("Total = {total}");
    
    Ok(())
}
