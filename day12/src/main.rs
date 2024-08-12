use std::fs;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Opcode {
    Cpy,
    Inc,
    Dec,
    Jnz,
}

impl Opcode {
    fn from(input: &str) -> Opcode {
        match input {
            "cpy" => Opcode::Cpy,
            "inc" => Opcode::Inc,
            "dec" => Opcode::Dec,
            "jnz" => Opcode::Jnz,
            _ => panic!("at the disco!")
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum ParameterType {
    Imm,
    Reg,
    Nul,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Parameter {
    p_type: ParameterType,
    imm: isize,
    reg: char,
}

impl Parameter {
    fn from(input: &str) -> Parameter {
        if let Ok(num) = input.parse::<isize>() {
            Parameter{
                p_type: ParameterType::Imm,
                imm: num,
                reg: '0',
            }
        } else {
            Parameter{
                p_type: ParameterType::Reg,
                imm: 0,
                reg: input.chars().next().unwrap(),
            }
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Instruction {
    opcode: Opcode,
    parameters: [Parameter; 2],
}

impl Instruction {
    fn from(input: &str) -> Instruction {
        let mut splits = input.split(" ");
        let opcode = Opcode::from(splits.next().unwrap());
        let p1;
        let p2;
        if [Opcode::Cpy, Opcode::Jnz].contains(&opcode) {
            p1 = Parameter::from(splits.next().unwrap());
            p2 = Parameter::from(splits.next().unwrap());
        } else {
            p1 = Parameter::from(splits.next().unwrap());
            p2 = Parameter {
                p_type: ParameterType::Nul,
                imm: 0,
                reg: '0',
            }
        }
        Instruction {
            opcode,
            parameters: [p1, p2]
        }
    }
}

struct CPU {
    a: isize,
    b: isize,
    c: isize,
    d: isize,
    program: Vec<Instruction>,
}

impl CPU {
    fn from(input: &str) -> CPU {
        let [a, b, c, d] = [0, 0, 0, 0];
        let mut program = Vec::new();
        for line in input.lines() {
            program.push(Instruction::from(line));
        }
        CPU {
            a,
            b,
            c,
            d,
            program,
        }
    }

    fn run(&mut self) {
        let mut pc = 0;
        let end = self.program.len();
        while pc < end {
            match self.program[pc].opcode {
                Opcode::Cpy => {
                    let src = self.program[pc].parameters[0];
                    let val;
                    match src.p_type {
                        ParameterType::Imm => {val = src.imm}
                        ParameterType::Reg => {
                            match src.reg {
                                'a' => val = self.a,
                                'b' => val = self.b,
                                'c' => val = self.c,
                                'd' => val = self.d,
                                _ => panic!("at the disco")
                            }
                        }
                        ParameterType::Nul => panic!("at the disco")
                    }
                    let dst = self.program[pc].parameters[1].reg;
                    match dst {
                        'a' => self.a = val,
                        'b' => self.b = val,
                        'c' => self.c = val,
                        'd' => self.d = val,
                        _ => panic!("at the disco")
                    }
                    pc += 1;
                }
                Opcode::Inc => {
                    match self.program[pc].parameters[0].reg {
                        'a' => self.a += 1,
                        'b' => self.b += 1,
                        'c' => self.c += 1,
                        'd' => self.d += 1,
                        _ => panic!("at the disco")
                    }
                    pc += 1;
                }
                Opcode::Dec => {
                    match self.program[pc].parameters[0].reg {
                        'a' => self.a -= 1,
                        'b' => self.b -= 1,
                        'c' => self.c -= 1,
                        'd' => self.d -= 1,
                        _ => panic!("at the disco")
                    }
                    pc += 1;
                }
                Opcode::Jnz => {
                    let src = self.program[pc].parameters[0];
                    let test;
                    match src.p_type {
                        ParameterType::Imm => {test = src.imm}
                        ParameterType::Reg => {
                            match src.reg {
                                'a' => test = self.a,
                                'b' => test = self.b,
                                'c' => test = self.c,
                                'd' => test = self.d,
                                _ => panic!("at the disco")
                            }
                        }
                        ParameterType::Nul => panic!("at the disco")
                    }
                    let val = self.program[pc].parameters[1].imm;
                    if test != 0 {
                        if val < 0 {
                            pc -= (-1*val) as usize;
                        } else {
                            pc += val as usize;
                        }
                    } else {
                        pc += 1;
                    }
                }
            }
        }
    }
}

fn part1(input: &str) -> isize {
    let mut cpu = CPU::from(input);
    cpu.run();
    cpu.a
}

fn part2(input: &str) -> isize {
    let mut cpu = CPU::from(input);
    cpu.c = 1;
    cpu.run();
    cpu.a
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(42, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(42, part2(&input));
    }
}
