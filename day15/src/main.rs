use std::fs;

struct Disc {
    id: isize,
    starting_position: isize,
    positions: isize,
}

// Disc #1 has 5 positions; at time=0, it is at position 4.

impl Disc {
    fn new(id: isize, starting_position: isize, positions: isize) -> Disc {
        Disc {
            id,
            starting_position,
            positions,
        }
    }

    fn from(input: &str) -> Disc {
        let item = input
            .replace("Disc #", "")
            .replace(" has ", " ")
            .replace(" positions; at time=0, it is at position ", " ")
            .replace(".", "");
        let mut splits = item.split(" ");
        let id = splits.next().unwrap().parse().unwrap();
        let positions = splits.next().unwrap().parse().unwrap();
        let starting_position = splits.next().unwrap().parse().unwrap();
        Disc {
            id,
            starting_position,
            positions,
        }
    }
}

pub fn extended_gcd(a: isize, b: isize) -> (isize, isize, isize) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (d, q, p) = extended_gcd(b, a % b);
        (d, p, q - a / b * p)
    }
}

pub fn chinese_remainder_theorem(b: &[isize], modulo: &[isize]) -> Option<(isize, isize)> {
    let (mut result, mut m) = (0, 1);
    for i in 0..b.len() {
        let (d, p, _) = extended_gcd(m, modulo[i]);
        if (b[i] - result) % d != 0 {
            return None;
        }
        let tmp = ((b[i] - result) / d * p) % (modulo[i] / d);
        result += m * tmp;
        m *= modulo[i] / d;
    }
    Some(((result % m + m) % m, m))
}

fn part1(input: &str) -> isize {
    let mut discs = Vec::new();
    for line in input.lines() {
        discs.push(Disc::from(line));
    }
    let mut initial_values = Vec::new();
    let mut disc_sizes = Vec::new();
    for (idx, disc) in discs.iter().enumerate() {
        initial_values.push(-1 * disc.starting_position - (idx + 1) as isize);
        disc_sizes.push(disc.positions);
    }
    let (first, _) = chinese_remainder_theorem(&initial_values, &disc_sizes).unwrap();
    first
}

fn part2(input: &str) -> isize {
    let mut discs = Vec::new();
    for line in input.lines() {
        discs.push(Disc::from(line));
    }
    discs.push(Disc::new(7, 0, 11));
    let mut initial_values = Vec::new();
    let mut disc_sizes = Vec::new();
    for (idx, disc) in discs.iter().enumerate() {
        initial_values.push(-1 * disc.starting_position - (idx + 1) as isize);
        disc_sizes.push(disc.positions);
    }
    let (first, _) = chinese_remainder_theorem(&initial_values, &disc_sizes).unwrap();
    first
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
        assert_eq!(5, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(85, part2(&input));
    }
}
