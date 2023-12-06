extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day06.txt");

fn collect_as_u32<'a>(it: impl Iterator<Item = &'a u8>) -> u32 {
    it.fold(0, |acc, c| acc * 10 + (c - b'0') as u32)
}

fn collect_as_usize<'a>(it: impl Iterator<Item = &'a u8>) -> usize {
    it.fold(0, |acc, c| acc * 10 + (c - b'0') as usize)
}

fn part1(input: &[u8]) -> usize {
    let mut lines = input.split(|c| *c == b'\n');
    let durations = lines
        .next()
        .unwrap()
        .split(|c| *c == b' ')
        .skip(1)
        .filter(|group| !group.is_empty())
        .map(|num| collect_as_u32(num.iter()));
    let times = lines
        .next()
        .unwrap()
        .split(|c| *c == b' ')
        .skip(1)
        .filter(|group| !group.is_empty())
        .map(|num| collect_as_u32(num.iter()));

    std::iter::zip(durations, times)
        .map(|(t, s)| {
            let d = ((t * t - 4 * s) as f64).sqrt();
            let t = t as f64;
            ((t + d - 2.0) / 2.0).ceil() as usize - ((t - d + 2.0) / 2.0).floor() as usize + 1
        })
        .product()
}

fn part2(input: &[u8]) -> usize {
    let mut lines = input.split(|c| *c == b'\n');
    let t: usize = collect_as_usize(lines.next().unwrap().iter().filter(|c| c.is_ascii_digit()));
    let s: usize = collect_as_usize(lines.next().unwrap().iter().filter(|c| c.is_ascii_digit()));

    let d = ((t * t - 4 * s) as f64).sqrt();
    let t = t as f64;

    let a = (t - d + 2.0) / 2.0;
    let b = (t + d - 2.0) / 2.0;

    b.ceil() as usize - a.floor() as usize + 1
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day06.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 288);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 71503);
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
