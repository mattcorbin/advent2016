use std::fs;

fn parse_input(input: &str) -> Vec<[usize; 3]> {
    let mut retval = Vec::new();
    for line in input.lines() {
        let mut splits = line.split_whitespace();
        retval.push([
            splits.next().unwrap().parse::<usize>().unwrap(),
            splits.next().unwrap().parse::<usize>().unwrap(),
            splits.next().unwrap().parse::<usize>().unwrap(),
        ])
    }
    retval
}

fn is_valid_triangle(sides: [usize; 3]) -> bool {
    if (sides[0] + sides[1] <= sides[2])
        || (sides[0] + sides[2] <= sides[1])
        || (sides[1] + sides[2] <= sides[0])
    {
        false
    } else {
        true
    }
}

fn part1(input: &str) -> usize {
    let mut valid = 0;
    for triangle in parse_input(input) {
        if is_valid_triangle(triangle) {
            valid += 1;
        }
    }
    valid
}



fn parse_input_chunks(input: &str) -> Vec<[usize; 3]> {
    let mut retval = Vec::new();
    for line in input.lines().collect::<Vec<&str>>().chunks(3) {
        let mut splits1 = line[0].split_whitespace();
        let mut splits2 = line[1].split_whitespace();
        let mut splits3 = line[2].split_whitespace();
        retval.push([
            splits1.next().unwrap().parse::<usize>().unwrap(),
            splits2.next().unwrap().parse::<usize>().unwrap(),
            splits3.next().unwrap().parse::<usize>().unwrap(),
        ]);
        retval.push([
            splits1.next().unwrap().parse::<usize>().unwrap(),
            splits2.next().unwrap().parse::<usize>().unwrap(),
            splits3.next().unwrap().parse::<usize>().unwrap(),
        ]);
        retval.push([
            splits1.next().unwrap().parse::<usize>().unwrap(),
            splits2.next().unwrap().parse::<usize>().unwrap(),
            splits3.next().unwrap().parse::<usize>().unwrap(),
        ])
    }
    retval
}


fn part2(input: &str) -> usize {
    let mut valid = 0;
    for triangle in parse_input_chunks(input) {
        if is_valid_triangle(triangle) {
            valid += 1;
        }
    }
    valid
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
        assert_eq!(0, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(0, part2(&input));
    }
}
