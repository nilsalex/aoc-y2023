extern crate test;

use std::collections::HashMap;

const INPUT: &[u8] = include_bytes!("../inputs/day23.txt");

fn is_path(byte: u8) -> bool {
    matches!(byte, b'<' | b'>' | b'^' | b'v' | b'.')
}

fn trace_path(
    grid: &[Vec<u8>],
    nodes: &[(usize, usize)],
    start: (usize, usize),
    next: (usize, usize),
) -> Option<(usize, usize)> {
    if grid[next.0][next.1] == b'#' {
        return None;
    }
    if next.0 < start.0 && grid[next.0][next.1] == b'v' {
        return None;
    }
    if next.0 > start.0 && grid[next.0][next.1] == b'^' {
        return None;
    }
    if next.1 < start.1 && grid[next.0][next.1] == b'>' {
        return None;
    }
    if next.1 > start.1 && grid[next.0][next.1] == b'<' {
        return None;
    }
    let mut previous = start;
    let mut current = next;
    let mut length = 1;

    loop {
        length += 1;
        for (candidate, arrow) in [
            ((current.0 - 1, current.1), b'v'),
            ((current.0 + 1, current.1), b'^'),
            ((current.0, current.1 - 1), b'>'),
            ((current.0, current.1 + 1), b'<'),
        ] {
            if candidate == previous {
                continue;
            }
            if let Ok(index) = nodes.binary_search(&candidate) {
                return Some((index, length));
            }
            if is_path(grid[candidate.0][candidate.1]) {
                if grid[candidate.0][candidate.1] == arrow {
                    return None;
                }
                previous = current;
                current = candidate;
                break;
            }
        }
    }
}

fn part1(input: &[u8]) -> usize {
    let grid = input
        .split(|&byte| byte == b'\n')
        .map(|line| line.to_vec())
        .collect::<Vec<_>>();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut nodes = Vec::new();
    nodes.push((0, 1));

    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            if is_path(grid[r][c]) {
                let mut count = 0;
                if is_path(grid[r - 1][c]) {
                    count += 1;
                }
                if is_path(grid[r + 1][c]) {
                    count += 1;
                }
                if is_path(grid[r][c - 1]) {
                    count += 1;
                }
                if is_path(grid[r][c + 1]) {
                    count += 1;
                }
                if count >= 3 {
                    nodes.push((r, c));
                }
            }
        }
    }

    nodes.push((rows - 1, cols - 2));

    let goal_node = nodes.len() - 1;

    let mut adj = HashMap::new();
    adj.insert(0, vec![trace_path(&grid, &nodes, (0, 1), (1, 1)).unwrap()]);

    for (index, node) in nodes[0..nodes.len() - 1].iter().enumerate().skip(1) {
        let mut neighbours = Vec::new();
        for next in [
            (node.0 - 1, node.1),
            (node.0 + 1, node.1),
            (node.0, node.1 - 1),
            (node.0, node.1 + 1),
        ] {
            if let Some((neigh_index, length)) = trace_path(&grid, &nodes, *node, next) {
                neighbours.push((neigh_index, length));
            }
        }
        adj.insert(index, neighbours);
    }

    let mut max_distance = 0;
    let mut stack = Vec::new();
    stack.push((0, 0, 0_usize));
    while let Some((node, distance, visited)) = stack.pop() {
        if visited & (1 << goal_node) != 0 {
            max_distance = max_distance.max(distance);
            continue;
        }
        if let Some(neighs) = adj.get(&node) {
            for (neigh, length) in neighs {
                if visited & (1 << neigh) == 0 {
                    stack.push((*neigh, distance + length, visited | (1 << neigh)));
                }
            }
        }
    }

    max_distance
}

fn part2(input: &[u8]) -> usize {
    0
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day23.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 94);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 0);
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
