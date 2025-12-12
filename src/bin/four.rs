use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn moveout(chars: &Vec<char>, cols: usize) -> (usize, Vec<usize>) {
    let neighbours:Vec<(i32, i32)> = vec![
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),  (0, 0),  (0, 1),
        (1, -1),  (1, 0),  (1, 1)
    ];

    let mut cntroll = 0;
    let mut mutator: Vec<usize> = vec!();
    
    for (idx, c) in chars.iter().enumerate() {
        if chars[idx] == '@' {
            let origin: (usize, usize) = (idx / cols, idx % cols);
            let orig_i32 : (i32, i32) = (origin.0 as i32, origin.1 as i32);
            let result: Vec<(i32, i32)> = neighbours
                .iter()
                .map(|(dx, dy)| (dx + origin.0 as i32, dy + origin.1 as i32))
                .collect();

            let mut cntat = 0;
            
            for r in result {
                if r.0 >= 0 && r.1 >= 0 && r != orig_i32 && r.1 <= cols as i32 - 1 {
                    let cl = cols as i32;
                    let idxx = r.0 * cl + r.1;
                    if idxx >= 0 && idxx < chars.len() as i32 {
                        let idxx = idxx as usize;
                        if chars[idxx] == '@' {
                            cntat = cntat + 1;
                        }
                    }
                }
            }

            if cntat < 4 {
                cntroll = cntroll + 1;               
                mutator.push(idx);
            }
        }
    }
    return (cntroll, mutator);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).expect("usage: prog <input-file>");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut chars: Vec<char> = vec!();
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

    let mut sum: usize = 0;
    loop {
        let (cnt, mutator) = moveout(&chars, cols);
        // mutate here
        for i in mutator { chars[i] = 'x'; }
        if cnt > 0 { sum = sum + cnt }
        else { break }
    }
    
    println!("Total rollouts = {sum}");
    Ok(())
}
