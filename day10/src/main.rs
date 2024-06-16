use std::collections::HashMap;
use std::fs;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum DestinationType {
    Bot,
    Output,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Destination {
    destination_type: DestinationType,
    value: usize,
}

impl Destination {
    fn new(destination_type: DestinationType, value: usize) -> Self {
        Destination {
            destination_type,
            value,
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Bot {
    hands: Vec<usize>,
    dest_low: Destination,
    dest_high: Destination,
}

impl Bot {
    fn new() -> Self {
        Bot {
            hands: Vec::new(),
            dest_low: Destination::new(DestinationType::Bot, 0),
            dest_high: Destination::new(DestinationType::Bot, 0),
        }
    }
}

fn parse_input(input: &str) -> HashMap<usize, Bot> {
    let mut bots: HashMap<usize, Bot> = HashMap::new();
    for line in input.lines() {
        if line.starts_with("value") {
            let mut splits = line.split(" ");
            splits.next();
            let chip = splits.next().unwrap().parse::<usize>().unwrap();
            splits.next();
            splits.next();
            splits.next();
            let bot_id = splits.next().unwrap().parse::<usize>().unwrap();
            if let Some(bot) = bots.get(&bot_id) {
                let mut temp = bot.clone();
                temp.hands.push(chip);
                bots.insert(bot_id, temp);
            } else {
                let mut bot = Bot::new();
                bot.hands.push(chip);
                bots.insert(bot_id, bot);
            }
        } else {
            let mut splits = line.split(" ");
            let mut dest_low = Destination::new(DestinationType::Bot, 0);
            let mut dest_high = Destination::new(DestinationType::Bot, 0);
            splits.next();
            let bot_id = splits.next().unwrap().parse::<usize>().unwrap();
            splits.next();
            splits.next();
            splits.next();
            match splits.next().unwrap() {
                "bot" => dest_low.destination_type = DestinationType::Bot,
                "output" => dest_low.destination_type = DestinationType::Output,
                _ => panic!("at the disco"),
            }
            dest_low.value = splits.next().unwrap().parse().unwrap();
            splits.next();
            splits.next();
            splits.next();
            match splits.next().unwrap() {
                "bot" => dest_high.destination_type = DestinationType::Bot,
                "output" => dest_high.destination_type = DestinationType::Output,
                _ => panic!("at the disco"),
            }
            dest_high.value = splits.next().unwrap().parse().unwrap();
            if let Some(bot) = bots.get(&bot_id) {
                let mut temp = bot.clone();
                temp.dest_low = dest_low;
                temp.dest_high = dest_high;
                bots.insert(bot_id, temp);
            } else {
                let mut bot = Bot::new();
                bot.dest_low = dest_low;
                bot.dest_high = dest_high;
                bots.insert(bot_id, bot);
            }
        }
    }
    bots
}

fn part1(input: &str, high: usize, low: usize) -> usize {
    let mut desired_bot = 0;
    let mut bots = parse_input(input);
    loop {
        let mut execute = 0;
        for (id, bot) in &bots {
            if bot.hands.len() == 2 {
                execute = *id;
                break;
            }
        }
        if let Some(bot) = bots.clone().get(&execute) {
            let mut new_bot = bot.clone();
            let chips = bot.hands.clone();
            let high_chip = *chips.iter().max().unwrap();
            let low_chip = *chips.iter().min().unwrap();
            if high_chip == high && low_chip == low {
                desired_bot = execute;
                break;
            }
            if bot.dest_high.destination_type == DestinationType::Bot {
                if let Some(b) = bots.clone().get(&bot.dest_high.value) {
                    let mut new_b = b.clone();
                    new_b.hands.push(high_chip);
                    bots.insert(bot.dest_high.value, new_b);
                } else {
                    panic!("at the disco");
                }
            }
            if bot.dest_low.destination_type == DestinationType::Bot {
                if let Some(b) = bots.clone().get(&bot.dest_low.value) {
                    let mut new_b = b.clone();
                    new_b.hands.push(low_chip);
                    bots.insert(bot.dest_low.value, new_b);
                } else {
                    panic!("at the disco");
                }
            }
            new_bot.hands = Vec::new();
            bots.insert(execute, new_bot);
        } else {
            panic!("at the disco");
        }
    }
    desired_bot
}

fn part2(input: &str) -> usize {
    let mut bots = parse_input(input);
    let mut output = HashMap::new();
    let mut run = true;
    loop {
        run = false;
        let mut execute = 0;
        for (id, bot) in &bots {
            if bot.hands.len() == 2 {
                execute = *id;
                run = true;
                break;
            }
        }
        if !run {
            break
        }
        if let Some(bot) = bots.clone().get(&execute) {
            let mut new_bot = bot.clone();
            let chips = bot.hands.clone();
            let high_chip = *chips.iter().max().unwrap();
            let low_chip = *chips.iter().min().unwrap();
            if bot.dest_high.destination_type == DestinationType::Bot {
                if let Some(b) = bots.clone().get(&bot.dest_high.value) {
                    let mut new_b = b.clone();
                    new_b.hands.push(high_chip);
                    bots.insert(bot.dest_high.value, new_b);
                } else {
                    panic!("at the disco");
                }
            } else {
                output.insert(bot.dest_high.value, high_chip);
            }
            if bot.dest_low.destination_type == DestinationType::Bot {
                if let Some(b) = bots.clone().get(&bot.dest_low.value) {
                    let mut new_b = b.clone();
                    new_b.hands.push(low_chip);
                    bots.insert(bot.dest_low.value, new_b);
                } else {
                    panic!("at the disco");
                }
            } else {
                output.insert(bot.dest_low.value, low_chip);
            }
            new_bot.hands = Vec::new();
            bots.insert(execute, new_bot);
        } else {
            panic!("at the disco");
        }
    }
    output.get(&0).unwrap() * output.get(&1).unwrap() * output.get(&2).unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    println!("{}", part1(&input, 61, 17));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(2, part1(&input, 5, 2));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(30, part2(&input));
    }
}
