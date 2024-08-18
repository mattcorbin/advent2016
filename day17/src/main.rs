extern crate md5;
extern crate pathfinding;

use std::fs;

use md5::{Digest, Md5};
use pathfinding::prelude::{dijkstra, yen};

const VALID_NODES: [Pos; 16] = [
    Pos::new(0, 0),
    Pos::new(1, 0),
    Pos::new(2, 0),
    Pos::new(3, 0),
    Pos::new(0, 1),
    Pos::new(1, 1),
    Pos::new(2, 1),
    Pos::new(3, 1),
    Pos::new(0, 2),
    Pos::new(1, 2),
    Pos::new(2, 2),
    Pos::new(3, 2),
    Pos::new(0, 3),
    Pos::new(1, 3),
    Pos::new(2, 3),
    Pos::new(3, 3),
];

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_char(self) -> char {
        match self {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
        }
    }
}

fn hash(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input);
    format!("{:02x}", hasher.finalize())
}

fn decode_to_directions(hash: &str) -> Vec<Direction> {
    let mut retval = Vec::new();
    for (i, c) in hash.chars().enumerate() {
        if i >= 4 {
            break;
        }
        if ['b', 'c', 'd', 'e', 'f'].contains(&c) {
            match i {
                0 => retval.push(Direction::Up),
                1 => retval.push(Direction::Down),
                2 => retval.push(Direction::Left),
                3 => retval.push(Direction::Right),
                _ => panic!("at the disco!"),
            }
        }
    }
    retval
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    x: isize,
    y: isize,
    state: String,
}

impl Pos {
    const fn new(x: isize, y: isize) -> Pos {
        Pos {
            x,
            y,
            state: String::new(),
        }
    }

    fn travel(&self, direction: Direction) -> Pos {
        let mut retval = self.clone();
        match direction {
            Direction::Up => retval.y -= 1,
            Direction::Down => retval.y += 1,
            Direction::Left => retval.x -= 1,
            Direction::Right => retval.x += 1,
        }
        retval.state.push(direction.to_char());
        retval
    }

    fn successor(&self, direction: Direction) -> Option<Pos> {
        let next = self.travel(direction);
        if VALID_NODES
            .iter()
            .any(|node| node.x == next.x && node.y == next.y)
        {
            return Some(next);
        }
        None
    }

    fn successors(&self) -> Vec<(Pos, usize)> {
        let mut retval = Vec::new();
        let potential_directions = decode_to_directions(&hash(&self.state));
        for direction in potential_directions {
            if let Some(node) = self.successor(direction) {
                retval.push((node, 1));
            }
        }
        retval
    }
}

fn part1(input: &str) -> String {
    let res = dijkstra(
        &Pos {
            x: 0,
            y: 0,
            state: input.to_string(),
        },
        |p| p.successors(),
        |p| p.x == 3 && p.y == 3,
    )
    .expect("no path from start to end");
    let last = res.0.clone().pop().unwrap();
    last.state.replace(input, "")
}

fn part2(input: &str) -> usize {
    let res = yen(
        &Pos {
            x: 0,
            y: 0,
            state: input.to_string(),
        },
        |p| p.successors(),
        |p| p.x == 3 && p.y == 3,
        100000
    );
    let last = res.clone().pop().unwrap();
    last.1
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
        for line in input.lines() {
            let mut splits = line.split(",");
            let state = splits.next().unwrap();
            let expected = splits.next().unwrap().to_string();
            assert_eq!(expected, part1(state));
        }
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        for line in input.lines() {
            let mut splits = line.split(",");
            let state = splits.next().unwrap();
            splits.next();
            let expected: usize = splits.next().unwrap().parse().unwrap();
            assert_eq!(expected, part2(state));
        }
    }
}
