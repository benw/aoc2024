use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    let grid = Grid::new(input);
    println!("Part 1: {}", grid.cost());

}

#[derive(Debug, Clone)]
struct Grid {
    rows: Vec<Vec<char>>
}

impl Grid {
    fn new(input: &str) -> Self {
        Grid {
            rows: input.lines().map(|line| line.chars().collect()).collect()
        }
    }

    fn get(&self, x: i32, y: i32) -> char {
        if y >= 0 && y < self.rows.len() as i32 {
            let row = &self.rows[y as usize];
            if x >= 0 && x < row.len() as i32 {
                return row[x as usize];
            }
        }
        0 as char
    }

    fn cost(mut self) -> u64 {
        let mut total = 0;
        for y in 0..self.rows.len() {
            for x in 0..self.rows[y].len() {
                let c = self.rows[y][x];
                if c != '.' {
                    let mut perim = 0;
                    let mut set = HashSet::new();
                    self.fill(x as i32, y as i32, c, &mut perim, &mut set);
                    total += set.len() as u64 * perim;
                    for (xx, yy) in set {
                        self.rows[yy as usize][xx as usize] = '.';
                    }
                }
            }
        }
        total
    }

    fn fill(&mut self, x: i32, y: i32, c: char, perim: &mut u64, set: &mut HashSet<(i32, i32)>) {
        if set.contains(&(x, y)) {
            return;
        }
        set.insert((x, y));
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if !set.contains(&(x + dx, y + dy)) {
                if self.get(x + dx, y + dy) == c {
                    self.fill(x + dx, y + dy, c, perim, set);
                } else {
                    *perim += 1;
                }
            }
        }
    }
}
