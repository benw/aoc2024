use std::fmt;
use std::fmt::Display;


#[derive(Copy, Clone, Default)]
struct Cpu {
    a: u64,
    b: u64,
    c: u64,
    ip: usize,
}

impl Cpu {
    fn combo(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid combo operand {}", operand)
        }
    }

    fn run(&mut self, mem: &[u8]) -> Vec<u8> {
        let mut out = vec![];
        while let Some(x) = self.run_to_out(mem) {
            out.push(x);
        }
        out
    }

    fn run_to_out(&mut self, mem: &[u8]) -> Option<u8> {
        loop {
            if self.ip >= mem.len() {
                return None;
            }
            if let Some(x) = self.step(mem) {
                return Some(x);
            }
        }
    }

    fn step(&mut self, mem: &[u8]) -> Option<u8> {
        let opcode = mem[self.ip];
        let operand = mem[self.ip + 1];

        match opcode {
            0 => {
                self.a >>= self.combo(operand);
                self.ip += 2;
            }
            1 => {
                self.b ^= operand as u64;
                self.ip += 2;
            }
            2 => {
                self.b = self.combo(operand) % 8;
                self.ip += 2;
            }
            3 => {
                if self.a == 0 {
                    self.ip += 2;
                } else {
                    self.ip = operand as usize;
                }
            }
            4 => {
                self.b ^= self.c;
                self.ip += 2;
            }
            5 => {
                let x = (self.combo(operand) % 8) as u8;
                self.ip += 2;
                return Some(x)
            }
            6 => {
                self.b = self.a >> self.combo(operand);
                self.ip += 2;
            }
            7 => {
                self.c = self.a >> self.combo(operand);
                self.ip += 2;
            }
            _ => panic!("Invalid opcode {}", opcode)
        }
        None
    }
}

fn outputs_self(a: u64, mem: &[u8]) -> bool {
    let mut cpu: Cpu = Default::default();
    cpu.a = a;
    let mut good = true;
    for &x in mem {
        good = match cpu.run_to_out(&mem) {
            None => false,
            Some(out) => out == x,
        };
        if !good {
            break;
        }
    }
    good && cpu.run_to_out(&mem).is_none()
}

struct CommaSep<I>(I);

impl <T> Display for CommaSep<T>
where
    T: Clone + IntoIterator,
    T::Item: Display,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut it = self.0.clone().into_iter();
        if let Some(first) = it.next() {
            write!(fmt, "{}", first)?;
        }
        while let Some(item) = it.next() {
            write!(fmt, ",{}", item)?;
        }
        Ok(())
    }
}

fn main() {
    let mut cpu = Cpu {
        a: 47719761,
        b: 0,
        c: 0,
        ip: 0,
    };
    let mem = [2,4,1,5,7,5,0,3,4,1,1,6,5,5,3,0];
    let out = cpu.run(&mem);
    println!("Part 1, with cpu.run(): {}", CommaSep(&out));

    print!(  "Part 1, with step():    ");
    let mut a = 47719761;
    while a != 0 {
        let (x, new_a) = step(a);
        print!("{},", x);
        a = new_a;
    }
    println!("");

    let mut found = vec![];
    search(&mem, 0, &mut found);

    for &a in &found {
        assert!(outputs_self(a, &mem));
        println!("Part 2: {}", a);
    }
}

fn search(out: &[u8], a: u64, found: &mut Vec<u64>) {
    if out.len() == 0 {
        found.push(a);
        return;
    }
    let len = out.len() - 1;
    let x = out[len];
    let rest = &out[..len];

    //println!("Searching for a that results in (out {}, a=0o{:o})", x, a);
    for a_bits in 0..=7 {
        let candidate_a = (a << 3) ^ a_bits;
        if step(candidate_a) == (x, a) {
            search(rest, candidate_a, found);
        }
    }
}

/*
2,4,    b = a & 7   dead: b, c
1,5,    b = b ^ 5
7,5,    c = a >> b
0,3,    a = a >> 3
4,1,    b = b ^ c
1,6,    b = b ^ 6
5,5,    out b & 7
3,0     jnz a, 0
*/
fn step(mut a: u64) -> (u8, u64) {
    let mut b = a & 7;
    b = b ^ 5;
    let c = a >> b;
    a = a >> 3;
    b = b ^ c;
    b = b ^ 6;
    ((b & 7) as u8, a)
}
