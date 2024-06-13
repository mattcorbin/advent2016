use std::collections::HashMap;
use std::fs;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("at the disco"),
        }
    }
}

#[derive(Debug, Clone)]
struct Button {
    id: usize,
    neighbours: HashMap<Direction, usize>,
}

impl Button {
    fn new(id: usize) -> Self {
        let mut neighbours = HashMap::new();
        match id {
            1 => {
                neighbours.insert(Direction::Right, 2);
                neighbours.insert(Direction::Down, 4);
            }
            2 => {
                neighbours.insert(Direction::Right, 3);
                neighbours.insert(Direction::Left, 1);
                neighbours.insert(Direction::Down, 5);
            }
            3 => {
                neighbours.insert(Direction::Left, 2);
                neighbours.insert(Direction::Down, 6);
            }
            4 => {
                neighbours.insert(Direction::Up, 1);
                neighbours.insert(Direction::Right, 5);
                neighbours.insert(Direction::Down, 7);
            }
            5 => {
                neighbours.insert(Direction::Up, 2);
                neighbours.insert(Direction::Right, 6);
                neighbours.insert(Direction::Down, 8);
                neighbours.insert(Direction::Left, 4);
            }
            6 => {
                neighbours.insert(Direction::Up, 3);
                neighbours.insert(Direction::Left, 5);
                neighbours.insert(Direction::Down, 9);
            }
            7 => {
                neighbours.insert(Direction::Right, 8);
                neighbours.insert(Direction::Up, 4);
            }
            8 => {
                neighbours.insert(Direction::Right, 9);
                neighbours.insert(Direction::Left, 7);
                neighbours.insert(Direction::Up, 5);
            }
            9 => {
                neighbours.insert(Direction::Left, 8);
                neighbours.insert(Direction::Up, 6);
            }
            _ => panic!("at the disco"),
        }
        Button { id, neighbours }
    }

    fn advance(&self, direction: Direction) -> usize {
        match self.neighbours.get(&direction) {
            None => self.id,
            Some(id) => *id
        }
    }
}

#[derive(Debug, Clone)]
struct WonkyButton {
    id: char,
    neighbours: HashMap<Direction, char>,
}

impl WonkyButton {
    fn new(id: char) -> Self {
        let mut neighbours = HashMap::new();
        match id {
            '1' => {
                neighbours.insert(Direction::Down, '3');
            }
            '2' => {
                neighbours.insert(Direction::Right, '3');
                neighbours.insert(Direction::Down, '6');
            }
            '3' => {
                neighbours.insert(Direction::Up, '1');
                neighbours.insert(Direction::Down, '7');
                neighbours.insert(Direction::Right, '4');
                neighbours.insert(Direction::Left, '2');
            }
            '4' => {
                neighbours.insert(Direction::Left, '3');
                neighbours.insert(Direction::Down, '8');
            }
            '5' => {
                neighbours.insert(Direction::Right, '6');
            }
            '6' => {
                neighbours.insert(Direction::Up, '2');
                neighbours.insert(Direction::Left, '5');
                neighbours.insert(Direction::Down, 'A');
                neighbours.insert(Direction::Right, '7');

            }
            '7' => {
                neighbours.insert(Direction::Up, '3');
                neighbours.insert(Direction::Left, '6');
                neighbours.insert(Direction::Down, 'B');
                neighbours.insert(Direction::Right, '8');
            }
            '8' => {
                neighbours.insert(Direction::Up, '4');
                neighbours.insert(Direction::Left, '7');
                neighbours.insert(Direction::Down, 'C');
                neighbours.insert(Direction::Right, '9');
            }
            '9' => {
                neighbours.insert(Direction::Left, '8');
            }
            'A' => {
                neighbours.insert(Direction::Right, 'B');
                neighbours.insert(Direction::Up, '6');
            }
            'B' => {
                neighbours.insert(Direction::Up, '7');
                neighbours.insert(Direction::Left, 'A');
                neighbours.insert(Direction::Down, 'D');
                neighbours.insert(Direction::Right, 'C');
            }
            'C' => {
                neighbours.insert(Direction::Left, 'B');
                neighbours.insert(Direction::Up, '8');
            }
            'D' => {
                neighbours.insert(Direction::Up, 'B');
            }
            _ => panic!("at the disco"),
        }
        WonkyButton { id, neighbours }
    }

    fn advance(&self, direction: Direction) -> char {
        match self.neighbours.get(&direction) {
            None => self.id,
            Some(id) => *id
        }
    }
}

#[derive(Debug, Clone)]
struct Keypad {
    buttons: HashMap<usize, Button>,
    wonky_buttons: HashMap<char, WonkyButton>
}

impl Keypad {
    fn new() -> Self {
        let mut buttons = HashMap::new();
        for i in 1usize..=9 {
            buttons.insert(i, Button::new(i));
        }

        let mut wonky_buttons = HashMap::new();
        let chars = ['1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D'];
        for c in chars {
            wonky_buttons.insert(c, WonkyButton::new(c));
        }

        Keypad { buttons, wonky_buttons }
    }

    fn advance(&self, button: usize, direction: Direction) -> usize {
        self.buttons.get(&button).unwrap().advance(direction)
    }

    fn advance_wonky(&self, button: char, direction: Direction) -> char {
        self.wonky_buttons.get(&button).unwrap().advance(direction)
    }
}

fn part1(input: &str) -> usize {
    let keypad = Keypad::new();
    let mut inst = Vec::new();
    for line in input.lines() {
        let mut set = Vec::new();
        for char in line.chars() {
            set.push(Direction::from(char));
        }
        inst.push(set);
    }
    let mut code = Vec::new();
    let mut current = 5;
    for line in inst {
        for dir in line {
            current = keypad.advance(current, dir);
        }
        code.push(current.to_string())
    }
    code.join("").parse::<usize>().unwrap()
}

fn part2(input: &str) -> String {
    let keypad = Keypad::new();
    let mut inst = Vec::new();
    for line in input.lines() {
        let mut set = Vec::new();
        for char in line.chars() {
            set.push(Direction::from(char));
        }
        inst.push(set);
    }
    let mut code = Vec::new();
    let mut current = '5';
    for line in inst {
        for dir in line {
            current = keypad.advance_wonky(current, dir);
        }
        code.push(current.to_string())
    }
    code.join("")
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
        assert_eq!(1985, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!("5DB3", part2(&input));
    }
}
