extern crate test;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

const INPUT: &[u8] = include_bytes!("../inputs/day17.txt");

#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd, Ord)]
enum Dir {
    U = 0,
    D = 1,
    L = 2,
    R = 3,
}

struct Grid {
    cells: Vec<usize>,
    rows: usize,
    cols: usize,
    size: usize,
}

impl Grid {
    fn parse(input: &[u8]) -> Self {
        let cols = input.iter().take_while(|&&b| b != b'\n').count();
        let cells = input
            .iter()
            .cloned()
            .filter(|&b| b != b'\n')
            .map(|b| (b - b'0') as usize)
            .collect::<Vec<usize>>();
        let rows = cells.len() / cols;
        let size = rows * cols;

        Self {
            cells,
            rows,
            cols,
            size,
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn get_pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn is_in_bounds(&self, pos: &(isize, isize)) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.rows as isize && pos.1 < self.cols as isize
    }

    fn get_edges(&self, node: Node, min_steps: usize, max_steps: usize) -> Vec<Edge> {
        let mut result = Vec::new();

        let arms = match node.dir {
            Dir::U | Dir::D => [(Dir::R, (0, 1)), (Dir::L, (0, -1))],
            Dir::L | Dir::R => [(Dir::D, (1, 0)), (Dir::U, (-1, 0))],
        };
        for (dir, (delta_row, delta_col)) in arms {
            let mut cost = 0;
            for d in 1..=max_steps {
                let pos = (
                    node.row as isize + d as isize * delta_row,
                    node.col as isize + d as isize * delta_col,
                );

                if !self.is_in_bounds(&pos) {
                    break;
                }

                cost += self.cells[self.get_pos(pos.0 as usize, pos.1 as usize)];

                if d >= min_steps {
                    result.push(Edge {
                        node: Node {
                            row: pos.0 as usize,
                            col: pos.1 as usize,
                            dir,
                        },
                        cost,
                    });
                }
            }
        }

        result
    }

    fn get_node_index(&self, node: &Node) -> usize {
        4 * self.get_pos(node.row, node.col) + node.dir as usize
    }
}

#[derive(Debug)]
struct Edge {
    node: Node,
    cost: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    row: usize,
    col: usize,
    dir: Dir,
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct State {
    node: Node,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(
    grid: &Grid,
    start: Node,
    goal: Node,
    ultra: bool,
    adj_cache: &mut [Option<Vec<Edge>>],
) -> Option<usize> {
    let mut dist: Vec<usize> = vec![usize::MAX; 4 * grid.len()];
    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    dist[grid.get_node_index(&start)] = 0;
    heap.push(State {
        cost: 0,
        node: start,
    });

    while let Some(State { cost, node }) = heap.pop() {
        if node == goal {
            return Some(cost);
        }

        let node_index = grid.get_node_index(&node);
        if cost > dist[node_index] {
            continue;
        }

        let (min_steps, max_steps) = if ultra { (4, 10) } else { (1, 3) };

        let edges;
        if let Some(cached_edges) = &adj_cache[node_index] {
            edges = cached_edges;
        } else {
            adj_cache[node_index] = Some(grid.get_edges(node, min_steps, max_steps));
            if let Some(tmp_edges) = &adj_cache[node_index] {
                edges = tmp_edges
            } else {
                panic!();
            }
        }

        for edge in edges {
            let next = State {
                cost: cost + edge.cost,
                node: edge.node,
            };

            if next.cost < dist[grid.get_node_index(&next.node)] {
                heap.push(next);
                dist[grid.get_node_index(&next.node)] = next.cost;
            }
        }
    }

    None
}

fn solve(grid: &Grid, ultra: bool) -> usize {
    let mut result = usize::MAX;
    let mut adj_cache = (0..grid.size * 4).map(|_| None).collect::<Vec<_>>();

    for start_dir in [Dir::D, Dir::R] {
        for end_dir in [Dir::D, Dir::R] {
            let start = Node {
                row: 0,
                col: 0,
                dir: start_dir,
            };
            let goal = Node {
                row: grid.rows - 1,
                col: grid.cols - 1,
                dir: end_dir,
            };
            if let Some(dist) = shortest_path(grid, start, goal, ultra, &mut adj_cache) {
                result = std::cmp::min(result, dist);
            }
        }
    }

    result
}

fn part1(input: &[u8]) -> usize {
    let grid = Grid::parse(input);
    solve(&grid, false)
}

fn part2(input: &[u8]) -> usize {
    let grid = Grid::parse(input);
    solve(&grid, true)
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day17.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 102);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 94);
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
