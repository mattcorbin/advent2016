use std::collections::HashSet;
use std::fs;
use crate::Direction::*;

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    West,
    South,
    East
}

fn turn(current: Direction, turn: char) -> Direction {
    match turn {
        'L' => {
            match current {
                North => West,
                West => South,
                South => East,
                East => North,
            }
        }
        'R' => {
            match current {
                North => East,
                West => North,
                South => West,
                East => South,
            }
        }
        _ => panic!("at the disco!")
    }
}

fn part1(input: &str) -> isize {
    let (mut x, mut y) = (0, 0);
    let mut dir = North;
    for inst in input.split(", ") {
        let chars = inst.chars().collect::<Vec<char>>();
        let t = chars[0];
        let mut distance = String::new();
        for c in chars[1..].iter() {
            distance.push(*c);
        }
        let distance = distance.parse::<isize>().expect("should be a number");
        dir = turn(dir, t);
        match dir {
            North => y-=distance,
            West => x-=distance,
            South => y+=distance,
            East => x+=distance,
        }
    }
    (0 - x).abs() + (0 - y).abs()
}

fn part2(input: &str) -> isize {
    let (mut x, mut y) = (0, 0);
    let mut dir = North;
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    for inst in input.split(", ") {
        let chars = inst.chars().collect::<Vec<char>>();
        let t = chars[0];
        let mut distance = String::new();
        for c in chars[1..].iter() {
            distance.push(*c);
        }
        let distance = distance.parse::<isize>().expect("should be a number");
        dir = turn(dir, t);
        visited.remove(&(x, y));
        match dir {
            North => {
                let start = y-distance;
                let end = y;
                for new_y in (start..=end).rev() {
                    if visited.contains(&(x, new_y)) {
                        return (0 - x).abs() + (0 - new_y).abs();
                    }
                    visited.insert((x, new_y));
                }
                y-=distance
            },
            West => {
                let start = x-distance;
                let end = x;
                for new_x in (start..=end).rev() {
                    if visited.contains(&(new_x, y)) {
                        return (0 - new_x).abs() + (0 - y).abs();
                    }
                    visited.insert((new_x, y));
                }
                x-=distance
            },
            South => {
                let start = y;
                let end = y+distance;
                for new_y in start..=end {
                    if visited.contains(&(x, new_y)) {
                        return (0 - x).abs() + (0 - new_y).abs();
                    }
                    visited.insert((x, new_y));
                }
                y+=distance
            },
            East => {
                let start = x;
                let end = x+distance;
                for new_x in start..=end {
                    if visited.contains(&(new_x, y)) {
                        return (0 - new_x).abs() + (0 - y).abs();
                    }
                    visited.insert((new_x, y));
                }
                x+=distance
            },
        }
    }
    panic!("at the disco!")
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
        let tests = input.lines().collect::<Vec<&str>>();
        assert_eq!(5, part1(tests[0]));
        assert_eq!(2, part1(tests[1]));
        assert_eq!(12, part1(tests[2]));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        let tests = input.lines().collect::<Vec<&str>>();
        assert_eq!(4, part2(tests[3]));
    }
}
