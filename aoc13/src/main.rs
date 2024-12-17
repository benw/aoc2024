
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

impl std::ops::Mul<i64> for Vect {
    type Output = Vect;

    fn mul(self, rhs: i64) -> Self::Output {
        Vect {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Machine {
    a: Vect,
    b: Vect,
    p: Vect,
}

fn cost(sln: Option<(i64, i64)>) -> Option<i64> {
    sln.map(|(i, j)| {
        i * 3 + j
    })
}

impl Machine {
    fn solve(&self) -> Option<(i64, i64)> {
        self.inner(self.p)
    }

    fn solve_part2(&self) -> Option<(i64, i64)> {
        self.inner(self.p + Vect { x: 10000000000000, y: 10000000000000 })
    }

    fn inner(&self, p: Vect) -> Option<(i64, i64)> {
        let a = self.a;
        let b = self.b;

        // px = i * ax + j * bx
        // py = i * ay + j * by

        // i*ax = px - j*bx
        // i = px/ax - j*bx/ax

        // py = (px/ax - j*bx/ax) * ay + j*by
        // py = px*ay/ax - j*bx*ay/ax + j*by
        // py = px*ay/ax + j*(by - bx*ay/ax)
        // py - px*ay/ax = j*(by - bx*ay/ax)
        // (py - px*ay/ax)/(by - bx*ay/ax) = j

        let j = (a.x * p.y - p.x * a.y) / (a.x * b.y - b.x * a.y);
        let i = (p.x - j * b.x) / a.x;

        if a * i + b * j == p {
            Some((i, j))
        } else {
            None
        }
    }
}

fn main() {
    let input = include_str!("input.txt");

    let machines = parser::machines(input).unwrap();
    let mut total = 0;
    let mut total_part2 = 0;
    for machine in &machines {
        if let Some(tokens) = cost(machine.solve()) {
            total += tokens;
        }
        if let Some(tokens) = cost(machine.solve_part2()) {
            total_part2 += tokens;
        }
    }
    println!("Part 1: {}", total);
    println!("Part 2: {}", total_part2);
}

peg::parser!{
    grammar parser() for str {

        rule num() -> i64
            = n:$(['0'..='9']+) { n.parse().unwrap() }
        
        rule button() -> Vect
            = "Button " ['A' | 'B'] ": X+" x:num() ", Y+" y:num() { Vect { x, y } }

        rule prize() -> Vect
            = "Prize: X=" x:num() ", Y=" y:num() { Vect { x, y } }

        rule machine() -> Machine
            = a:button() "\n" b:button() "\n" p:prize() "\n"+ { Machine { a, b, p } }

        pub rule machines() -> Vec<Machine>
            = machine()*
    }
}
