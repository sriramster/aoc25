use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

type Point = (i64, i64, i64);
type Edge = (usize, usize, i128);

const CONNECTION_LIMIT: usize = 1000;

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return;
        }
        if self.size[ra] < self.size[rb] {
            self.parent[ra] = rb;
            self.size[rb] += self.size[ra];
        } else {
            self.parent[rb] = ra;
            self.size[ra] += self.size[rb];
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).expect("usage: prog <input-file>");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Read points
    let mut points: Vec<Point> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let mut it = line.split(',');
        points.push((
            it.next().unwrap().parse()?,
            it.next().unwrap().parse()?,
            it.next().unwrap().parse()?,
        ));
    }

    let n = points.len();

    // Build all unique edges
    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1, z1) = points[i];
            let (x2, y2, z2) = points[j];
            let dx = (x1 - x2) as i128;
            let dy = (y1 - y2) as i128;
            let dz = (z1 - z2) as i128;
            let dist_sq = dx * dx + dy * dy + dz * dz;
            edges.push((i, j, dist_sq));
        }
    }

    edges.sort_by_key(|e| e.2);

    let mut uf = UnionFind::new(n);
    for (count, (a, b, _)) in edges.iter().enumerate() {
        if count == CONNECTION_LIMIT {
            break;
        }
        uf.union(*a, *b);
    }

    let mut component_sizes: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        let root = uf.find(i);
        *component_sizes.entry(root).or_insert(0) += 1;
    }

    let mut sizes: Vec<usize> = component_sizes.values().cloned().collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    println!("{:?}",sizes);
    
    let answer = sizes[0] * sizes[1] * sizes[2];
    println!("AoC Day 8 Answer: {}", answer);

    Ok(())
}

