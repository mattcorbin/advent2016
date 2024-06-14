use std::collections::HashMap;
use std::fs;

fn build_cols(input: &str) -> Vec<HashMap<char, usize>> {
    let num_cols = input.lines().next().unwrap().len();
    let mut columns = Vec::new();
    for _ in 0..num_cols {
        columns.push(HashMap::new())
    }
    for line in input.lines() {
        for (idx, c) in line.chars().enumerate() {
            if let Some(&count) = columns[idx].get(&c) {
                columns[idx].insert(c, count+1);
            } else {
                columns[idx].insert(c, 1);
            }
        }
    }
    columns
}

fn part1(input: &str) -> String {
    let mut result = String::new();
    let columns = build_cols(input);
    for column in columns {
        let mut max = (' ', 0);
        for (key, value) in column {
            if value > max.1 {
                max = (key, value);
            }
        }
        result.push(max.0);

    }
    result
}

fn part2(input: &str) -> String {
    let mut result = String::new();
    let columns = build_cols(input);
    for column in columns {
        let mut min = (' ', usize::MAX);
        for (key, value) in column {
            if value < min.1 {
                min = (key, value);
            }
        }
        result.push(min.0);

    }
    result
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
        assert_eq!("easter".to_string(), part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!("advent".to_string(), part2(&input));
    }
}
