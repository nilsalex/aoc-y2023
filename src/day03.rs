extern crate test;

use std::collections::HashSet;

const INPUT: &[u8] = include_bytes!("../inputs/input03.txt");

const POWERS_OF_TEN: [usize; 6] = [1, 10, 100, 1000, 10000, 100000];

fn usize_from_bytes(bytes: &[u8]) -> usize {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, x)| {
        acc + (x & 0x0f) as usize * POWERS_OF_TEN[ix]
    })
}

fn part1(input: &[u8]) -> usize {
    let symbols: HashSet<(isize, isize)> = input
        .split(|c| *c == b'\n')
        .enumerate()
        .flat_map(move |(row, line)| {
            line.iter()
                .enumerate()
                .flat_map(move |(col, elem)| match elem {
                    b'0'..=b'9' => None,
                    b'.' => None,
                    _ => Some((row as isize, col as isize)),
                })
        })
        .collect();

    let numbers: Vec<(isize, isize, isize, usize)> = input
        .split(|c| *c == b'\n')
        .enumerate()
        .flat_map(|(r, line)| {
            let line = Vec::from(line);
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
                let number_bytes = &line[i..]
                    .iter()
                    .take_while(|c| c.is_ascii_digit())
                    .cloned()
                    .collect::<Vec<u8>>();
                let length = number_bytes.len();
                result.push((
                    r as isize,
                    i as isize,
                    length as isize,
                    usize_from_bytes(number_bytes),
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

const DIRECTIONS: [(usize, usize); 8] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (1, 0),
    (1, 2),
    (2, 0),
    (2, 1),
    (2, 2),
];

fn part2(input: &[u8]) -> usize {
    let width = input.iter().take_while(|c| **c != b'\n').count();
    let dummy_line = vec![b'.'; width];

    let lines: Vec<Vec<u8>> = std::iter::once(&dummy_line[..])
        .chain(input.split(|c| *c == b'\n'))
        .chain(std::iter::once(&dummy_line[..]))
        .map(|line| [b".", line, b"."].concat())
        .collect();

    lines
        .windows(3)
        .flat_map(|window| {
            window[1]
                .iter()
                .enumerate()
                .flat_map(|(i, v)| if *v == b'*' { Some(i) } else { None })
                .flat_map(|i| {
                    let mut neighbours: Vec<bool> = DIRECTIONS
                        .into_iter()
                        .map(|(dy, dx)| window[dy][i + dx - 1].is_ascii_digit())
                        .collect();
                    if neighbours[0] && neighbours[1] {
                        neighbours[0] = false;
                    }
                    if neighbours[1] && neighbours[2] {
                        neighbours[2] = false;
                    }
                    if neighbours[5] && neighbours[6] {
                        neighbours[5] = false;
                    }
                    if neighbours[6] && neighbours[7] {
                        neighbours[7] = false;
                    }
                    let real_neighbours: Vec<(usize, usize)> =
                        std::iter::zip(neighbours, DIRECTIONS)
                            .flat_map(|(is_neigh, dir)| if is_neigh { Some(dir) } else { None })
                            .collect();
                    if real_neighbours.len() != 2 {
                        None
                    } else {
                        let (y1, x1) = (real_neighbours[0].0, i + real_neighbours[0].1 - 1);
                        let (y2, x2) = (real_neighbours[1].0, i + real_neighbours[1].1 - 1);

                        let left1 = window[y1][0..x1]
                            .iter()
                            .rev()
                            .take_while(|c| c.is_ascii_digit())
                            .count();
                        let right1 = window[y1][x1..]
                            .iter()
                            .take_while(|c| c.is_ascii_digit())
                            .count();

                        let left2 = window[y2][0..x2]
                            .iter()
                            .rev()
                            .take_while(|c| c.is_ascii_digit())
                            .count();
                        let right2 = window[y2][x2..]
                            .iter()
                            .take_while(|c| c.is_ascii_digit())
                            .count();

                        let a = usize_from_bytes(&window[y1][x1 - left1..x1 + right1]);
                        let b = usize_from_bytes(&window[y2][x2 - left2..x2 + right2]);

                        Some(a * b)
                    }
                })
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/input03.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 467835);
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
