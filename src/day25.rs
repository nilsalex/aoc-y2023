extern crate test;

use std::collections::HashMap;
use std::collections::VecDeque;

const INPUT: &[u8] = include_bytes!("../inputs/day25.txt");

struct IntSet {
    data: [u128; 12],
}

impl IntSet {
    fn new() -> Self {
        Self { data: [0; 12] }
    }

    fn insert(&mut self, value: u16) {
        let (index, bit) = (value / 128, value % 128);
        self.data[index as usize] |= 1 << bit;
    }

    fn contains(&self, value: u16) -> bool {
        let (index, bit) = (value / 128, value % 128);
        self.data[index as usize] & (1 << bit) != 0
    }

    fn len(&self) -> usize {
        self.data.iter().map(|&x| x.count_ones() as usize).sum()
    }
}

#[derive(Debug)]
struct Graph {
    edges: HashMap<u16, Vec<u16>>,
}

impl Graph {
    fn label_to_u16(label: &[u8]) -> u16 {
        (label[0] - b'a') as u16 * 26 * 26
            + (label[1] - b'a') as u16 * 26
            + (label[2] - b'a') as u16
    }

    fn parse(bytes: &[u8]) -> Self {
        let mut node_map = HashMap::new();
        let mut next_node_id = 0;
        let mut edges = HashMap::new();

        for line in bytes.split(|&b| b == b'\n') {
            let mut iter = line.split(|&b| b == b':' || b == b' ');
            let src_bytes = iter.next().unwrap();
            let src = Self::label_to_u16(src_bytes);
            let src_id = if let Some(&id) = node_map.get(&src) {
                id
            } else {
                node_map.insert(src, next_node_id);
                next_node_id += 1;
                next_node_id - 1
            };
            iter.next();
            for dst_bytes in iter {
                let dst = Self::label_to_u16(dst_bytes);
                let dst_id = if let Some(&id) = node_map.get(&dst) {
                    id
                } else {
                    node_map.insert(dst, next_node_id);
                    next_node_id += 1;
                    next_node_id - 1
                };
                edges.entry(src_id).or_insert_with(Vec::new).push(dst_id);
                edges.entry(dst_id).or_insert_with(Vec::new).push(src_id);
            }
        }

        Self { edges }
    }

    fn find_connected(&self, start: u16, ignored_edges: &[(u16, u16)]) -> IntSet {
        let mut queue = VecDeque::new();
        let mut visited = IntSet::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(node) = queue.pop_front() {
            for &neighbour in &self.edges[&node] {
                if ignored_edges
                    .iter()
                    .any(|&(a, b)| a == node && b == neighbour || a == neighbour && b == node)
                {
                    continue;
                }

                if visited.contains(neighbour) {
                    continue;
                }

                visited.insert(neighbour);
                queue.push_back(neighbour);
            }
        }

        visited
    }
}

fn part1(input: &[u8]) -> usize {
    let graph = Graph::parse(input);

    let mut nodes = graph.edges.keys().collect::<Vec<_>>();
    nodes.sort();
    nodes.dedup();

    let num_nodes = nodes.len();

    let mut unique_edges = Vec::new();
    for (src, dsts) in graph.edges.iter() {
        for dst in dsts {
            unique_edges.push((*std::cmp::min(src, dst), *std::cmp::max(src, dst)));
        }
    }
    unique_edges.sort();
    unique_edges.dedup();

    for i in 0..unique_edges.len() {
        println!("pass {} ...", i);
        for j in i + 1..unique_edges.len() {
            println!("    pass {} ...", j);
            for k in j + 1..unique_edges.len() {
                let e1 = unique_edges[i];
                let e2 = unique_edges[j];
                let e3 = unique_edges[k];
                let connected = Graph::find_connected(&graph, *nodes[0], &[e1, e2, e3]);
                let num_connected = connected.len();
                if num_connected < num_nodes {
                    return num_connected * (num_nodes - num_connected);
                }
            }
        }
    }

    0
}

pub fn main() {
    let input = INPUT.trim_ascii_end();

    println!("{}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day25.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 54);
    }

    #[test]
    fn test_intset() {
        let mut set = IntSet::new();

        assert!(set.len() == 0);
        assert!(!set.contains(0));
        assert!(!set.contains(1));
        assert!(!set.contains(1000));

        set.insert(20);
        assert!(set.len() == 1);
        assert!(!set.contains(0));
        assert!(!set.contains(1));
        assert!(!set.contains(1000));
        assert!(set.contains(20));

        set.insert(0);
        assert!(set.len() == 2);
        assert!(set.contains(0));
        assert!(!set.contains(1));
        assert!(!set.contains(1000));
        assert!(set.contains(20));

        set.insert(800);
        assert!(set.len() == 3);
        assert!(set.contains(0));
        assert!(!set.contains(1));
        assert!(!set.contains(1000));
        assert!(set.contains(20));
        assert!(set.contains(800));
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = INPUT.trim_ascii_end();
        b.iter(|| part1(input))
    }
}
