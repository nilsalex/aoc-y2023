extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day07.txt");

fn get_value(hand: &mut [u8]) -> u8 {
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

fn get_value_with_joker(hand: &mut [u8]) -> u8 {
    let value = get_value(hand);
    let num_jokers = hand.iter().filter(|c| **c == b'J').count();
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
    let mut hands: Vec<u64> = input
        .split(|c| *c == b'\n')
        .map(|line| {
            let entry = 0_u64;
            let hand = line.iter().take(5).enumerate().fold(entry, |acc, (i, c)| {
                let bits: u64 = match c {
                    b'2'..=b'9' => (c - b'2') as u64,
                    b'T' => 8,
                    b'J' => 9,
                    b'Q' => 10,
                    b'K' => 11,
                    b'A' => 12,
                    _ => panic!(),
                } << (64 - 4 * (i + 2));
                acc | bits
            });
            let mut hand_buf = [0; 5];
            hand_buf.copy_from_slice(&line[0..=4]);
            let value = (get_value(&mut hand_buf) as u64) << 60;
            let bid = line
                .iter()
                .skip(6)
                .fold(0, |acc, digit| 10 * acc + (digit - b'0') as u64);
            value | hand | bid
        })
        .collect();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(index, bid)| (index + 1) * (bid & 0xfff) as usize)
        .sum()
}

fn part2(input: &[u8]) -> usize {
    let mut hands: Vec<u64> = input
        .split(|c| *c == b'\n')
        .map(|line| {
            let entry = 0_u64;
            let hand = line.iter().take(5).enumerate().fold(entry, |acc, (i, c)| {
                let bits: u64 = match c {
                    b'2'..=b'9' => (c - b'1') as u64,
                    b'T' => 9,
                    b'J' => 0,
                    b'Q' => 10,
                    b'K' => 11,
                    b'A' => 12,
                    _ => panic!(),
                } << (64 - 4 * (i + 2));
                acc | bits
            });
            let mut hand_buf = [0; 5];
            hand_buf.copy_from_slice(&line[0..=4]);
            let value = (get_value_with_joker(&mut hand_buf) as u64) << 60;
            let bid = line
                .iter()
                .skip(6)
                .fold(0, |acc, digit| 10 * acc + (digit - b'0') as u64);
            value | hand | bid
        })
        .collect();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(index, bid)| (index + 1) * (bid & 0xfff) as usize)
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
