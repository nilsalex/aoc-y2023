extern crate test;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

const INPUT: &[u8] = include_bytes!("../inputs/day17.txt");

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

fn get_range(start: isize, end: isize) -> impl Iterator<Item = isize> {
    if start < end {
        Either::A(start..=end)
    } else {
        Either::B(end..=start)
    }
}

#[derive(Eq, PartialEq, Copy, Clone, PartialOrd, Ord)]
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

    fn get_edges(&self, node: Node) -> Vec<Edge> {
        let this_cost = self.cells[self.get_pos(node.row, node.col)];

        let (row_i, col_i) = (node.row as isize, node.col as isize);

        let candidates = match node.dir {
            Dir::U | Dir::D => [
                ((row_i, col_i - 3), Dir::L),
                ((row_i, col_i - 2), Dir::L),
                ((row_i, col_i - 1), Dir::L),
                ((row_i, col_i + 1), Dir::R),
                ((row_i, col_i + 2), Dir::R),
                ((row_i, col_i + 3), Dir::R),
            ],
            Dir::L | Dir::R => [
                ((row_i - 3, col_i), Dir::U),
                ((row_i - 2, col_i), Dir::U),
                ((row_i - 1, col_i), Dir::U),
                ((row_i + 1, col_i), Dir::D),
                ((row_i + 2, col_i), Dir::D),
                ((row_i + 3, col_i), Dir::D),
            ],
        };

        candidates
            .into_iter()
            .filter(|((row_i, col_i), _)| {
                *row_i >= 0
                    && *row_i < self.rows as isize
                    && *col_i >= 0
                    && *col_i < self.cols as isize
            })
            .map(|((next_row_i, next_col_i), dir)| {
                let mut cost = 0;

                for r in get_range(row_i, next_row_i) {
                    for c in get_range(col_i, next_col_i) {
                        cost += self.cells[self.get_pos(r as usize, c as usize)];
                    }
                }

                cost -= this_cost;

                Edge {
                    node: Node {
                        row: next_row_i as usize,
                        col: next_col_i as usize,
                        dir,
                    },
                    cost,
                }
            })
            .collect::<Vec<Edge>>()
    }

    fn get_edges_ultra(&self, node: Node) -> Vec<Edge> {
        let this_cost = self.cells[self.get_pos(node.row, node.col)];

        let (row_i, col_i) = (node.row as isize, node.col as isize);

        let candidates = match node.dir {
            Dir::U | Dir::D => [
                ((row_i, col_i - 10), Dir::L),
                ((row_i, col_i - 9), Dir::L),
                ((row_i, col_i - 8), Dir::L),
                ((row_i, col_i - 7), Dir::L),
                ((row_i, col_i - 6), Dir::L),
                ((row_i, col_i - 5), Dir::L),
                ((row_i, col_i - 4), Dir::L),
                ((row_i, col_i + 4), Dir::R),
                ((row_i, col_i + 5), Dir::R),
                ((row_i, col_i + 6), Dir::R),
                ((row_i, col_i + 7), Dir::R),
                ((row_i, col_i + 8), Dir::R),
                ((row_i, col_i + 9), Dir::R),
                ((row_i, col_i + 10), Dir::R),
            ],
            Dir::L | Dir::R => [
                ((row_i - 10, col_i), Dir::U),
                ((row_i - 9, col_i), Dir::U),
                ((row_i - 8, col_i), Dir::U),
                ((row_i - 7, col_i), Dir::U),
                ((row_i - 6, col_i), Dir::U),
                ((row_i - 5, col_i), Dir::U),
                ((row_i - 4, col_i), Dir::U),
                ((row_i + 4, col_i), Dir::D),
                ((row_i + 5, col_i), Dir::D),
                ((row_i + 6, col_i), Dir::D),
                ((row_i + 7, col_i), Dir::D),
                ((row_i + 8, col_i), Dir::D),
                ((row_i + 9, col_i), Dir::D),
                ((row_i + 10, col_i), Dir::D),
            ],
        };

        candidates
            .into_iter()
            .filter(|((row_i, col_i), _)| {
                *row_i >= 0
                    && *row_i < self.rows as isize
                    && *col_i >= 0
                    && *col_i < self.cols as isize
            })
            .map(|((next_row_i, next_col_i), dir)| {
                let mut cost = 0;

                for r in get_range(row_i, next_row_i) {
                    for c in get_range(col_i, next_col_i) {
                        cost += self.cells[self.get_pos(r as usize, c as usize)];
                    }
                }

                cost -= this_cost;

                Edge {
                    node: Node {
                        row: next_row_i as usize,
                        col: next_col_i as usize,
                        dir,
                    },
                    cost,
                }
            })
            .collect::<Vec<Edge>>()
    }

    fn get_node_index(&self, node: &Node) -> usize {
        4 * self.get_pos(node.row, node.col) + node.dir as usize
    }
}

struct Edge {
    node: Node,
    cost: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    row: usize,
    col: usize,
    dir: Dir,
}

impl Node {
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

fn shortest_path(grid: &Grid, start: Node, goal: Node, ultra: bool) -> Option<usize> {
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

        if cost > dist[grid.get_node_index(&node)] {
            continue;
        }

        let edges = if ultra {
            grid.get_edges_ultra(node)
        } else {
            grid.get_edges(node)
        };

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

fn part1(input: &[u8]) -> usize {
    let grid = Grid::parse(input);

    let mut result = usize::MAX;

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
            if let Some(dist) = shortest_path(&grid, start, goal, false) {
                result = std::cmp::min(result, dist);
            }
        }
    }

    result
}

fn part2(input: &[u8]) -> usize {
    let grid = Grid::parse(input);

    let mut result = usize::MAX;

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
            if let Some(dist) = shortest_path(&grid, start, goal, true) {
                result = std::cmp::min(result, dist);
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
