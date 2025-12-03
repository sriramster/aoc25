use std::fs::File;
use std::io::{self, BufRead, Write};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = match env::args().nth(1) {
        Some(p) => p,
        _ => unimplemented!(),
    };
    
    let mut pos = 50_i32;
    let mut cnt = 0_i32;

    for line in io::BufReader::new(File::open(path)?).lines() {
        let line = line?;
        let step = line[1..].parse::<i32>().unwrap_or_else(|_| unimplemented!());
        pos = match line.chars().next().unwrap() {
            'L' => pos - (step % 100),
            'R' => pos + step,
            _   => unimplemented!(),
        };
        pos = if pos < 0 { pos + 100 } else { pos % 100 };
        cnt += (pos == 0) as i32;
        println!("{pos}");
    }

    println!("{cnt}");
    Ok(())
}
