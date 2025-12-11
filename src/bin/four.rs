use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).expect("usage: prog <input-file>");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut chars: Vec<char> = vec!();
    let index_kernel:Vec<(i32, i32)> = vec![
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),  (0, 0),  (0, 1),
        (1, -1),  (1, 0),  (1, 1)
    ];

    let mut cols = 0;
    let mut rows = 0;
    
    for line in reader.lines() {
        let line = line?;
        // Safe to overwwrite
        cols = line.len();
        rows = rows + 1;
        let trimmer = line.trim();
        if trimmer.is_empty() { continue; }
        chars.extend(line.chars());
    }

    for (idx, c) in chars.iter().enumerate() {
        let i = idx / cols;
        let j = idx % cols;

        let result: Vec<(i32, i32)> = index_kernel
            .iter()
            .map(|(x, y)| (x + i as i32, y + j as i32))
            .collect();

        println!("{:?}", result);
        let mut cnt = 0;
        for r in result {
            let idx = r.0 * cols as i32 + r.1;
            if idx >= 0  {
                let idx = idx as usize;
                println!("val = {}, idx = {}", chars[idx], idx);
                if chars[idx] == '@' { cnt = cnt + 1; }
            }
            
        }
        println!("{idx}, {cnt}");
    }
    
    Ok(())
}
