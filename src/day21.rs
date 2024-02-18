extern crate test;

use std::collections::HashSet;

const INPUT: &[u8] = include_bytes!("../inputs/day21.txt");

#[derive(Debug)]
struct Grid {
    rocks: HashSet<(isize, isize)>,
    start: (isize, isize),
    rows: usize,
    cols: usize,
}

impl Grid {
    fn parse(input: &[u8]) -> Self {
        let mut rocks = HashSet::new();
        let mut start = (0, 0);

        for (row, line) in input.split(|&b| b == b'\n').enumerate() {
            for (col, byte) in line.iter().enumerate() {
                match byte {
                    b'S' => {
                        start = (row as isize, col as isize);
                    }
                    b'#' => {
                        rocks.insert((row as isize, col as isize));
                    }
                    _ => {}
                }
            }
        }

        let rows = input.split(|&b| b == b'\n').count();
        let cols = input.iter().take_while(|&&b| b != b'\n').count();

        Self {
            rocks,
            start,
            rows,
            cols,
        }
    }

    fn is_rock(&self, pos: &(isize, isize)) -> bool {
        self.rocks.contains(&(
            pos.0.rem_euclid(self.rows as isize),
            pos.1.rem_euclid(self.cols as isize),
        ))
    }
}

fn count_reachable(grid: &Grid, steps: usize) -> usize {
    let mut reachable = HashSet::from([grid.start]);

    for _ in 0..steps {
        let mut next_reachable = HashSet::new();

        for &(row, col) in reachable.iter() {
            for next in [
                (row - 1, col),
                (row + 1, col),
                (row, col - 1),
                (row, col + 1),
            ] {
                if !grid.is_rock(&next) {
                    next_reachable.insert(next);
                }
            }
        }

        std::mem::swap(&mut reachable, &mut next_reachable);
    }

    reachable.len()
}

fn get_coefficients(f0: isize, f1: isize, f2: isize) -> (isize, isize, isize) {
    let a = (f0 - 2 * f1 + f2) / 2;
    let b = (-3 * f0 + 4 * f1 - f2) / 2;
    let c = f0;

    (a, b, c)
}

fn part1(input: &[u8], steps: usize) -> usize {
    let grid = Grid::parse(input);
    count_reachable(&grid, steps)
}

fn part2(input: &[u8]) -> usize {
    let grid = Grid::parse(input);
    let half_dist = (grid.rows - 1) / 2;

    let f0 = count_reachable(&grid, half_dist);
    let f1 = count_reachable(&grid, grid.rows + half_dist);
    let f2 = count_reachable(&grid, 2 * grid.rows + half_dist);

    let (a, b, c) = get_coefficients(f0 as isize, f1 as isize, f2 as isize);

    let steps = 26501365;
    let num_iterations = ((steps - half_dist) / grid.rows) as isize;

    (a * num_iterations * num_iterations + b * num_iterations + c) as usize
}

pub fn main() {
    let input = INPUT.trim_ascii_end();

    println!("{}", part1(input, 64));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day21.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input, 1), 2);
        assert_eq!(part1(input, 2), 4);
        assert_eq!(part1(input, 3), 6);
        assert_eq!(part1(input, 4), 9);
        assert_eq!(part1(input, 5), 13);
        assert_eq!(part1(input, 6), 16);
    }

    #[test]
    fn test_part2() {
        let f = |x: isize| (2 * x * x + 4 * x + 9);
        let g = |x: isize| ((-7) * x * x + 22 * x - 1);
        assert_eq!(get_coefficients(f(0), f(1), f(2)), (2, 4, 9));
        assert_eq!(get_coefficients(g(0), g(1), g(2)), (-7, 22, -1));
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = INPUT.trim_ascii_end();
        b.iter(|| part1(input, 64))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = INPUT.trim_ascii_end();
        b.iter(|| part2(input))
    }
}
