extern crate test;

use std::collections::HashMap;

const INPUT: &[u8] = include_bytes!("../inputs/day14.txt");

fn load(grid: &[Vec<u8>]) -> usize {
    grid.iter()
        .rev()
        .enumerate()
        .skip(1)
        .map(|(i, line)| i * line.iter().filter(|&&b| b == b'O').count())
        .sum()
}

#[derive(Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
}

enum Either<A, B> {
    A(A),
    B(B),
}

impl<A, B> Iterator for Either<A, B>
where
    A: Iterator,
    B: Iterator<Item = A::Item>,
{
    type Item = A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Either::A(a) => a.next(),
            Either::B(b) => b.next(),
        }
    }
}

fn get_row_range(dir: Dir, num_rows: usize) -> impl Iterator<Item = usize> {
    match dir {
        Dir::North => Either::A(1..=num_rows - 2),
        Dir::South => Either::B((1..=num_rows - 2).rev()),
        Dir::West => Either::A(1..=num_rows - 2),
        Dir::East => Either::A(1..=num_rows - 2),
    }
}

fn get_col_range(dir: Dir, num_cols: usize) -> impl Iterator<Item = usize> {
    match dir {
        Dir::North => Either::A(1..=num_cols - 2),
        Dir::South => Either::A(1..=num_cols - 2),
        Dir::West => Either::A(1..=num_cols - 2),
        Dir::East => Either::B((1..=num_cols - 2).rev()),
    }
}

fn get_prev_and_cur(grid: &[Vec<u8>], dir: Dir, row: usize, col: usize) -> (u8, u8) {
    match dir {
        Dir::North => (grid[row - 1][col], grid[row][col]),
        Dir::South => (grid[row + 1][col], grid[row][col]),
        Dir::West => (grid[row][col - 1], grid[row][col]),
        Dir::East => (grid[row][col + 1], grid[row][col]),
    }
}

fn set_prev_and_cur(grid: &mut [Vec<u8>], dir: Dir, row: usize, col: usize, prev: u8, cur: u8) {
    match dir {
        Dir::North => {
            grid[row - 1][col] = cur;
            grid[row][col] = prev;
        }
        Dir::South => {
            grid[row + 1][col] = cur;
            grid[row][col] = prev;
        }
        Dir::West => {
            grid[row][col - 1] = cur;
            grid[row][col] = prev;
        }
        Dir::East => {
            grid[row][col + 1] = cur;
            grid[row][col] = prev;
        }
    }
}

fn tilt(grid: &mut [Vec<u8>], dir: Dir) {
    loop {
        let mut modified = false;

        for row in get_row_range(dir, grid.len()) {
            for col in get_col_range(dir, grid[0].len()) {
                let (prev, cur) = get_prev_and_cur(grid, dir, row, col);

                if prev == b'.' && cur == b'O' {
                    set_prev_and_cur(grid, dir, row, col, prev, cur);
                    modified = true;
                }
            }
        }

        if !modified {
            break;
        }
    }
}

fn cycle(grid: &mut [Vec<u8>]) {
    tilt(grid, Dir::North);
    tilt(grid, Dir::West);
    tilt(grid, Dir::South);
    tilt(grid, Dir::East);
}

fn part1(input: &[u8]) -> usize {
    let num_cols = input.iter().take_while(|&&b| b != b'\n').count();
    let mut grid: Vec<Vec<u8>> = std::iter::once(vec![b'#'; num_cols + 2])
        .chain(input.split(|&b| b == b'\n').map(|line| {
            std::iter::once(b'#')
                .chain(line.iter().cloned())
                .chain(std::iter::once(b'#'))
                .collect()
        }))
        .chain(std::iter::once(vec![b'#'; num_cols + 2]))
        .collect();

    tilt(&mut grid, Dir::North);

    load(&grid)
}

fn part2(input: &[u8]) -> usize {
    let num_cols = input.iter().take_while(|&&b| b != b'\n').count();
    let mut grid: Vec<Vec<u8>> = std::iter::once(vec![b'#'; num_cols + 2])
        .chain(input.split(|&b| b == b'\n').map(|line| {
            std::iter::once(b'#')
                .chain(line.iter().cloned())
                .chain(std::iter::once(b'#'))
                .collect()
        }))
        .chain(std::iter::once(vec![b'#'; num_cols + 2]))
        .collect();

    let mut first_occurences: HashMap<Vec<Vec<u8>>, usize> = HashMap::new();
    first_occurences.insert(grid.clone(), 0);

    let mut loads: Vec<usize> = Vec::new();
    loads.push(load(&grid));

    let mut n = 0;
    let cycle_start;
    let cycle_len;

    loop {
        n += 1;
        cycle(&mut grid);

        if let Some(first_occurence) = first_occurences.get(&grid) {
            cycle_start = *first_occurence;
            cycle_len = n - cycle_start;
            break;
        }

        first_occurences.insert(grid.clone(), n);
        loads.push(load(&grid));
    }

    /*
        N = cycle_start + k * cycle_len + l
        N - cycle_start = k * cycle_len + l
        k = (N - cycle_start) `div` cycle_len
        l = N - cycle_start - k * cycle_len
    */

    const N: usize = 1000000000;

    let k = (N - cycle_start) / cycle_len;
    let l = N - cycle_start - k * cycle_len;

    let n_reduced = cycle_start + l;

    loads[n_reduced]
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day14.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 136);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 64);
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
