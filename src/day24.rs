extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day24.txt");

const POWERS_OF_TEN: [usize; 16] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
];

fn i128_from_bytes(bytes: &[u8]) -> i128 {
    let sign;
    let digits;
    if bytes[0] == b'-' {
        sign = -1;
        digits = &bytes[1..]
    } else {
        sign = 1;
        digits = bytes;
    }

    sign * digits.iter().rev().enumerate().fold(0, |acc, (ix, x)| {
        acc + (x & 0x0f) as usize * POWERS_OF_TEN[ix]
    }) as i128
}

#[derive(Debug)]
struct Line2D {
    start: (i128, i128),
    dir: (i128, i128),
}

impl Line2D {
    fn parse(bytes: &[u8]) -> Self {
        let mut nums = bytes.split(|&b| b == b',' || b == b'@');

        let start = (
            i128_from_bytes(nums.next().unwrap().trim_ascii()),
            i128_from_bytes(nums.next().unwrap().trim_ascii()),
        );
        nums.next();

        let dir = (
            i128_from_bytes(nums.next().unwrap().trim_ascii()),
            i128_from_bytes(nums.next().unwrap().trim_ascii()),
        );

        Self { start, dir }
    }

    fn is_in_future(&self, x: f64) -> bool {
        if self.dir.0 > 0 {
            x > self.start.0 as f64
        } else {
            x < self.start.0 as f64
        }
    }

    fn intersection(&self, other: &Line2D) -> Option<(f64, f64)> {
        let d = self.dir.0 * other.dir.1 - self.dir.1 * other.dir.0;

        if d == 0 {
            return None;
        }

        Some((
            (-self.dir.1 * other.dir.0 * self.start.0 + self.dir.0 * other.dir.1 * other.start.0
                - self.dir.0 * other.dir.0 * (other.start.1 - self.start.1)) as f64
                / d as f64,
            (self.dir.0 * other.dir.1 * self.start.1 - self.dir.1 * other.dir.0 * other.start.1
                + self.dir.1 * other.dir.1 * (other.start.0 - self.start.0)) as f64
                / d as f64,
        ))
    }
}

#[derive(Debug)]
struct Line3D {
    start: (i128, i128, i128),
    dir: (i128, i128, i128),
}

impl Line3D {
    fn parse(bytes: &[u8]) -> Self {
        let mut nums = bytes.split(|&b| b == b',' || b == b'@');

        let start = (
            i128_from_bytes(nums.next().unwrap().trim_ascii()),
            i128_from_bytes(nums.next().unwrap().trim_ascii()),
            i128_from_bytes(nums.next().unwrap().trim_ascii()),
        );

        let dir = (
            i128_from_bytes(nums.next().unwrap().trim_ascii()),
            i128_from_bytes(nums.next().unwrap().trim_ascii()),
            i128_from_bytes(nums.next().unwrap().trim_ascii()),
        );

        Self { start, dir }
    }
}

fn part1(input: &[u8], min: isize, max: isize) -> usize {
    let lines = input
        .split(|&b| b == b'\n')
        .map(Line2D::parse)
        .collect::<Vec<_>>();

    let mut result = 0;

    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            if let Some((x, y)) = lines[i].intersection(&lines[j]) {
                if x >= min as f64
                    && x <= max as f64
                    && y >= min as f64
                    && y <= max as f64
                    && lines[i].is_in_future(x)
                    && lines[j].is_in_future(x)
                {
                    result += 1;
                }
            }
        }
    }

    result
}

fn part2(input: &[u8]) -> usize {
    let lines = input
        .split(|&b| b == b'\n')
        .take(3)
        .map(Line3D::parse)
        .collect::<Vec<_>>();

    let p1 = (
        lines[1].start.0 - lines[0].start.0,
        lines[1].start.1 - lines[0].start.1,
        lines[1].start.2 - lines[0].start.2,
    );
    let p2 = (
        lines[2].start.0 - lines[0].start.0,
        lines[2].start.1 - lines[0].start.1,
        lines[2].start.2 - lines[0].start.2,
    );

    let v1 = (
        lines[1].dir.0 - lines[0].dir.0,
        lines[1].dir.1 - lines[0].dir.1,
        lines[1].dir.2 - lines[0].dir.2,
    );
    let v2 = (
        lines[2].dir.0 - lines[0].dir.0,
        lines[2].dir.1 - lines[0].dir.1,
        lines[2].dir.2 - lines[0].dir.2,
    );

    print!("{{");
    print!("(y - x) * u - ({}) - ({}) * y,", p1.0, v1.0);
    print!("(y - x) * v - ({}) - ({}) * y,", p1.1, v1.1);
    print!("(y - x) * w - ({}) - ({}) * y,", p1.2, v1.2);

    print!("(z - x) * u - ({}) - ({}) * z,", p2.0, v2.0);
    print!("(z - x) * v - ({}) - ({}) * z,", p2.1, v2.1);
    print!("(z - x) * w - ({}) - ({}) * z", p2.2, v2.2);
    print!("}}");

    println!();

    // solved using wolfram alpha and groebner basis

    let x = 568483965344_i128;
    // let y = 547833135830 as i128;
    // let z = 728788861354 as i128;

    let u = -325_128;
    let v = -200_i128;
    let w = 247_i128;

    ((-x * u) + lines[0].start.0 + (-x * v) + lines[0].start.1 + (-x * w) + lines[0].start.2)
        as usize
}

pub fn main() {
    let input = INPUT.trim_ascii_end();

    println!("{}", part1(input, 200000000000000, 400000000000000));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day24.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input, 7, 27), 2);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 47);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = INPUT.trim_ascii_end();
        b.iter(|| part1(input, 200000000000000, 400000000000000))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = INPUT.trim_ascii_end();
        b.iter(|| part2(input))
    }
}
