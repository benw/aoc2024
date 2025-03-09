use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    let grid = Grid::new(input);
    let (a, b) = grid.cost();
    println!("Part 1: {}", a);
    println!("Part 2: {}", b);
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

    fn cost(mut self) -> (u64, u64) {
        let mut total = 0;
        let mut total_part2 = 0;
        for y in 0..self.rows.len() {
            for x in 0..self.rows[y].len() {
                let c = self.rows[y][x];
                if c != '.' {
                    let mut perim = 0;
                    let mut set = HashSet::new();
                    let mut fence = [HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new()];
                    self.fill(x as i32, y as i32, c, &mut perim, &mut set, &mut fence);
                    total += set.len() as u64 * perim;
                    total_part2 += set.len() as u64 * sides(fence);
                    for (xx, yy) in set {
                        self.rows[yy as usize][xx as usize] = '.';
                    }
                }
            }
        }
        (total, total_part2)
    }

    fn fill(&mut self, x: i32, y: i32, c: char, perim: &mut u64, set: &mut HashSet<(i32, i32)>, fence: &mut [HashSet<(i32, i32)>; 4]) {
        if set.contains(&(x, y)) {
            return;
        }
        set.insert((x, y));
        for (dir, (dx, dy)) in [(1, 0), (-1, 0), (0, 1), (0, -1)].into_iter().enumerate() {
            if !set.contains(&(x + dx, y + dy)) {
                if self.get(x + dx, y + dy) == c {
                    self.fill(x + dx, y + dy, c, perim, set, fence);
                } else {
                    *perim += 1;
                    fence[dir].insert((x, y));
                }
            }
        }
    }
}

fn sides(mut fence: [HashSet<(i32, i32)>; 4]) -> u64 {
    let mut sides = 0;
    for (dir, (dx, dy)) in [(1, 0), (-1, 0), (0, 1), (0, -1)].into_iter().enumerate() {
        let f = &mut fence[dir];
        while f.len() > 0 {
            sides += 1;
            let &(x, y) = f.iter().next().unwrap();
            let mut xx = x;
            let mut yy = y;
            while f.remove(&(xx, yy)) {
                xx += dy;
                yy += dx;
            }
            let mut xx = x;
            let mut yy = y;
            xx -= dy;
            yy -= dx;
        while f.remove(&(xx, yy)) {
                xx -= dy;
                yy -= dx;
            }
        }
    }
    sides
}
