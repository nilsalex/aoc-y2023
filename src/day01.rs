extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day01.txt");

const CANDIDATES: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

const REV_CANDIDATES: [&[u8]; 9] = [
    b"eno", b"owt", b"eerht", b"ruof", b"evif", b"xis", b"neves", b"thgie", b"enin",
];

fn get_first_in_line(line: &[u8], candidates: &[&[u8]]) -> u32 {
    let mut parser_state: [usize; 10] = [0; 10];
    for c in line {
        if c.is_ascii_digit() {
            return (c - b'0') as u32;
        }
        if candidates.is_empty() {
            continue;
        }
        for i in 0..=candidates.len() - 1 {
            if *c != candidates[i][parser_state[i]] {
                parser_state[i] = 0;
                if *c != candidates[i][0] {
                    continue;
                }
            }
            if parser_state[i] < candidates[i].len() {
                parser_state[i] += 1;
            }
            if parser_state[i] == candidates[i].len() {
                return (i + 1) as u32;
            }
        }
    }
    panic!();
}

pub fn solve(input: &[u8], fwd_candidates: &[&[u8]], rev_candidates: &[&[u8]]) -> u32 {
    let mut result = 0;

    input.split(|c| *c == b'\n').for_each(|line| {
        let mut line = Vec::from(line);
        let a = get_first_in_line(&line, fwd_candidates);

        line.reverse();
        let b = get_first_in_line(&line, rev_candidates);

        result += 10 * a + b;
    });

    result
}

pub fn part1(input: &[u8]) -> u32 {
    solve(input, &[], &[])
}

pub fn part2(input: &[u8]) -> u32 {
    solve(input, &CANDIDATES, &REV_CANDIDATES)
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

    const TEST_INPUT1: &[u8] = include_bytes!("../test_inputs/day01_1.txt");
    const TEST_INPUT2: &[u8] = include_bytes!("../test_inputs/day01_2.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT1.trim_ascii_end();
        assert_eq!(part1(input), 142);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT2.trim_ascii_end();
        assert_eq!(part2(input), 281);
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
