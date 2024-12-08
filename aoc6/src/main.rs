const W: usize = 130;
const H: usize = 130;

#[derive(Copy, Clone)]
struct Grid {
    rows: [[u8; W]; H],
    gx: i32,
    gy: i32,
    dx: i32,
    dy: i32,
}

impl Grid {
    fn new() -> Self {
        Grid {
            rows: [[0; W]; H],
            gx: 0,
            gy: 0,
            dx: 0,
            dy: -1,
        }
    }

    fn get_char(&self, x: i32, y: i32) -> u8 {
        if x >= 0 && x < W as i32 && y >=0 && y < H as i32 {
            self.rows[y as usize][x as usize]
        } else {
            0
        }
    }

    fn walk_terminates(&mut self) -> bool {
        loop {
            self.rows[self.gy as usize][self.gx as usize] = b'X';
            let next_gx = self.gx + self.dx;
            let next_gy = self.gy + self.dy;
            let ch = self.get_char(next_gx, next_gy);
            match ch {
                b'.' | b'X' => {
                    self.gx += self.dx;
                    self.gy += self.dy;
                }
                b'#' | b'O' => {
                    // turn right
                    let old_dx = self.dx;
                    self.dx = -self.dy;
                    self.dy = old_dx;
                }
                0 => break true,
                _ => panic!("unexpected char '{}'", ch as char),
            }
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    //let input = include_str!("sample.txt");

    let mut grid = Grid::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid.rows[y][x] = ch as u8;
            if ch == '^' {
                grid.gx = x as i32;
                grid.gy = y as i32;
            }
        }
    }
    assert!(grid.walk_terminates());

    let mut count = 0;
    for row in &grid.rows {
        for &ch in row {
            if ch == b'X' {
                count += 1;
            }
        }
    }

    println!("Part 1: {}", count);
}
