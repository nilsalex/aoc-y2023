extern crate test;

use std::collections::BinaryHeap;
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

#[derive(Debug, Eq, PartialEq)]
struct State {
    pos: (isize, isize),
    cost: usize,
}

impl State {
    fn get_next(&self, grid: &Grid) -> Vec<State> {
        let mut neighbors = Vec::new();

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next_pos = (self.pos.0 + dx, self.pos.1 + dy);

            if grid.is_rock(&next_pos) {
                continue;
            }

            neighbors.push(State {
                pos: next_pos,
                cost: self.cost + 1,
            });
        }

        neighbors
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn count_reachable(grid: &Grid, steps: usize) -> usize {
    let mut dist = vec![usize::MAX; (2 * steps + 1) * (2 * steps + 1)];
    let mut heap = BinaryHeap::new();
    let (mut even, mut odd) = (0, 0);

    heap.push(State {
        pos: grid.start,
        cost: 0,
    });

    while let Some(state) = heap.pop() {
        let pos_index = (2 * steps + 1) * (state.pos.0 - grid.start.0 + steps as isize) as usize
            + (state.pos.1 - grid.start.1 + steps as isize) as usize;
        if state.cost > dist[pos_index] {
            continue;
        }

        for next in state.get_next(grid) {
            if next.cost > steps {
                continue;
            }

            let next_pos_index = (2 * steps + 1)
                * (next.pos.0 - grid.start.0 + steps as isize) as usize
                + (next.pos.1 - grid.start.1 + steps as isize) as usize;
            if next.cost < dist[next_pos_index] {
                dist[next_pos_index] = next.cost;
                heap.push(next);
            }
        }

        if state.cost % 2 == 0 {
            even += 1;
        } else {
            odd += 1;
        }
    }

    if steps % 2 == 0 {
        even - 1
    } else {
        odd
    }
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
