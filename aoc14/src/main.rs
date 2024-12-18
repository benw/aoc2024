
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
struct Robot {
    p: Vect,
    v: Vect,
}

impl Robot {
    fn advance(&mut self, t: i64, space: Vect) {
        for _ in 0..t {
            self.p = self.p + self.v + space;
            self.p.x %= space.x;
            self.p.y %= space.y;
        }
    }
}

// p=0,4 v=3,-3
peg::parser!{
    grammar parser() for str {

        rule num() -> i64
            = n:$("-"? ['0'..='9']+) { n.parse().unwrap() }
        
        rule vect() -> Vect
            = x:num() "," y:num() { Vect { x, y } }

        rule robot() -> Robot
            = "p=" p:vect()  " v=" v:vect() "\n"+ { Robot { p, v } }

        pub rule robots() -> Vec<Robot>
            = robot()*
    }
}

fn main() {
    // let input = include_str!("sample.txt");
    // let space = Vect { x: 11, y: 7 };

    let input = include_str!("input.txt");
    let space = Vect { x: 101, y: 103 };

    let mut robots = parser::robots(input).unwrap();
    for robot in &mut robots {
        robot.advance(100, space);
    }
    let mut count = [0; 4];
    let mid = Vect { x: space.x / 2, y: space.y /  2 };
    for robot in &mut robots {
        if robot.p.y < mid.y {
            if robot.p.x < mid.x {
                count[0] += 1;
            }
            if robot.p.x > mid.x {
                count[1] += 1;
            }
        }
        if robot.p.y > mid.y {
            if robot.p.x < mid.x {
                count[2] += 1;
            }
            if robot.p.x > mid.x {
                count[3] += 1;
            }
        }
    }
    let safety: i64 = count.into_iter().product();
    println!("Part 1: {}", safety);
}

