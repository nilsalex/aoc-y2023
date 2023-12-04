extern crate test;

const INPUT: &str = include_str!("../inputs/day04.txt");

fn get_winning_count(line: &str) -> u32 {
    let numbers = line.split(": ").nth(1).unwrap();
    let mut groups = numbers.split(" | ");
    let winning: u128 = groups
        .next()
        .unwrap()
        .split_whitespace()
        .fold(0, |acc, num| {
            let num: u32 = num.parse().unwrap();
            acc | (1_u128 << (num - 1))
        });
    groups
        .next()
        .unwrap()
        .split_whitespace()
        .filter(|num| {
            let num: u32 = num.parse().unwrap();
            winning & (1_u128 << (num - 1)) != 0
        })
        .count() as u32
}

fn part1(input: &str) -> usize {
    input
        .lines()
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

fn part2(input: &str) -> usize {
    let winning_map: Vec<u32> = input.lines().map(get_winning_count).collect();

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
    let input = INPUT.trim_end();

    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &str = include_str!("../test_inputs/day04.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_end();
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_end();
        assert_eq!(part2(input), 30);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = INPUT.trim_end();
        b.iter(|| part1(input))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = INPUT.trim_end();
        b.iter(|| part2(input))
    }
}
