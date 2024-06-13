use std::fs;

use md5;
use rayon::prelude::*;

fn compute(input: &str, num: usize) -> char {
    let test = input.to_string() + num.to_string().as_str();
    let digest = md5::compute(test);
    let hash = format!("{:x}", digest);
    if hash.len() >= 6 {
        let chars = hash.chars().collect::<Vec<char>>();
        if chars[..5].iter().all(|c| *c == '0') {
            return chars[5];
        }
    }
    ' '
}

fn compute_with_position(input: &str, num: usize) -> (char, usize) {
    let test = input.to_string() + num.to_string().as_str();
    let digest = md5::compute(test);
    let hash = format!("{:x}", digest);
    if hash.len() >= 6 {
        let chars = hash.chars().collect::<Vec<char>>();
        if chars[..5].iter().all(|c| *c == '0') {
            if let Ok(loc) = chars[5].to_string().parse() {
                if loc < 8 {
                    return (chars[6], loc);
                }
            }
        }
    }
    (' ', 0)
}

fn part1(input: &str) -> String {
    let retval = (0..99999999)
        .into_par_iter()
        .map(|x| compute(input, x))
        .filter(|c| c.is_alphanumeric())
        .collect::<Vec<char>>();
    retval[..8].into_iter().collect()
}

fn part2(input: &str) -> String {
    let temp = (0..99999999)
        .into_par_iter()
        .map(|x| compute_with_position(input, x))
        .filter(|(c, _)| c.is_alphanumeric())
        .collect::<Vec<(char, usize)>>();
    let mut retval = vec![' ',' ',' ',' ',' ',' ',' ',' '];
    for (c, loc) in temp {
        if retval[loc] == ' ' {
            retval[loc] = c;
        }
    }
    retval.into_iter().collect()
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
        assert_eq!("18f47a30".to_string(), part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!("05ace8e3".to_string(), part2(&input));
    }
}
