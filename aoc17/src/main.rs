

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

fn main() {
    let mut cpu = Cpu {
        a: 47719761,
        b: 0,
        c: 0,
        ip: 0,
    };
    let mem = [2,4,1,5,7,5,0,3,4,1,1,6,5,5,3,0];
    let out = cpu.run(&mem);
    print!("Part 1: {}", out[0]);
    for x in &out[1..] {
        print!(",{}", x);
    }
    println!("");

    for a in 0.. {
        if a % 1000000 == 0 {
            print!("a={}\r", a);
        }
        let mut cpu: Cpu = Default::default();
        cpu.a = a;
        let mut good = true;
        for &x in &mem {
            good = match cpu.run_to_out(&mem) {
                None => false,
                Some(out) => out == x,
            };
            if !good {
                break;
            }
        }
        if !good || cpu.run_to_out(&mem).is_some() {
            continue;
        }
        println!("Part 2: {}", a);
        break;
    }
}
