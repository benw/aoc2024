use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    //let input = include_str!("sample.txt");

    let mut ants_by_freq: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let w = input.lines().next().unwrap().len() as i32;
    let h = input.lines().count() as i32;

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                ants_by_freq.entry(ch).or_default().push((x as i32, y as i32));
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for ants in ants_by_freq.values() {
        for &a in ants {
            for &b in ants {
                if a == b {
                    continue;
                }
                for (x, y) in get_antinodes(a, b) {
                    if x >= 0 && x < w && y >= 0 && y < h {
                        antinodes.insert((x, y));
                    }
                }
            }
        }
    }

    println!("Part 1: {}", antinodes.len());
}

fn get_antinodes((ax, ay): (i32, i32), (bx, by): (i32, i32)) -> [(i32, i32); 2] {
    let dx = bx - ax;
    let dy = by - ay;
    [
        (ax - dx, ay - dy),
        (bx + dx, by + dy),
    ]
}
