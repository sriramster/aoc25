use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).expect("usage: prog <input-file>");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut data = vec!();

    for line in reader.lines() {
        let line = line?;
        let row: Vec<char> = line.chars().collect();
        data.push(row);
    }

    let b_startidx = match data[0].iter().position(|&x| x == 'S') {
        Some(val) => val,
        None => unimplemented!(),
    };

    let mut beampos = vec![b_startidx];
    let mut splitcnt = 0;
    
    for c in 1..=data.len() - 1 {
        let split_indices: Vec<usize> = data[c].iter()
            .enumerate()
            .filter_map(|(idx, &val)| {
                if val == '^' {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();

        if beampos == split_indices {
            beampos.clear();
            split_indices.iter().for_each(|x| {
                beampos.push(x - 1);
                beampos.push(x + 1);
                beampos.dedup();
            });
        } else {
            let mut tmp = vec!();
            for i in split_indices {
                if !beampos.contains(&i) {
                    tmp.push(i - 1);
                    tmp.push(i + 1);
                }
            }
            if tmp.len() > 0 {
                tmp.dedup();
                beampos.clear();
                beampos = tmp.clone();
            }
        }

        // Silly debugging
        let mut j = &mut data[c];
        for q in &beampos { j[*q] = '|' }
        println!("{:?}", j);
    }
    Ok(())
}
