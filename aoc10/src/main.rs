use std::collections::HashSet;

const W: usize = 43;
const H: usize = 43;

#[derive(Clone, Default)]
struct Cell {
    level: i8,
    score: Option<HashSet<(i32, i32)>>,
}

#[derive(Clone)]
struct Grid {
    rows: [[Cell; W]; H],
}

impl Grid {
    fn new() -> Self {
        let row: [Cell; W] = std::array::from_fn(|_| Cell { level: -1, score: None });
        let rows: [[Cell; W]; H] = std::array::from_fn(|_| row.clone());
        Grid { rows }
    }

    fn get_cell(&mut self, x: i32, y: i32) -> Option<&mut Cell> {
        if x >= 0 && x < W as i32 && y >=0 && y < H as i32 {
            Some(&mut self.rows[y as usize][x as usize])
        } else {
            None
        }
    }

    fn score(&mut self, x: i32, y: i32, level: i8) -> HashSet<(i32, i32)> {
        if let Some(cell) = self.get_cell(x, y) {
            if level != cell.level {
                return HashSet::new();
            }
            if let Some(score) = cell.score.as_ref() {
                return score.clone();
            }
            let mut score = HashSet::new();
            if level == 9 {
                score.insert((x, y));
                return score;
            }
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                score.extend(self.score(x + dx, y + dy, level + 1));
            }
            let cell = self.get_cell(x, y).unwrap();
            cell.score = Some(score.clone());
            score
        } else {
            HashSet::new()
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    //let input = include_str!("sample.txt");

    let mut grid = Grid::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid.rows[y][x].level = ch.to_digit(10).unwrap() as i8;
        }
    }

    let mut total = 0;
    for y in 0..H {
        for x in 0..W {
            total += grid.score(x as i32, y as i32, 0).len();
        }
    }
    println!("Part 1: {}", total);
}
