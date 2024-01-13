extern crate test;

use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("../inputs/day12.txt");

fn springs_matches_counts(springs: &[u8], counts: &[usize]) -> bool {
    let mut actual_counts = springs
        .split(|c| *c == b'.')
        .filter(|group| !group.is_empty())
        .map(|group| group.len());

    let mut counts = counts.iter();

    loop {
        match (actual_counts.next(), counts.next()) {
            (None, None) => break,
            (Some(a), Some(b)) => {
                if a != *b {
                    return false;
                }
            }
            _ => return false,
        }
    }

    true
}

fn part1(input: &[u8]) -> usize {
    input
        .split(|c| *c == b'\n')
        .map(|line| {
            let mut groups = line.split(|c| *c == b' ');
            let springs = groups.next().unwrap();
            let counts: Vec<usize> = groups
                .next()
                .unwrap()
                .split(|c| *c == b',')
                .map(|num| num.iter().fold(0, |acc, c| acc * 10 + (c - b'0') as usize))
                .collect();

            let pos_unknown: Vec<usize> = springs
                .iter()
                .enumerate()
                .filter(|(_, c)| **c == b'?')
                .map(|(i, _)| i)
                .collect();

            let total_damaged: usize = counts.iter().sum();

            let known_damaged: usize = springs.iter().filter(|c| **c == b'#').count();

            let mut springs_buf = vec![0_u8; springs.len()];

            pos_unknown
                .iter()
                .combinations(total_damaged - known_damaged)
                .filter(|combination| {
                    for (i, &c) in springs.iter().enumerate() {
                        match c {
                            b'.' => springs_buf[i] = c,
                            b'#' => springs_buf[i] = c,
                            b'?' => {
                                if combination.contains(&&i) {
                                    springs_buf[i] = b'#'
                                } else {
                                    springs_buf[i] = b'.'
                                }
                            }
                            _ => panic!(),
                        }
                    }

                    springs_matches_counts(&springs_buf, &counts)
                })
                .count()
        })
        .sum()
}

fn part2(input: &[u8]) -> usize {
    input
        .split(|c| *c == b'\n')
        .map(|line| {
            let mut groups = line.split(|c| *c == b' ');
            let springs_single = groups.next().unwrap();
            let mut springs = vec![];
            for _ in 0..=4 {
                springs.extend_from_slice(springs_single);
                springs.push(b'?');
            }
            springs.pop();

            let counts_single: Vec<usize> = groups
                .next()
                .unwrap()
                .split(|c| *c == b',')
                .map(|num| num.iter().fold(0, |acc, c| acc * 10 + (c - b'0') as usize))
                .collect();
            let mut counts = vec![];
            for _ in 0..=4 {
                counts.extend_from_slice(&counts_single);
            }

            let mut memo = vec![None; 1024 * 1024];
            recurse(&springs, &counts, &mut memo)
        })
        .sum()
}

fn recurse(springs: &[u8], groups: &[usize], memo: &mut [Option<usize>]) -> usize {
    if let Some(count) = memo[1024 * springs.len() + groups.len()] {
        count
    } else {
        let mut count = 0;
        let remaining: usize = groups.iter().sum();
        let margin = springs.len() - remaining;
        let group = groups[0];

        for i in 0..=margin {
            if i > 0 && springs[i - 1] == b'#' {
                break;
            }

            if springs[i..i + group].iter().all(|c| *c != b'.') {
                if groups.len() == 1 {
                    if springs[i + group..].iter().all(|c| *c != b'#') {
                        count += 1;
                    }
                } else if i + group == springs.len()
                    || springs[i + group] != b'#' && springs.len() > i + remaining
                {
                    count += recurse(&springs[i + group + 1..], &groups[1..], memo);
                }
            }
        }

        memo[1024 * springs.len() + groups.len()] = Some(count);

        count
    }
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day12.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 21);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 525152);
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
