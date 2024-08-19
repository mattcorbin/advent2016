use std::fs;

#[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Tile {
    #[default]
    Safe,
    Trap,
}

impl Tile {
    fn from(c: char) -> Tile {
        match c {
            '.' => Tile::Safe,
            '^' => Tile::Trap,
            _ => panic!("at the disco!"),
        }
    }

    fn compute(left: Tile, centre: Tile, right: Tile) -> Tile {
        if (left != right) && (left == centre || right == centre) {
            Tile::Trap
        } else {
            Tile::Safe
        }
    }
}

#[derive(Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct Room {
    floor: Vec<Vec<Tile>>,
}

impl Room {
    fn from(s: &str) -> Room {
        let mut row = Vec::new();
        for char in s.chars() {
            row.push(Tile::from(char));
        }
        Room { floor: vec![row] }
    }

    fn add_row(&mut self) {
        let mut new_row = Vec::new();
        let previous_row = self.floor.len() -1;
        for (idx, _) in self.floor[previous_row].iter().enumerate() {
            let left;
            let centre;
            let right;

            if idx == 0 {
                left = Tile::Safe;
            } else {
                left = self.floor[previous_row][idx-1];
            }

            if idx == self.floor[previous_row].len() - 1 {
                right = Tile::Safe;
            } else {
                right = self.floor[previous_row][idx+1];
            }

            centre = self.floor[previous_row][idx];
            new_row.push(Tile::compute(left, centre, right));
        }
        self.floor.push(new_row);
    }

    fn add_rows(&mut self, rows: usize) {
        while self.floor.len() < rows {
            self.add_row();
        }
    }

    fn count_safe(&self) -> usize {
        let mut count = 0;
        for row in &self.floor {
            for tile in row {
                if *tile == Tile::Safe {
                    count += 1;
                }
            }
        }
        count
    }
}

fn part1(input: &str, rows: usize) -> usize {
    let mut room = Room::from(input);
    room.add_rows(rows);
    room.count_safe()
}

fn part2(input: &str, rows:usize) -> usize {
    part1(input, rows)
}

fn main() {
    let f = fs::read_to_string("input.txt").expect("input.txt should exist");
    let mut lines = f.lines();
    let mut splits = lines.next().unwrap().split(",");
    let input = splits.next().unwrap();
    let rows = splits.next().unwrap().parse().unwrap();
    println!("{}", part1(input, rows));
    let mut splits = lines.next().unwrap().split(",");
    let input = splits.next().unwrap();
    let rows = splits.next().unwrap().parse().unwrap();
    println!("{}", part2(&input, rows));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let f = fs::read_to_string("test.txt").expect("test.txt should exist");
        for line in f.lines() {
            let mut splits = line.split(",");
            let input = splits.next().unwrap();
            let rows = splits.next().unwrap().parse().unwrap();
            let expected: usize = splits.next().unwrap().parse().unwrap();
            assert_eq!(expected, part1(input, rows));
        }
    }

    #[test]
    fn test_p2() {
        let f = fs::read_to_string("test.txt").expect("test.txt should exist");
        for line in f.lines() {
            let mut splits = line.split(",");
            let input = splits.next().unwrap();
            let rows = splits.next().unwrap().parse().unwrap();
            let expected: usize = splits.next().unwrap().parse().unwrap();
            assert_eq!(expected, part2(input, rows));
        }    }
}
