extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/input02.txt");

const POWERS_OF_TEN: [u32; 3] = [1, 10, 100];

fn u32_from_bytes(bytes: &[u8]) -> u32 {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, x)| {
        acc + (x & 0x0f) as u32 * POWERS_OF_TEN[ix]
    })
}

fn part1(input: &[u8]) -> u32 {
    input
        .split(|c| *c == b'\n')
        .flat_map(|line| {
            let mut entries = line.split(|c| *c == b':' || *c == b';' || *c == b',');
            let index_bytes = entries
                .next()
                .unwrap()
                .split(|c| *c == b' ')
                .nth(1)
                .unwrap();
            let index = u32_from_bytes(index_bytes);
            for entry in entries {
                let mut group = entry[1..].split(|c| *c == b' ');
                let number_bytes = group.next().unwrap();
                let number = u32_from_bytes(number_bytes);
                let color = group.next().unwrap();
                match color {
                    b"red" => {
                        if number > 12 {
                            return None;
                        }
                    }
                    b"green" => {
                        if number > 13 {
                            return None;
                        }
                    }
                    b"blue" => {
                        if number > 14 {
                            return None;
                        }
                    }
                    _ => panic!(),
                };
            }
            Some(index)
        })
        .sum()
}

fn part2(input: &[u8]) -> u32 {
    input
        .split(|c| *c == b'\n')
        .map(|line| {
            let entries = line.split(|c| *c == b':' || *c == b';' || *c == b',');
            let (r, g, b) = entries.skip(1).fold((0, 0, 0), |(r, g, b), entry| {
                let mut group = entry[1..].split(|c| *c == b' ');
                let number_bytes = group.next().unwrap();
                let number = u32_from_bytes(number_bytes);
                let color = group.next().unwrap();
                match color {
                    b"red" => (std::cmp::max(r, number), g, b),
                    b"green" => (r, std::cmp::max(g, number), b),
                    b"blue" => (r, g, std::cmp::max(b, number)),
                    _ => panic!(),
                }
            });
            r * g * b
        })
        .sum()
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/input02.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 2286);
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
