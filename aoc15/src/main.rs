
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

peg::parser!{
    grammar parser() for str {

        rule row() -> Vec<char>
            = [ '#' | 'O' | '.' | '@' ]+

        rule rows() -> Vec<Vec<char>>
             = row()++"\n"
        
        rule move() -> Vect
            = "^" { Vect { x: 0, y: -1 } }
            / "v" { Vect { x: 0, y: 1 } }
            / "<" { Vect { x: -1, y: 0 } }
            / ">" { Vect { x: 1, y: 0 } }
        
        rule moves() -> Vec<Vect>
            = move()++("\n"?)
        
        pub rule doc() -> (Vec<Vec<char>>, Vec<Vect>)
            = rows:rows() "\n\n" moves:moves() "\n"? { (rows, moves) }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    rows: Vec<Vec<char>>,
    robot: Vect,
}

impl Grid {
    fn can_move(&self, pos: Vect, dir: Vect) -> bool {
        let dest = pos + dir;
        match self.rows[dest.y as usize][dest.x as usize] {
            '.' => true,
            '#' => false,
            'O' => self.can_move(dest, dir),
            '[' => {
                if dir.x == 0 {
                    self.can_move(dest, dir) && self.can_move(Vect { x: dest.x + 1, y: dest.y }, dir)
                } else {
                    self.can_move(dest, dir)
                }
            }
            ']' => {
                if dir.x == 0 {
                    self.can_move(dest, dir) && self.can_move(Vect { x: dest.x - 1, y: dest.y }, dir)
                } else {
                    self.can_move(dest, dir)
                }
            }
            c => panic!("Unexpected '{}' at {},{}", c, dest.x, dest.y),
        }
    }

    fn do_move(&mut self, pos: Vect, dir: Vect) {
        let dest = pos + dir;
        match self.rows[dest.y as usize][dest.x as usize] {
            '.' => (),
            '#' => panic!("Cannot move #"),
            'O' => self.do_move(dest, dir),
            '[' => {
                if dir.x == 0 {
                    self.do_move(dest, dir);
                    self.do_move(Vect { x: dest.x + 1, y: dest.y }, dir);
                } else {
                    self.do_move(dest, dir);
                }
            }
            ']' => {
                if dir.x == 0 {
                    self.do_move(dest, dir);
                    self.do_move(Vect { x: dest.x - 1, y: dest.y }, dir);
                } else {
                    self.do_move(dest, dir);
                }
            }
            c => panic!("Unexpected '{}' at {},{}", c, dest.x, dest.y),
        }
        assert_eq!('.', self.rows[dest.y as usize][dest.x as usize]);
        self.rows[dest.y as usize][dest.x as usize] = self.rows[pos.y as usize][pos.x as usize];
        self.rows[pos.y as usize][pos.x as usize] = '.';
    }
    fn try_move(&mut self, pos: Vect, dir: Vect) -> bool {
        let able = self.can_move(pos, dir);
        if able {
            self.do_move(pos, dir);
        }
        able
    }

    fn move_robot(&mut self, dir: Vect) {
        if self.try_move(self.robot, dir) {
            self.robot = self.robot + dir;
        }
    }

    fn _display(&self) {
        for row in self.rows.iter() {
            for c in row {
                print!("{}", c);
            }
            println!("");
        }
    }

    fn sum_gps(&self) -> usize {
        let mut total = 0;
        for (y, row) in self.rows.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if c == 'O' || c == '[' {
                    total += 100 * y + x;
                }
            }
        }
        total
    }

    fn double(&mut self) {
        for row in self.rows.iter_mut() {
            let doubled = double_row(row);
            let _ = std::mem::replace(row, doubled);
        }
        self.robot.x *= 2;
    }
}

fn double_row(row: &[char]) -> Vec<char> {
    let mut doubled = vec![];
    for &c in row {
        match c {
            'O' => {
                doubled.push('[');
                doubled.push(']');
            }
            '@' => {
                doubled.push('@');
                doubled.push('.');
            }
            _ => {
                doubled.push(c);
                doubled.push(c);
            }
        }
    }
    doubled
}


fn main() {
    let input = include_str!("input.txt");

    let mut robot = Vect { x: 0, y: 0 };
    let (rows, moves) = parser::doc(input).unwrap();
    for (y, row) in rows.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '@' {
                robot = Vect { x: x as i64, y: y as i64 };
            }
        }
    }
    let mut grid = Grid { rows, robot };
    let orig_grid = grid.clone();
    for &dir in moves.iter() {
        grid.move_robot(dir);
    }
    println!("Part 1: {}", grid.sum_gps());

    let mut grid = orig_grid;
    grid.double();
    for &dir in moves.iter() {
        grid.move_robot(dir);
    }
    println!("Part 2: {}", grid.sum_gps());
}
