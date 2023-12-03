extern crate test;

const INPUT: &str = include_str!("../inputs/input02.txt");

fn part1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| {
            let mut entries = line.split([':', ';', ',']);
            let index = entries
                .next()
                .unwrap()
                .split(' ')
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            for entry in entries {
                let mut group = entry[1..].split(' ');
                let number = group.next().unwrap().parse::<usize>().unwrap();
                let color = group.next().unwrap();
                match color {
                    "red" => {
                        if number > 12 {
                            return None;
                        }
                    }
                    "green" => {
                        if number > 13 {
                            return None;
                        }
                    }
                    "blue" => {
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

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let entries = line.split([':', ';', ',']);
            let (r, g, b) = entries.skip(1).fold((0, 0, 0), |(r, g, b), entry| {
                let mut group = entry[1..].split(' ');
                let number = group.next().unwrap().parse::<usize>().unwrap();
                let color = group.next().unwrap();
                match color {
                    "red" => (std::cmp::max(r, number), g, b),
                    "green" => (r, std::cmp::max(g, number), b),
                    "blue" => (r, g, std::cmp::max(b, number)),
                    _ => panic!(),
                }
            });
            r * g * b
        })
        .sum()
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

    const TEST_INPUT: &str = include_str!("../test_inputs/input02.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_end();
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_end();
        assert_eq!(part2(input), 2286);
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
