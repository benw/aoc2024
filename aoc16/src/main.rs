use std::cmp::Ordering;
use std::collections::BinaryHeap;

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
struct State {
    pos: Vect,
    dir: Dir,
    score: u64,
}

impl Ord for State {
    fn cmp(&self, rhs: &Self) -> Ordering {
        // BinaryHeap is max-heap, but lowest dist come first, so reverse.
        // In case of a tie we compare pos, dir - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        rhs.score.cmp(&self.score)
            .then_with(|| self.pos.y.cmp(&rhs.pos.y))
            .then_with(|| self.pos.y.cmp(&rhs.pos.x))
            .then_with(|| self.dir.index().cmp(&rhs.dir.index()))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Node {
    score: u64,
    prev: Option<State>, // state immediately prior to arriving here
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Cell {
    wall: bool,
    nodes: [Node; 4], // one for each dir we can be facing
}

#[derive(Debug, Clone)]
struct Grid {
    rows: Vec<Vec<Cell>>,
}

impl Grid {
    fn is_wall(&self, pos: Vect) -> bool {
        self.rows[pos.y as usize][pos.x as usize].wall
    }

    fn get_mut(&mut self, pos: Vect, dir: Dir) -> &mut Node {
        &mut self.rows[pos.y as usize][pos.x as usize].nodes[dir.index()]
    }

    // get the lowest score to the end, starting at start, facing direction dir
    fn search(&mut self, start_pos: Vect, start_dir: Dir, end_pos: Vect) -> Option<u64> {
        // Based on example at https://doc.rust-lang.org/std/collections/binary_heap/index.html
        let mut heap = BinaryHeap::new();
        self.get_mut(start_pos, start_dir).score = 0;
        heap.push(State { pos: start_pos, dir: start_dir, score: 0 });

        while let Some(state) = heap.pop() {
            let State { pos, dir, score } = state;
            if pos == end_pos {
                return Some(score);
            }
            if self.is_wall(pos) {
                continue;
            }
            let node = self.get_mut(pos, dir);
            if score > node.score {
                // We already found a better way here
                continue;
            }
            for next in [
                State { pos: pos + dir.vect(), dir, score: score + 1 },
                State { pos, dir: dir.left(), score: score + 1000 },
                State { pos, dir: dir.right(), score: score + 1000 },
            ] {
                let next_cell = self.get_mut(next.pos, next.dir);
                if next.score < next_cell.score {
                    heap.push(next);
                    next_cell.score = next.score;
                    next_cell.prev = Some(state);
                }
            }
        }
        None
    }
}

fn main() {
    let input = include_str!("input.txt");

    let mut grid = parser::grid(input).unwrap();
    let start = Vect { x: 1, y: grid.rows.len() as i64 - 2 };
    let dir = Dir::E;
    let end = Vect { x: grid.rows[0].len() as i64 - 2, y: 1 };
    let score = grid.search(start, dir, end).unwrap();
    println!("Part 1: {}", score);
}

peg::parser!{
    grammar parser() for str {
        rule cell() -> Cell
            = "#" { Cell { wall: true, nodes: [ Node { score: u64::MAX, prev: None }; 4] } }
            / ['.'|'S'|'E'] { Cell { wall: false, nodes: [ Node { score: u64::MAX, prev: None }; 4] } }

        rule row() -> Vec<Cell>
            = cell()+

        rule rows() -> Vec<Vec<Cell>>
            = row()++"\n"

        pub rule grid() -> Grid
            = rows:rows() "\n"* { Grid { rows } }
    }
}
