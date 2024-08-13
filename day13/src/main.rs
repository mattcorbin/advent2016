extern crate pathfinding;

use std::fs;

use pathfinding::prelude::dijkstra;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }

    fn neighbours(&self) -> Vec<Pos> {
        let mut retval = Vec::new();
        if self.x == 0 && self.y == 0 {
            retval.push(Pos::new(self.x + 1, self.y));
            retval.push(Pos::new(self.x, self.y + 1));
        } else if self.x == 0 {
            retval.push(Pos::new(self.x + 1, self.y));
            retval.push(Pos::new(self.x, self.y + 1));
            retval.push(Pos::new(self.x, self.y - 1));
        } else if self.y == 0 {
            retval.push(Pos::new(self.x + 1, self.y));
            retval.push(Pos::new(self.x - 1, self.y));
            retval.push(Pos::new(self.x, self.y + 1));
        } else {
            retval.push(Pos::new(self.x + 1, self.y));
            retval.push(Pos::new(self.x - 1, self.y));
            retval.push(Pos::new(self.x, self.y + 1));
            retval.push(Pos::new(self.x, self.y - 1));
        }
        retval
    }

    fn valid(&self, pos: &Pos, maze: &Maze) -> bool {
        let num = pos.x * pos.x + 3 * pos.x + 2 * pos.x * pos.y + pos.y + pos.y * pos.y + maze.magic_number;
        let binary = format!("{num:b}");
        let binary = binary.replace("0", "");
        binary.len() % 2 == 0
    }

    fn successors(&self, maze: &Maze) -> Vec<(Pos, usize)> {
        let mut retval = Vec::new();
        for node in self.neighbours() {
            if self.valid(&node, maze) {
                retval.push((node, 1));
            }
        }
        retval
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Maze {
    magic_number: usize,
    start: Pos,
    goal: Pos,
}

impl Maze {
    fn from(input: &str) -> Maze {
        let start = Pos { x: 1, y: 1 };
        let mut lines = input.lines();
        let goal: Vec<usize> = lines
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();
        let goal = Pos {
            x: goal[0],
            y: goal[1],
        };
        let magic_number = lines.next().unwrap().parse::<usize>().unwrap();
        Maze {
            magic_number,
            start,
            goal,
        }
    }

    fn solve(&self) -> usize {
        let res = dijkstra(&self.start, |p| p.successors(&self), |p| *p == self.goal)
            .expect("no path from start to end");
        res.1
    }

    fn within(&self, max_dist: usize) -> usize {
        let mut count = 0;
        for x in 0..=50 {
            for y in 0..=50 {
                if let Some(res) = dijkstra(&self.start, |p| p.successors(&self), |p| *p == Pos::new(x, y)) {
                    if res.1 <= max_dist {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

fn part1(input: &str) -> usize {
    let maze = Maze::from(input);
    maze.solve()
}

fn part2(input: &str) -> usize {
    let maze = Maze::from(input);
    maze.within(50)
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
        assert_eq!(11, part1(&input));
    }
}
