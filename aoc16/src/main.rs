
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Vect {
    x: i64,
    y: i64,
}

impl std::ops::Add for Vect {
    type Output = Vect;

    fn add(self, rhs: Vect) -> Self::Output {
        Vect {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Dir { E, N, W, S }

impl Dir {
    fn index(self) -> usize {
        match self {
            Dir::E => 0,
            Dir::N => 1,
            Dir::W => 2,
            Dir::S => 3,
        }
    }

    fn vect(self) -> Vect {
        match self {
            Dir::E => Vect { x: 1, y: 0 },
            Dir::N => Vect { x: 0, y: -1 },
            Dir::W => Vect { x: -1, y: 0 },
            Dir::S => Vect { x: 0, y: 1 },
        }
    }

    fn left(self) -> Dir {
        match self {
            Dir::E => Dir::N,
            Dir::N => Dir::W,
            Dir::W => Dir::S,
            Dir::S => Dir::E,
        }
    }

    fn right(self) -> Dir {
        match self {
            Dir::E => Dir::S,
            Dir::N => Dir::E,
            Dir::W => Dir::N,
            Dir::S => Dir::W,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Cell {
    wall: bool,
    memo: [Option<u64>; 4],
}

#[derive(Debug, Clone)]
struct Grid {
    rows: Vec<Vec<Cell>>,
}

impl Grid {
    fn get_mut(&mut self, pos: Vect) -> &mut Cell {
        &mut self.rows[pos.y as usize][pos.x as usize]
    }

    // get the lowest score to the end, starting at pos, facing direction dir
    fn score(&mut self, pos: Vect, dir: Dir) -> u64 {
        let cell = self.get_mut(pos);
        if cell.wall {
            // can't get there this way
            return u64::MAX / 2;
        }
        if let Some(score) = cell.memo[dir.index()] {
            return score;
        }
        // temporary, to avoid looping (BUG this approach produces wrong results)
        cell.memo[dir.index()] = Some(u64::MAX / 2);

        // continue straight, turn left or turn right
        let score = 1 + self.score(pos + dir.vect(), dir);
        let score = score.min(1000 + self.score(pos, dir.left()));
        let score = score.min(1000 + self.score(pos, dir.right()));
        let score = score.min(2000 + self.score(pos, dir.right().right()));

        let cell = self.get_mut(pos);
        cell.memo[dir.index()] = Some(score);
        score
    }
}

fn main() {
    let input = include_str!("input.txt");

    let mut grid = parser::grid(input).unwrap();
    let pos = Vect { x: 1, y: grid.rows.len() as i64 - 2 };
    let dir = Dir::E;
    println!("Part 1: {}", grid.score(pos, dir));
}

peg::parser!{
    grammar parser() for str {
        rule cell() -> Cell
            = "#" { Cell { wall: true, memo: [None; 4] } }
            / "." { Cell { wall: false, memo: [None; 4] } }
            / "S" { Cell { wall: false, memo: [None; 4] } }
            / "E" { Cell { wall: false, memo: [Some(0); 4] } }

        rule row() -> Vec<Cell>
            = cell()+

        rule rows() -> Vec<Vec<Cell>>
            = row()++"\n"

        pub rule grid() -> Grid
            = rows:rows() "\n"* { Grid { rows } }
    }
}
