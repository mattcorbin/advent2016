extern crate md5;

use std::collections::VecDeque;
use std::fs;

use md5::{Digest, Md5};

fn parse_input(input: &str) -> (String, usize) {
    let mut lines = input.lines();
    (
        lines.next().unwrap().to_string(),
        lines.next().unwrap().parse().unwrap(),
    )
}

fn hash(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input);
    format!("{:02x}", hasher.finalize())
}

fn streched_hash(input: &str) -> String {
    let mut current_hash;
    let mut hasher = Md5::new();
    hasher.update(input);
    current_hash = hasher.finalize();
    for _ in 0..2016 {
        let mut hasher = Md5::new();
        hasher.update(format!("{:02x}", current_hash));
        current_hash = hasher.finalize();
    }
    format!("{:02x}", current_hash)
}

fn potential_key(hash: &str) -> Option<char> {
    for window in hash.chars().collect::<Vec<char>>().windows(3) {
        if window[0] == window[1] && window[0] == window[2] {
            return Some(window[0]);
        }
    }
    None
}

fn is_key(items: &VecDeque<(usize, String)>, desired: char) -> bool {
    let test = format!("{desired}{desired}{desired}{desired}{desired}");
    for item in items {
        if item.1.contains(&test) {
            return true;
        }
    }
    false
}

fn part1(salt: &str, desired_count: usize) -> usize {
    let mut queue = VecDeque::with_capacity(1000);
    let mut current_index = 0;
    let mut queue_index = 0;
    let mut count = 0;
    for _ in 0..1000 {
        queue.push_back((queue_index, hash(&format!("{}{}", salt, queue_index))));
        queue_index += 1;
    }
    while count < desired_count {
        queue.push_back((queue_index, hash(&format!("{}{}", salt, queue_index))));
        queue_index += 1;
        let pop = queue.pop_front().unwrap();
        current_index = pop.0;
        let test = pop.1;
        if let Some(c) = potential_key(&test) {
            if is_key(&queue, c) {
                count += 1;
            }
        }
    }
    current_index
}

fn part2(salt: &str, desired_count: usize) -> usize {
    let mut queue = VecDeque::with_capacity(1000);
    let mut current_index = 0;
    let mut queue_index = 0;
    let mut count = 0;
    for _ in 0..1000 {
        queue.push_back((queue_index, streched_hash(&format!("{}{}", salt, queue_index))));
        queue_index += 1;
    }
    while count < desired_count {
        queue.push_back((queue_index, streched_hash(&format!("{}{}", salt, queue_index))));
        queue_index += 1;
        let pop = queue.pop_front().unwrap();
        current_index = pop.0;
        let test = pop.1;
        if let Some(c) = potential_key(&test) {
            if is_key(&queue, c) {
                count += 1;
            }
        }
    }
    current_index
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    let (salt, desired_count) = parse_input(&input);
    println!("{}", part1(&salt, desired_count));
    println!("{}", part2(&salt, desired_count));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        let (salt, desired_count) = parse_input(&input);
        assert_eq!(22728, part1(&salt, desired_count));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        let (salt, desired_count) = parse_input(&input);
        assert_eq!(22551, part2(&salt, desired_count));
    }
}
