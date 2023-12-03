extern crate test;

use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &str = include_str!("../inputs/input03.txt");

fn part1(input: &str) -> usize {
    let symbols: HashSet<(isize, isize)> = input
        .lines()
        .enumerate()
        .flat_map(move |(row, line)| {
            line.chars()
                .enumerate()
                .flat_map(move |(col, elem)| match elem {
                    '0'..='9' => None,
                    '.' => None,
                    _ => Some((row as isize, col as isize)),
                })
        })
        .collect();

    let numbers: Vec<(isize, isize, isize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            let line = line.chars().collect::<Vec<char>>();
            let mut result = vec![];
            let mut i: usize = 0;
            loop {
                if i >= line.len() {
                    break;
                }
                if !line[i].is_ascii_digit() {
                    i += 1;
                    continue;
                }
                let number = &line[i..]
                    .iter()
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>();
                let length = number.len();
                result.push((
                    r as isize,
                    i as isize,
                    length as isize,
                    number.parse::<usize>().unwrap(),
                ));
                i += length;
            }
            result
        })
        .collect();

    numbers
        .iter()
        .filter_map(|(row, col_, length, number)| {
            for col in *col_..=*col_ + *length - 1 {
                for indices in [
                    (*row - 1, col - 1),
                    (*row - 1, col),
                    (*row - 1, col + 1),
                    (*row, col - 1),
                    (*row, col + 1),
                    (*row + 1, col - 1),
                    (*row + 1, col),
                    (*row + 1, col + 1),
                ] {
                    if symbols.contains(&indices) {
                        return Some(number);
                    }
                }
            }
            None
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut gears: HashMap<(isize, isize), Vec<usize>> = input
        .lines()
        .enumerate()
        .flat_map(move |(row, line)| {
            line.chars().enumerate().flat_map(move |(col, elem)| {
                if elem == '*' {
                    Some(((row as isize, col as isize), vec![]))
                } else {
                    None
                }
            })
        })
        .collect();

    let numbers: Vec<(isize, isize, isize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            let line = line.chars().collect::<Vec<char>>();
            let mut result = vec![];
            let mut i: usize = 0;
            loop {
                if i >= line.len() {
                    break;
                }
                if !line[i].is_ascii_digit() {
                    i += 1;
                    continue;
                }
                let number = &line[i..]
                    .iter()
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>();
                let length = number.len();
                result.push((
                    r as isize,
                    i as isize,
                    length as isize,
                    number.parse::<usize>().unwrap(),
                ));
                i += length;
            }
            result
        })
        .collect();

    numbers.iter().for_each(|(row, col_, length, number)| {
        'outer: for col in *col_..=*col_ + *length - 1 {
            for indices in [
                (*row - 1, col - 1),
                (*row - 1, col),
                (*row - 1, col + 1),
                (*row, col - 1),
                (*row, col + 1),
                (*row + 1, col - 1),
                (*row + 1, col),
                (*row + 1, col + 1),
            ] {
                if let Some(num_list) = gears.get_mut(&indices) {
                    num_list.push(*number);
                    break 'outer;
                }
            }
        }
    });

    gears
        .values()
        .flat_map(|nums| {
            if nums.len() == 2 {
                Some(nums[0] * nums[1])
            } else {
                None
            }
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

    const TEST_INPUT: &str = include_str!("../test_inputs/input03.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_end();
        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_end();
        assert_eq!(part2(input), 467835);
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
