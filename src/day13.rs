extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day13.txt");

fn solve_pattern(pattern: &[Vec<u8>]) -> usize {
    let transposed: Vec<Vec<u8>> = (0..=pattern[0].len() - 1)
        .map(|x| {
            (0..=pattern.len() - 1)
                .map(|y| pattern[y][x])
                .collect::<Vec<u8>>()
        })
        .collect();

    for i in 1..=pattern.len() - 1 {
        let n = std::cmp::min(i, pattern.len() - i);

        if pattern[i - n..=i - 1]
            .iter()
            .rev()
            .eq(pattern[i..=i + n - 1].iter())
        {
            return 100 * i;
        }
    }

    for i in 1..=transposed.len() - 1 {
        let n = std::cmp::min(i, transposed.len() - i);

        if transposed[i - n..=i - 1]
            .iter()
            .rev()
            .eq(transposed[i..=i + n - 1].iter())
        {
            return i;
        }
    }

    panic!()
}

fn solve_pattern_fuzzy(pattern: &[Vec<u8>]) -> usize {
    let transposed: Vec<Vec<u8>> = (0..=pattern[0].len() - 1)
        .map(|x| {
            (0..=pattern.len() - 1)
                .map(|y| pattern[y][x])
                .collect::<Vec<u8>>()
        })
        .collect();

    for i in 1..=pattern.len() - 1 {
        let n = std::cmp::min(i, pattern.len() - i);

        let mut diff = 0;
        'outer: for (line1, line2) in pattern[i - n..=i - 1]
            .iter()
            .rev()
            .zip(pattern[i..=i + n - 1].iter())
        {
            for (&a, &b) in line1.iter().zip(line2.iter()) {
                if a != b {
                    diff += 1;
                    if diff > 1 {
                        break 'outer;
                    }
                }
            }
        }

        if diff == 1 {
            return 100 * i;
        }
    }

    for i in 1..=transposed.len() - 1 {
        let n = std::cmp::min(i, transposed.len() - i);

        let mut diff = 0;
        'outer: for (line1, line2) in transposed[i - n..=i - 1]
            .iter()
            .rev()
            .zip(transposed[i..=i + n - 1].iter())
        {
            for (&a, &b) in line1.iter().zip(line2.iter()) {
                if a != b {
                    diff += 1;
                    if diff > 1 {
                        break 'outer;
                    }
                }
            }
        }

        if diff == 1 {
            return i;
        }
    }

    panic!()
}

fn part1(input: &[u8]) -> usize {
    let mut lines = input.split(|&c| c == b'\n');

    let mut result = 0;

    let mut pattern: Vec<Vec<u8>> = Vec::new();
    loop {
        if let Some(line) = lines.next() {
            if line.is_empty() {
                result += solve_pattern(&pattern);
                pattern.clear();
                continue;
            }

            pattern.push(Vec::from(line));
        } else {
            result += solve_pattern(&pattern);
            break;
        }
    }

    result
}

fn part2(input: &[u8]) -> usize {
    let mut lines = input.split(|&c| c == b'\n');

    let mut result = 0;

    let mut pattern: Vec<Vec<u8>> = Vec::new();
    loop {
        if let Some(line) = lines.next() {
            if line.is_empty() {
                result += solve_pattern_fuzzy(&pattern);
                pattern.clear();
                continue;
            }

            pattern.push(Vec::from(line));
        } else {
            result += solve_pattern_fuzzy(&pattern);
            break;
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day13.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 405);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 400);
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
