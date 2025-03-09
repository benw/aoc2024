


struct Cpu {
    a: u64,
    b: u64,
    c: u64,
    ip: usize,
    mem: Vec<u8>,
    out: Vec<u64>,
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

    fn run(&mut self) {
        loop {
            if self.ip >= self.mem.len() {
                break;
            }
            let opcode = self.mem[self.ip];
            let operand = self.mem[self.ip + 1];

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
                    let x = self.combo(operand) % 8;
                    self.out.push(x);
                    self.ip += 2;
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
        }
    }
}

fn main() {
    let mut cpu = Cpu {
        a: 47719761,
        b: 0,
        c: 0,
        ip: 0,
        mem: vec![2,4,1,5,7,5,0,3,4,1,1,6,5,5,3,0],
        out: vec![],
    };
    cpu.run();
    print!("Part 1: {}", cpu.out[0]);
    for x in &cpu.out[1..] {
        print!(",{}", x);
    }
    println!("");
}
