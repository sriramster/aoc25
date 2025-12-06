use std::fs::File;
use std::io::{self, BufRead, Write};
use std::env;

fn main() -> Result <(), Box<dyn std::error::Error>> {
    let path = match env::args().nth(1) {
        Some(p) => p,
        _ => unimplemented!(),
    };

    let mut sum = 0;
    let mut s = String::new();
    
    for line in io::BufReader::new(File::open(path)?).lines() {
        let line = line?;
        let mut maxnum = 0;
        
        for (idx, c) in line.chars().enumerate() {
            for c1 in line.chars().skip(idx + 1) {
                s.push(c); s.push(c1);
                let num:u32 = s.parse().expect("Failed to convert");
                if num > maxnum { maxnum = num }
                s.clear();
            }
        }
        sum = sum + maxnum;
    }
    println!("{sum}");
    Ok(())
}
