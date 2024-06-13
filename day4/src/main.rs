use std::collections::HashMap;
use std::fs;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct Room {
    name: Vec<char>,
    sector_id: usize,
    top: Vec<char>,
    checksum: Vec<char>,
}

impl Room {
    fn new(input: &str) -> Self {
        let temp = input.replace("]", "");
        let mut splits = temp.split("[");
        let mut name = Vec::new();
        let mut letters = HashMap::new();
        let mut sector_id = String::new();
        for c in splits.next().unwrap().chars() {
            if c.is_ascii_lowercase() {
                name.push(c);
                if let Some(val) = letters.get(&c) {
                    letters.insert(c, val + 1);
                } else {
                    letters.insert(c, 1);
                }
            } else if c.is_numeric() {
                sector_id.push(c);
            } else if c == '-' {
                name.push(' ');
            }
        }
        name.pop();
        let sector_id = sector_id.parse().unwrap();
        let top = Room::calculate_top(&letters);
        let checksum = splits.next().unwrap().chars().collect();
        Room {
            name,
            sector_id,
            top,
            checksum,
        }
    }

    fn calculate_top(letters: &HashMap<char, usize>) -> Vec<char> {
        let mut retval = letters
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<(char, usize)>>();
        retval.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
        retval.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));
        retval.iter().map(|(k, _)| *k).collect()
    }

    fn real(&self) -> bool {
        self.top[..5] == self.checksum[..5]
    }

    fn decrypt(&self) -> String {
        let mut retval = String::new();
        for c in &self.name {
            if *c == ' ' {
                retval.push(*c);
            } else {
                retval.push(
                    (((*c as usize - 'a' as usize + self.sector_id) % 26) + 'a' as usize) as u8
                        as char,
                );
            }
        }
        retval
    }
}

fn part1(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let room = Room::new(line);
        if room.real() {
            sum += room.sector_id;
        }
    }
    sum
}

fn part2(input: &str) -> usize {
    let mut rooms = Vec::new();
    for line in input.lines() {
        let room = Room::new(line);
        if room.real() {
            rooms.push(room);
        }
    }
    for room in rooms {
        let code = room.decrypt();
        if code.contains("north") {
            return room.sector_id;
        }
    }
    0
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
        assert_eq!(1514, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(0, part2(&input));
    }
}
