use std::fs;

#[derive(Clone, Debug, Default, Hash, Eq, PartialEq)]
struct IPv7 {
    raw: String,
    address: Vec<String>,
    hypernet_sequences: Vec<String>,
}

impl IPv7 {
    fn new(input: &str) -> Self {
        let local = input.to_string().replace("[", " ").replace("]", " ");
        let mut address = Vec::new();
        let mut hypernet_sequences = Vec::new();
        for (idx, split) in local.split(" ").enumerate() {
            if idx % 2 == 0 {
                address.push(split.to_string());
            } else {
                hypernet_sequences.push(split.to_string());
            }
        }
        IPv7 {
            raw: input.to_string(),
            address,
            hypernet_sequences,
        }
    }

    fn supports_tls(&self) -> bool {
        for sequence in &self.hypernet_sequences {
            for chunk in sequence.chars().collect::<Vec<char>>().windows(4) {
                if chunk[0] == chunk[3] && chunk[1] == chunk[2] && chunk[0] != chunk[1] {
                    return false;
                }
            }
        }
        for section in &self.address {
            for chunk in section.chars().collect::<Vec<char>>().windows(4) {
                if chunk[0] == chunk[3] && chunk[1] == chunk[2] && chunk[0] != chunk[1] {
                    return true;
                }
            }
        }
        false
    }

    fn supports_ssl(&self) -> bool {
        let mut abas = Vec::new();
        for section in &self.address {
            for chunk in section.chars().collect::<Vec<char>>().windows(3) {
                if chunk[0] == chunk[2] && chunk[0] != chunk[1] {
                    abas.push(chunk.to_vec());
                }
            }
        }
        if abas.len() == 0 {
            return false;
        }
        for sequence in &self.hypernet_sequences {
            for chunk in sequence.chars().collect::<Vec<char>>().windows(3) {
                if chunk[0] == chunk[2] && chunk[0] != chunk[1] {
                    for aba in &abas {
                        if aba[1] == chunk[0] && aba[0] == chunk[1] {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

fn part1(input: &str) -> usize {
    let mut ip_addrs = Vec::new();
    for line in input.lines() {
        ip_addrs.push(IPv7::new(line));
    }
    ip_addrs
        .iter()
        .map(|x| x.supports_tls())
        .filter(|x| *x)
        .collect::<Vec<bool>>()
        .len()
}

fn part2(input: &str) -> usize {
    let mut ip_addrs = Vec::new();
    for line in input.lines() {
        ip_addrs.push(IPv7::new(line));
    }
    ip_addrs
        .iter()
        .map(|x| x.supports_ssl())
        .filter(|x| *x)
        .collect::<Vec<bool>>()
        .len()
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
        assert_eq!(2, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(3, part2(&input));
    }
}
