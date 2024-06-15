use std::fs;

fn part1(input: &str) -> usize {
    let mut decompressed = String::new();
    let mut next_idx = 0;
    let mut in_marker = false;
    let mut marker = String::new();
    for (idx, c) in input.chars().enumerate() {
        if next_idx != 0 {
            if idx == next_idx {
                next_idx = 0;
            }
        } else if c == '(' {
            in_marker = true;
        } else if c == ')' {
            in_marker = false;
            let mut splits = marker.split("x");
            let num_chars: usize = splits.next().unwrap().parse().unwrap();
            let repeats: usize = splits.next().unwrap().parse().unwrap();
            let block = &input[idx+1..idx+1+num_chars];
            for _ in 0..repeats {
                decompressed += block;
            }
            marker.clear();
            next_idx = idx+num_chars;
        } else if in_marker {
            marker.push(c);
        } else {
            decompressed.push(c);
        }
    }
    decompressed.len()
}

fn part2(input: &str) -> usize {
    let mut count = 0;
    let mut next_idx = 0;
    let mut in_marker = false;
    let mut marker = String::new();
    for (idx, c) in input.chars().enumerate() {
        if next_idx != 0 {
            if idx == next_idx {
                next_idx = 0;
            }
        } else if c == '(' {
            in_marker = true;
        } else if c == ')' {
            in_marker = false;
            let mut splits = marker.split("x");
            let num_chars: usize = splits.next().unwrap().parse().unwrap();
            let repeats: usize = splits.next().unwrap().parse().unwrap();
            let block = &input[idx+1..idx+1+num_chars];
            count += repeats * part2(block);
            marker.clear();
            next_idx = idx+num_chars;
        } else if in_marker {
            marker.push(c);
        } else {
            count += 1;
        }
    }
    count
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
        let lines = input.lines().map(|x| x.to_string()).collect::<Vec<String>>();
        assert_eq!(6, part1(&lines[0]));
        assert_eq!(7, part1(&lines[1]));
        assert_eq!(9, part1(&lines[2]));
        assert_eq!(11, part1(&lines[3]));
        assert_eq!(6, part1(&lines[4]));
        assert_eq!(18, part1(&lines[5]));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        let lines = input.lines().map(|x| x.to_string()).collect::<Vec<String>>();
        assert_eq!(6, part2(&lines[0]));
        assert_eq!(7, part2(&lines[1]));
        assert_eq!(9, part2(&lines[2]));
        assert_eq!(11, part2(&lines[3]));
        assert_eq!(3, part2(&lines[4]));
        assert_eq!(20, part2(&lines[5]));
        assert_eq!(241920, part2(&lines[6]));
        assert_eq!(445, part2(&lines[7]));
    }
}
