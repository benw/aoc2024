
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

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<char>>,
    robot: Vect,
}

impl Grid {
    fn try_move(&mut self, pos: Vect, dir: Vect) -> bool {
        let dest = pos + dir;
        let able = match self.rows[dest.y as usize][dest.x as usize] {
            '.' => true,
            '#' => false,
            'O' => self.try_move(dest, dir),
            c => panic!("Unexpected '{}' at {},{}", c, dest.x, dest.y),
        };
        if able {
            assert_eq!('.', self.rows[dest.y as usize][dest.x as usize]);
            self.rows[dest.y as usize][dest.x as usize] = self.rows[pos.y as usize][pos.x as usize];
            self.rows[pos.y as usize][pos.x as usize] = '.';
        }
        able
    }

    fn move_robot(&mut self, dir: Vect) {
        if self.try_move(self.robot, dir) {
            self.robot = self.robot + dir;
        }
    }

    fn display(&self) {
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
                if c == 'O' {
                    total += 100 * y + x;
                }
            }
        }
        total
    }
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
    for &dir in moves.iter() {
        grid.move_robot(dir);
    }
    grid.display();
    println!("Part 1: {}", grid.sum_gps());
}