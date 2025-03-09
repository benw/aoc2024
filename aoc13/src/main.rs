
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Vect {
    x: u64,
    y: u64,
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

impl std::ops::Mul<u64> for Vect {
    type Output = Vect;

    fn mul(self, rhs: u64) -> Self::Output {
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

impl Machine {
    fn cost(&self) -> Option<u64> {
        let mut min_tokens = None;
        for a in 0..100 {
            for b in 0..100 {
                let p = self.a * a + self.b * b;
                if p == self.p {
                    let tokens = a * 3 + b;
                    if let Some(min) = min_tokens {
                        if tokens < min {
                            min_tokens = Some(tokens);
                        }
                    } else {
                        min_tokens = Some(tokens);
                    }
                }
            }
        }
        min_tokens        
    }
}

fn main() {
    let input = include_str!("input.txt");

    let machines = parser::machines(input).unwrap();
    let mut total = 0;
    for machine in &machines {
        if let Some(tokens) = machine.cost() {
            total += tokens;
        }
    }
    println!("Part 1: {}", total);
}

peg::parser!{
    grammar parser() for str {

        rule num() -> u64
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
