use std::fs;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum State {
    On,
    Off,
}

impl State {
    fn on(&self) -> bool {
        *self == State::On
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Operation {
    Rect,
    RotateRow,
    RotateColumn,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Command {
    operation: Operation,
    a: usize,
    b: usize,
}

impl Command {
    fn from(input: &str) -> Self {
        let operation;
        let a;
        let b;
        if input.contains("rect") {
            operation = Operation::Rect;
            let mut splits = input.split(" ");
            splits.next();
            let mut splits = splits.next().unwrap().split("x");
            a = splits.next().unwrap().parse().unwrap();
            b = splits.next().unwrap().parse().unwrap();
        } else {
            let mut splits = input.split(" ");
            splits.next();
            if splits.next().unwrap() == "row" {
                operation = Operation::RotateRow;
            } else {
                operation = Operation::RotateColumn;
            }
            a = splits
                .next()
                .unwrap()
                .replace("y=", "")
                .replace("x=", "")
                .parse()
                .unwrap();
            splits.next();
            b = splits.next().unwrap().parse().unwrap();
        }
        Command { operation, a, b }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Display {
    grid: [[State; 50]; 6],
}

impl Display {
    fn new() -> Display {
        let grid = [[State::Off; 50]; 6];
        Display { grid }
    }

    fn execute(&mut self, command: Command) {
        match command.operation {
            Operation::Rect => {
                for y in 0..command.b {
                    for x in 0..command.a {
                        self.grid[y][x] = State::On;
                    }
                }
            }
            Operation::RotateRow => {
                let row = command.a;
                for _ in 0..command.b {
                    let old_edge = self.grid[row][49];
                    for x in (0..50).rev() {
                        if x == 0 {
                            self.grid[row][x] = old_edge;
                        } else {
                            self.grid[row][x] = self.grid[row][x - 1];
                        }
                    }
                }
            }
            Operation::RotateColumn => {
                let col = command.a;
                for _ in 0..command.b {
                    let old_edge = self.grid[5][col];
                    for y in (0..6).rev() {
                        if y == 0 {
                            self.grid[y][col] = old_edge;
                        } else {
                            self.grid[y][col] = self.grid[y-1][col];
                        }
                    }
                }
            }
        }
    }

    fn lit(&self) -> usize {
        let mut count = 0;
        for col in self.grid {
            for cell in col {
                if cell.on() {
                    count += 1;
                }
            }
        }
        count
    }

    fn render(&self) {
        for col in self.grid {
            for cell in col {
                if cell.on() {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }
}

fn generate_display(input: &str) -> Display {
    let mut display = Display::new();
    let mut commands = Vec::new();
    for line in input.lines() {
        commands.push(Command::from(line));
    }
    for command in commands {
        display.execute(command);
    }
    display
}

fn part1(input: &str) -> usize {
    generate_display(input).lit()
}

fn part2(input: &str) {
    generate_display(input).render()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    println!("{}", part1(&input));
    part2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(0, part1(&input));
    }
}
