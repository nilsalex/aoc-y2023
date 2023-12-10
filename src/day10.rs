extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day10.txt");

fn get_start_type(p: &(usize, usize), grid: &[Vec<u8>]) -> u8 {
    let (x, y) = *p;

    let up = matches!(grid[y - 1][x], b'|' | b'7' | b'F');
    let down = matches!(grid[y + 1][x], b'|' | b'L' | b'J');
    let left = matches!(grid[y][x - 1], b'-' | b'L' | b'F');
    let right = matches!(grid[y][x + 1], b'-' | b'7' | b'J');

    match (up, down, left, right) {
        (true, true, false, false) => b'|',
        (false, false, true, true) => b'-',
        (true, false, true, false) => b'J',
        (true, false, false, true) => b'L',
        (false, true, true, false) => b'7',
        (false, true, false, true) => b'F',
        _ => panic!(),
    }
}

fn get_adjacent(p: &(usize, usize), grid: &[Vec<u8>]) -> ((usize, usize), (usize, usize)) {
    let (x, y) = *p;
    match grid[p.1][p.0] {
        b'-' => ((x - 1, y), (x + 1, y)),
        b'|' => ((x, y - 1), (x, y + 1)),
        b'L' => ((x, y - 1), (x + 1, y)),
        b'F' => ((x, y + 1), (x + 1, y)),
        b'7' => ((x, y + 1), (x - 1, y)),
        b'J' => ((x, y - 1), (x - 1, y)),
        _ => panic!(),
    }
}

fn part1(input: &[u8]) -> u32 {
    let num_cols = input.iter().take_while(|c| **c != b'\n').count() + 2;
    let dummy_row: Vec<u8> = vec![b'.'; num_cols];

    let mut start = (0, 0);

    let mut grid: Vec<Vec<u8>> = std::iter::once(&dummy_row[..])
        .chain(input.split(|c| *c == b'\n'))
        .chain(std::iter::once(&dummy_row[..]))
        .enumerate()
        .map(|(y, line)| {
            let line = std::iter::once(&b'.')
                .chain(line.iter())
                .chain(std::iter::once(&b'.'))
                .enumerate()
                .map(|(x, c)| {
                    if *c == b'S' {
                        start = (x, y);
                    }
                    *c
                })
                .collect();
            line
        })
        .collect();

    grid[start.1][start.0] = get_start_type(&start, &grid);

    let mut length = 0;
    let mut p = start;
    let mut prev = (start.0 - 1, start.1);

    loop {
        let adj = get_adjacent(&p, &grid);

        if adj.0 == prev {
            prev = p;
            p = adj.1;
        } else {
            prev = p;
            p = adj.0;
        }

        length += 1;

        if p == start {
            return (length / 2) as u32;
        }
    }
}

fn part2(input: &[u8]) -> u32 {
    let num_cols = input.iter().take_while(|c| **c != b'\n').count() + 2;
    let dummy_row: Vec<u8> = vec![b'.'; num_cols];

    let mut start = (0, 0);

    let mut grid: Vec<Vec<u8>> = std::iter::once(&dummy_row[..])
        .chain(input.split(|c| *c == b'\n'))
        .chain(std::iter::once(&dummy_row[..]))
        .enumerate()
        .map(|(y, line)| {
            let line = std::iter::once(&b'.')
                .chain(line.iter())
                .chain(std::iter::once(&b'.'))
                .enumerate()
                .map(|(x, c)| {
                    if *c == b'S' {
                        start = (x, y);
                    }
                    *c
                })
                .collect();
            line
        })
        .collect();

    grid[start.1][start.0] = get_start_type(&start, &grid);

    let mut loop_grid: Vec<Vec<u8>> = vec![vec![b'.'; grid[0].len()]; grid.len()];

    let mut p = start;
    let mut prev = (start.0 - 1, start.1);

    loop {
        loop_grid[p.1][p.0] = grid[p.1][p.0];

        let adj = get_adjacent(&p, &grid);

        if adj.0 == prev {
            prev = p;
            p = adj.1;
        } else {
            prev = p;
            p = adj.0;
        }

        if p == start {
            break;
        }
    }

    let mut result = 0;

    for line in loop_grid {
        let mut crossing_count: u8 = 0;
        let mut entry = 0;
        for c in line {
            match c {
                b'.' => {
                    if crossing_count % 2 == 1 {
                        result += 1;
                    }
                }
                b'|' => {
                    crossing_count += 1;
                }
                b'-' => {}
                b'L' | b'F' => {
                    entry = c;
                }
                b'7' | b'J' => match (entry, c) {
                    (b'L', b'7') => crossing_count += 1,
                    (b'F', b'J') => crossing_count += 1,
                    (b'L', b'J') => {}
                    (b'F', b'7') => {}
                    _ => panic!(),
                },
                _ => panic!(),
            }
        }
    }

    result
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

    const TEST_INPUT_1: &[u8] = include_bytes!("../test_inputs/day10_1.txt");
    const TEST_INPUT_2: &[u8] = include_bytes!("../test_inputs/day10_2.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT_1.trim_ascii_end();
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT_2.trim_ascii_end();
        assert_eq!(part2(input), 10);
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
