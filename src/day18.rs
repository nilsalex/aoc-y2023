extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day18.txt");

const POWERS_OF_TEN: [usize; 2] = [1, 10];

fn usize_from_bytes(bytes: &[u8]) -> usize {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, x)| {
        acc + (x & 0x0f) as usize * POWERS_OF_TEN[ix]
    })
}

fn part1(input: &[u8]) -> usize {
    let mut area_twice: isize = 0;
    let mut circumference: usize = 0;

    let mut pos = (0, 0);

    for line in input.split(|&b| b == b'\n') {
        let mut groups = line.split(|&b| b == b' ');
        let dir = groups.next().unwrap()[0];
        let steps = usize_from_bytes(groups.next().unwrap());

        let next = match dir {
            b'R' => (pos.0, pos.1 + steps as isize),
            b'L' => (pos.0, pos.1 - steps as isize),
            b'U' => (pos.0 - steps as isize, pos.1),
            b'D' => (pos.0 + steps as isize, pos.1),
            _ => panic!(),
        };

        area_twice += (pos.1 + next.1) * (pos.0 - next.0);
        circumference += steps;

        pos = next;
    }

    (area_twice.unsigned_abs() + circumference) / 2 + 1
}

fn part2(input: &[u8]) -> usize {
    let mut area_twice: isize = 0;
    let mut circumference: usize = 0;

    let mut pos = (0, 0);

    for line in input.split(|&b| b == b'\n') {
        let mut groups = line.split(|&b| b == b' ').skip(2);
        let code = groups.next().unwrap();

        let steps = usize::from_str_radix(std::str::from_utf8(&code[2..=6]).unwrap(), 16).unwrap();

        let next = match code[7] {
            b'0' => (pos.0, pos.1 + steps as isize),
            b'2' => (pos.0, pos.1 - steps as isize),
            b'3' => (pos.0 - steps as isize, pos.1),
            b'1' => (pos.0 + steps as isize, pos.1),
            _ => panic!(),
        };

        area_twice += (pos.1 + next.1) * (pos.0 - next.0);
        circumference += steps;

        pos = next;
    }

    (area_twice.unsigned_abs() + circumference) / 2 + 1
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day18.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 62);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 952408144115);
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
