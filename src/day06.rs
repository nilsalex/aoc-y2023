extern crate test;

const INPUT: &str = include_str!("../inputs/day06.txt");

fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let durations: Vec<f64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|num| num.parse().unwrap())
        .collect();
    let times: Vec<f64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|num| num.parse().unwrap())
        .collect();

    std::iter::zip(durations, times)
        .map(|(t, s)| {
            (
                (t - (t * t - 4.0 * s).sqrt()) / 2.0,
                (t + (t * t - 4.0 * s).sqrt()) / 2.0,
            )
        })
        .map(|(a, b)| (b - 1.0).ceil() as usize - (a + 1.0).floor() as usize + 1)
        .product()
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let t: f64 = (lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>())
    .parse()
    .unwrap();
    let s: f64 = (lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>())
    .parse()
    .unwrap();

    let a = (t - (t * t - 4.0 * s).sqrt()) / 2.0;
    let b = (t + (t * t - 4.0 * s).sqrt()) / 2.0;

    (b - 1.0).ceil() as usize - (a + 1.0).floor() as usize + 1
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

    const TEST_INPUT: &str = include_str!("../test_inputs/day06.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_end();
        assert_eq!(part1(input), 288);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_end();
        assert_eq!(part2(input), 71503);
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
