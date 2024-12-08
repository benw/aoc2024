const W: usize = 130;
const H: usize = 130;

struct Grid {
    rows: [[u8; W]; H],
}

impl Grid {
    fn new() -> Self {
        Grid { rows: [[0; W]; H] }
    }

    fn get_char(&self, x: i32, y: i32) -> u8 {
        if x >= 0 && x < W as i32 && y >=0 && y < H as i32 {
            self.rows[y as usize][x as usize]
        } else {
            0
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    //let input = include_str!("sample.txt");

    let mut gx = 0;
    let mut gy = 0;

    let mut grid = Grid::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid.rows[y][x] = ch as u8;
            if ch == '^' {
                gx = x as i32;
                gy = y as i32;
            }
        }
    }

    let mut dx = 0;
    let mut dy = -1;
    loop {
        grid.rows[gy as usize][gx as usize] = b'X';
        let next_gx = gx + dx;
        let next_gy = gy + dy;
        let ch = grid.get_char(next_gx, next_gy);
        match ch {
            b'.' | b'X' => {                
                gx += dx;
                gy += dy;
            }
            b'#' => {
                // turn right
                let old_dx = dx;
                dx = -dy;
                dy = old_dx;
            }
            0 => break,
            _ => panic!("unexpected char '{}'", ch as char),
        }

    }

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
