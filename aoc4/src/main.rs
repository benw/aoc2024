
struct Grid {
    rows: [[u8; 140]; 140],
}

impl Grid {
    fn new() -> Self {
        Grid { rows: [[0; 140]; 140] }
    }

    fn get_char(&self, x: i32, y: i32) -> u8 {
        if x >= 0 && x < 140 && y >=0 && y < 140 {
            self.rows[y as usize][x as usize]
        } else {
            0
        }
    }

    fn get_word(&self, x: i32, y: i32, dx: i32, dy: i32) -> [u8; 4] {
        [
            self.get_char(x + dx * 0, y + dy * 0),
            self.get_char(x + dx * 1, y + dy * 1),
            self.get_char(x + dx * 2, y + dy * 2),
            self.get_char(x + dx * 3, y + dy * 3),
        ]
    }

    fn get_cross(&self, x: i32, y: i32) -> [u8; 6] {
        [
            self.get_char(x - 1, y - 1),
            self.get_char(x, y),
            self.get_char(x + 1, y + 1),

            self.get_char(x - 1, y + 1),
            self.get_char(x, y),
            self.get_char(x + 1, y - 1),
        ]
    }
}

fn test_word(word: [u8; 4]) -> u32 {
    if &word == b"XMAS" {
        1
    } else {
        0
    }
}

fn test_cross(word: [u8; 6]) -> bool {
    &word == b"MASMAS"
    || &word == b"SAMSAM"
    || &word == b"SAMMAS"
    || &word == b"MASSAM"
}

fn main() {
    let input = include_str!("input.txt");

    let mut grid = Grid::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid.rows[y][x] = ch as u8;
        }
    }

    let mut count = 0;
    for y in 0..140 {
        for x in 0..140 {
            count += test_word(grid.get_word(x, y, 1, 0));
            count += test_word(grid.get_word(x, y, 1, 1));
            count += test_word(grid.get_word(x, y, 0, 1));
            count += test_word(grid.get_word(x, y, -1, 1));
            count += test_word(grid.get_word(x, y, -1, 0));
            count += test_word(grid.get_word(x, y, -1, -1));
            count += test_word(grid.get_word(x, y, 0, -1));
            count += test_word(grid.get_word(x, y, 1, -1));
        }
    }
    println!("Part 1: {}", count);

    let mut count = 0;
    for y in 0..140 {
        for x in 0..140 {
            if test_cross(grid.get_cross(x, y)) {
                count += 1;
            }
        }
    }
    println!("Part 2: {}", count);
}
