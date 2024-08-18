use std::fs;

fn flip(input: &str) -> String {
    let mut retval = String::new();
    for c in input.chars() {
        match c {
            '0' => retval.push('1'),
            '1' => retval.push('0'),
            _ => panic!("at the disco!"),
        }
    }
    retval
}

fn generate(input: String, min_len: usize) -> String {
    if input.len() >= min_len {
        return input;
    } else {
        let mut new_state = String::new();
        for char in input.chars() {
            new_state.push(char);
        }
        new_state.push('0');
        for char in flip(&input.chars().rev().collect::<String>()).chars() {
            new_state.push(char);
        }
        generate(new_state, min_len)
    }
}

fn checksum(state: String) -> String {
    if state.len() % 2 == 1 {
        state
    } else {
        let mut new_state = String::new();
        for chunk in state.chars().collect::<Vec<char>>().chunks(2) {
            if chunk[0] == chunk[1] {
                new_state.push('1');
            } else {
                new_state.push('0');
            }
        }
        checksum(new_state)
    }
}

fn part1(initial_state: String, min_len: usize) -> String {
    let mut state = generate(initial_state, min_len);
    state.truncate(min_len);
    checksum(state)
}

fn part2(initial_state: String, min_len: usize) -> String {
    part1(initial_state, min_len)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    let mut lines = input.lines();

    let mut splits = lines.next().unwrap().split(",");
    let initial_state = splits.next().unwrap().to_string();
    let min_len = splits.next().unwrap().parse().unwrap();
    println!("{}", part1(initial_state, min_len));

    let mut splits = lines.next().unwrap().split(",");
    let initial_state = splits.next().unwrap().to_string();
    let min_len = splits.next().unwrap().parse().unwrap();
    println!("{}", part2(initial_state, min_len));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        for line in input.lines() {
            let mut splits = line.split(",");
            let initial_state = splits.next().unwrap().to_string();
            let min_len = splits.next().unwrap().parse().unwrap();
            let expected = splits.next().unwrap().to_string();
            assert_eq!(expected, part1(initial_state, min_len));
        }
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        for line in input.lines() {
            let mut splits = line.split(",");
            let initial_state = splits.next().unwrap().to_string();
            let min_len = splits.next().unwrap().parse().unwrap();
            let expected = splits.next().unwrap().to_string();
            assert_eq!(expected, part1(initial_state, min_len));
        }
    }
}
