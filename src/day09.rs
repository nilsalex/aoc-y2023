extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day09.txt");

fn parse_i64(num: &[u8]) -> i64 {
    num.iter()
        .fold(0, |acc, digit| acc * 10 + (digit & 0x0f) as i64)
}

fn solution(input: &[u8], reversed: bool) -> i64 {
    input
        .split(|c| *c == b'\n')
        .map(|line| {
            let mut nums: Vec<i64> = line
                .split(|c| *c == b' ')
                .map(|num| {
                    if num[0] == b'-' {
                        -parse_i64(&num[1..])
                    } else {
                        parse_i64(num)
                    }
                })
                .collect();
            if reversed {
                nums.reverse();
            }

            let mut result = 0;

            for i in (0..=nums.len() - 1).rev() {
                let mut all_zero = true;
                for j in 0..i {
                    nums[j] = nums[j + 1] - nums[j];
                    all_zero &= nums[j] == 0;
                }
                result += nums[i];
                if all_zero {
                    break;
                }
            }

            result
        })
        .sum()
}

fn part1(input: &[u8]) -> i64 {
    solution(input, false)
}

fn part2(input: &[u8]) -> i64 {
    solution(input, true)
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day09.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 114);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 2);
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
