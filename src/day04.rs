extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day04.txt");

const POWERS_OF_TEN: [u32; 2] = [1, 10];

fn u32_from_bytes(bytes: &[u8]) -> u32 {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, x)| {
        acc + (x & 0x0f) as u32 * POWERS_OF_TEN[ix]
    })
}

fn get_winning_count(line: &[u8]) -> u32 {
    let numbers = line.split(|c| *c == b':').nth(1).unwrap();
    let mut groups = numbers
        .split(|c| *c == b'|')
        .map(|group| group.trim_ascii_end());
    let winning: u128 = groups
        .next()
        .unwrap()
        .split(|c| *c == b' ')
        .filter(|num| !num.is_empty())
        .fold(0, |acc, num| {
            let num: u32 = u32_from_bytes(num);
            acc | (1_u128 << (num - 1))
        });
    groups
        .next()
        .unwrap()
        .split(|c| *c == b' ')
        .filter(|num| !num.is_empty())
        .filter(|num| {
            let num: u32 = u32_from_bytes(num);
            winning & (1_u128 << (num - 1)) != 0
        })
        .count() as u32
}

fn part1(input: &[u8]) -> usize {
    input
        .split(|c| *c == b'\n')
        .map(|line| {
            let winning = get_winning_count(line);
            if winning == 0 {
                0
            } else {
                1_usize << (winning - 1)
            }
        })
        .sum()
}

fn part2(input: &[u8]) -> usize {
    let winning_map: Vec<u32> = input
        .split(|c| *c == b'\n')
        .map(get_winning_count)
        .collect();

    let mut result = 0;

    let mut counts: Vec<usize> = vec![1; winning_map.len()];

    for i in 0..=counts.len() - 1 {
        let current = counts[i];
        result += current;
        let won = winning_map[i];
        for count in counts.iter_mut().skip(i).take(won as usize + 1) {
            *count += current;
        }
    }

    result
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day04.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 30);
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
