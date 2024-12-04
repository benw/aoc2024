use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let mut words = line.split_ascii_whitespace();
        let a: u32 = words.next().unwrap().parse().unwrap();
        let b: u32 = words.next().unwrap().parse().unwrap();
        left.push(a);
        right.push(b);
    }
    left.sort();
    right.sort();
    let dist1: u32 = left.iter().zip(right.iter()).map(|(a, b)| a.max(b) - a.min(b)).sum();
    println!("Part one: {:?}", dist1);

    let mut rmap: HashMap<u32, u32> = HashMap::new();
    for &b in &right {
        *(rmap.entry(b).or_default()) += 1;
    }
    let mut dist2 = 0;
    for &a in &left {
        dist2 += a * rmap.get(&a).unwrap_or(&0);
    }
    println!("Part two: {:?}", dist2);
}

