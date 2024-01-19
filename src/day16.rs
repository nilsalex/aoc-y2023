extern crate test;

use std::collections::VecDeque;

const INPUT: &[u8] = include_bytes!("../inputs/day16.txt");

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Gate {
    SplitHorizontal,
    SplitVertical,
    MirrorDown,
    MirrorUp,
}

impl Gate {
    fn parse(char: u8) -> Option<Gate> {
        match char {
            b'|' => Some(Self::SplitVertical),
            b'-' => Some(Self::SplitHorizontal),
            b'\\' => Some(Self::MirrorDown),
            b'/' => Some(Self::MirrorUp),
            _ => None,
        }
    }

    fn apply(&self, dir: Dir) -> Vec<Dir> {
        match self {
            Self::SplitHorizontal => match dir {
                Dir::Up => vec![Dir::Left, Dir::Right],
                Dir::Down => vec![Dir::Left, Dir::Right],
                Dir::Left => vec![Dir::Left],
                Dir::Right => vec![Dir::Right],
            },
            Self::SplitVertical => match dir {
                Dir::Up => vec![Dir::Up],
                Dir::Down => vec![Dir::Down],
                Dir::Left => vec![Dir::Up, Dir::Down],
                Dir::Right => vec![Dir::Up, Dir::Down],
            },
            Self::MirrorDown => match dir {
                Dir::Up => vec![Dir::Left],
                Dir::Down => vec![Dir::Right],
                Dir::Left => vec![Dir::Up],
                Dir::Right => vec![Dir::Down],
            },
            Self::MirrorUp => match dir {
                Dir::Up => vec![Dir::Right],
                Dir::Down => vec![Dir::Left],
                Dir::Left => vec![Dir::Down],
                Dir::Right => vec![Dir::Up],
            },
        }
    }
}

#[derive(Clone)]
struct Grid {
    rows: usize,
    cols: usize,
    gates: Vec<Option<Gate>>,
    dirs: Vec<u8>,
}

impl Grid {
    fn parse(input: &[u8]) -> Self {
        let gates = input
            .iter()
            .filter(|&&b| b != b'\n')
            .map(|&b| Gate::parse(b))
            .collect::<Vec<Option<Gate>>>();
        let dirs = vec![0; gates.len()];
        let cols = input.iter().take_while(|&&b| b != b'\n').count();
        let rows = gates.len() / cols;

        Self {
            rows,
            cols,
            gates,
            dirs,
        }
    }

    fn get_gate(&self, row: usize, col: usize) -> Option<Gate> {
        self.gates[row * self.cols + col]
    }

    fn dir_to_byte(dir: Dir) -> u8 {
        match dir {
            Dir::Up => 0b0001,
            Dir::Down => 0b0010,
            Dir::Left => 0b0100,
            Dir::Right => 0b1000,
        }
    }

    fn set_dir(&mut self, row: usize, col: usize, dir: Dir) {
        self.dirs[row * self.cols + col] |= Self::dir_to_byte(dir)
    }

    fn check_dir(&self, row: usize, col: usize, dir: Dir) -> bool {
        self.dirs[row * self.cols + col] & Self::dir_to_byte(dir) != 0
    }

    fn get_energized(&self) -> usize {
        self.dirs.iter().filter(|&&b| b != 0).count()
    }

    fn next_tile(&self, row: usize, col: usize, dir: Dir) -> Option<(usize, usize)> {
        match dir {
            Dir::Up => {
                if row > 0 {
                    Some((row - 1, col))
                } else {
                    None
                }
            }
            Dir::Down => {
                if row + 1 < self.rows {
                    Some((row + 1, col))
                } else {
                    None
                }
            }
            Dir::Left => {
                if col > 0 {
                    Some((row, col - 1))
                } else {
                    None
                }
            }
            Dir::Right => {
                if col + 1 < self.cols {
                    Some((row, col + 1))
                } else {
                    None
                }
            }
        }
    }

    fn trace(&mut self, row: usize, col: usize, dir: Dir) {
        let mut queue = VecDeque::new();

        queue.push_back((row, col, dir));

        while let Some((row, col, dir)) = queue.pop_front() {
            if self.check_dir(row, col, dir) {
                continue;
            }

            self.set_dir(row, col, dir);

            let next_dirs = match self.get_gate(row, col) {
                None => vec![dir],
                Some(gate) => gate.apply(dir),
            };

            for &next_dir in next_dirs.iter() {
                if let Some((row, col)) = self.next_tile(row, col, next_dir) {
                    queue.push_back((row, col, next_dir));
                }
            }
        }
    }
}

fn part1(input: &[u8]) -> usize {
    let mut grid = Grid::parse(input);

    grid.trace(0, 0, Dir::Right);

    grid.get_energized()
}

fn part2(input: &[u8]) -> usize {
    let grid = Grid::parse(input);

    let mut values = Vec::new();

    for col in 0..=grid.cols - 1 {
        values.push((0, col, Dir::Down));
        values.push((grid.rows - 1, col, Dir::Up));
    }

    for row in 0..=grid.rows - 1 {
        values.push((row, 0, Dir::Right));
        values.push((row, grid.cols - 1, Dir::Left));
    }

    values
        .iter()
        .map(|&(row, col, dir)| {
            let mut grid = grid.clone();
            grid.trace(row, col, dir);
            grid.get_energized()
        })
        .max()
        .unwrap()
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day16.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 46);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 51);
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
