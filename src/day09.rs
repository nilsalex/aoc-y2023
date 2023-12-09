extern crate test;

const INPUT: &str = include_str!("../inputs/day09.txt");

fn solution(input: &str, reversed: bool) -> isize {
    input
        .lines()
        .map(|line| {
            let mut nums: Vec<isize> = line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            if reversed {
                nums.reverse();
            }

            let mut last_nums: Vec<isize> = vec![];

            loop {
                let mut all_zero = true;
                for i in 0..nums.len() - 1 {
                    nums[i] = nums[i + 1] - nums[i];
                    if nums[i] != 0 {
                        all_zero = false;
                    }
                }
                last_nums.push(nums.pop().unwrap());
                if all_zero {
                    break;
                }
            }

            last_nums.into_iter().sum::<isize>()
        })
        .sum()
}

fn part1(input: &str) -> isize {
    solution(input, false)
}

fn part2(input: &str) -> isize {
    solution(input, true)
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

    const TEST_INPUT: &str = include_str!("../test_inputs/day09.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_end();
        assert_eq!(part1(input), 114);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_end();
        assert_eq!(part2(input), 2);
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
