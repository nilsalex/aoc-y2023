extern crate test;

use std::collections::HashSet;

const INPUT: &[u8] = include_bytes!("../inputs/day11.txt");

fn solution(input: &[u8], scale: usize) -> usize {
    let mut galaxies: Vec<(usize, usize)> = vec![];
    let mut zero_rows: Vec<usize> = vec![];
    let mut zero_cols: Vec<usize> = vec![];
    let mut occupied_cols: HashSet<usize> = HashSet::new();

    input
        .split(|c| *c == b'\n')
        .enumerate()
        .for_each(|(y, line)| {
            let mut line_empty = true;

            line.iter().enumerate().for_each(|(x, c)| {
                if *c == b'#' {
                    galaxies.push((x, y));
                    occupied_cols.insert(x);
                    line_empty = false;
                }
            });

            if line_empty {
                zero_rows.push(y);
            }
        });

    galaxies.sort();

    let num_cols = input.iter().take_while(|c| **c != b'\n').count();
    for x in 0..=num_cols - 1 {
        if !occupied_cols.contains(&x) {
            zero_cols.push(x);
        }
    }

    let mut result = 0;

    for (i, (x1, y1)) in galaxies.iter().enumerate() {
        for (x2, y2) in &galaxies[i + 1..] {
            let x1_: usize = *(x1.min(x2));
            let x2_: usize = *(x1.max(x2));

            let y1_: usize = *(y1.min(y2));
            let y2_: usize = *(y1.max(y2));

            let mut distance = x1_.abs_diff(x2_) + y1_.abs_diff(y2_);

            for x in zero_cols.iter() {
                if *x > x1_ && *x < x2_ {
                    distance += scale - 1;
                }
            }

            for y in zero_rows.iter() {
                if *y > y1_ && *y < y2_ {
                    distance += scale - 1;
                }
            }

            result += distance;
        }
    }

    result
}

fn part1(input: &[u8]) -> usize {
    solution(input, 2)
}

fn part2(input: &[u8]) -> usize {
    solution(input, 1000000)
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day11.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(solution(input, 2), 374);
    }

    #[test]
    fn test_part2_scale_10() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(solution(input, 10), 1030);
    }

    #[test]
    fn test_part2_scale_100() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(solution(input, 100), 8410);
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
