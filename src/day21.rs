extern crate test;

use std::collections::HashSet;

const INPUT: &[u8] = include_bytes!("../inputs/day21.txt");

fn part1(input: &[u8], steps: usize) -> usize {
    let mut start = (0, 0);
    let mut rocks = HashSet::new();

    for (row, line) in input.split(|&b| b == b'\n').enumerate() {
        for (col, byte) in line.iter().enumerate() {
            match byte {
                b'S' => {
                    start = (row, col);
                }
                b'#' => {
                    rocks.insert((row, col));
                }
                _ => {}
            }
        }
    }

    let mut reachable = HashSet::from([start]);

    for _ in 0..steps {
        let mut next_reachable = HashSet::new();

        for &(row, col) in reachable.iter() {
            for next in [
                (row - 1, col),
                (row + 1, col),
                (row, col - 1),
                (row, col + 1),
            ] {
                if !rocks.contains(&next) {
                    next_reachable.insert(next);
                }
            }
        }

        std::mem::swap(&mut reachable, &mut next_reachable);
    }

    reachable.len()
}

fn part2(input: &[u8]) -> usize {
    0
}

pub fn main() {
    let input = INPUT.trim_ascii_end();

    println!("{}", part1(input, 64));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day21.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input, 6), 16);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 0);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = INPUT.trim_ascii_end();
        b.iter(|| part1(input, 64))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = INPUT.trim_ascii_end();
        b.iter(|| part2(input))
    }
}
