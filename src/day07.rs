extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day07.txt");

fn get_value(hand: &[u8]) -> u8 {
    let mut hand = Vec::from(hand);
    hand.sort();
    let mut counts: Vec<u8> = vec![];
    let mut current_count: u8 = 1;
    let mut previous = hand[0];
    for card in &hand[1..] {
        if *card == previous {
            current_count += 1;
        } else {
            counts.push(current_count);
            current_count = 1;
            previous = *card;
        }
    }
    counts.push(current_count);
    counts.sort();
    match counts.len() {
        5 => 1,
        4 => 2,
        3 => {
            if counts[2] == 2 {
                3
            } else {
                4
            }
        }
        2 => {
            if counts[0] == 2 {
                5
            } else {
                6
            }
        }
        1 => 7,
        _ => panic!(),
    }
}

fn get_value_with_joker(hand: &[u8]) -> u8 {
    let value = get_value(hand);
    let num_jokers = hand.iter().filter(|c| **c == 1).count();
    if num_jokers == 0 {
        value
    } else {
        match value {
            7 => 7,
            6 => 7,
            5 => 7,
            4 => 6,
            3 => {
                if num_jokers == 1 {
                    5
                } else {
                    6
                }
            }
            2 => 4,
            1 => 2,
            _ => panic!(),
        }
    }
}

fn part1(input: &[u8]) -> usize {
    let mut hands: Vec<(Vec<u8>, usize)> = input
        .split(|c| *c == b'\n')
        .map(|line| {
            let mut groups = line.split(|c| *c == b' ');
            let hand: Vec<u8> = groups
                .next()
                .unwrap()
                .iter()
                .map(|c| match c {
                    b'2'..=b'9' => c - b'0',
                    b'T' => 10,
                    b'J' => 11,
                    b'Q' => 12,
                    b'K' => 13,
                    b'A' => 14,
                    _ => panic!(),
                })
                .collect();
            let value = get_value(&hand);
            let hand = [&[value], &hand[..]].concat();
            let bid = groups
                .next()
                .unwrap()
                .iter()
                .fold(0, |acc, digit| 10 * acc + (digit - b'0') as usize);
            (hand, bid)
        })
        .collect();

    hands.sort_by(|(hand1, _), (hand2, _)| hand1.cmp(hand2));

    hands
        .iter()
        .enumerate()
        .map(|(index, (_, bid))| (index + 1) * bid)
        .sum()
}

fn part2(input: &[u8]) -> usize {
    let mut hands: Vec<(Vec<u8>, usize)> = input
        .split(|c| *c == b'\n')
        .map(|line| {
            let mut groups = line.split(|c| *c == b' ');
            let hand: Vec<u8> = groups
                .next()
                .unwrap()
                .iter()
                .map(|c| match c {
                    b'2'..=b'9' => c - b'0',
                    b'T' => 10,
                    b'J' => 1,
                    b'Q' => 11,
                    b'K' => 12,
                    b'A' => 13,
                    _ => panic!(),
                })
                .collect();
            let value = get_value_with_joker(&hand);
            let hand = [&[value], &hand[..]].concat();
            let bid = groups
                .next()
                .unwrap()
                .iter()
                .fold(0, |acc, digit| 10 * acc + (digit - b'0') as usize);
            (hand, bid)
        })
        .collect();

    hands.sort_by(|(hand1, _), (hand2, _)| hand1.cmp(hand2));

    hands
        .iter()
        .enumerate()
        .map(|(index, (_, bid))| (index + 1) * bid)
        .sum()
}

pub fn main() {
    let input = INPUT.trim_ascii_end();

    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day07.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 6440);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 5905);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = INPUT.trim_ascii_end();
        b.iter(|| part1(input))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = INPUT.trim_ascii_end();
        b.iter(|| part2(input))
    }
}
