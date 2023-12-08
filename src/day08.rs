extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day08.txt");

fn parse_instructions(line: &[u8]) -> Vec<u8> {
    line.iter()
        .map(|c| match c {
            b'L' => 0,
            b'R' => 1,
            _ => panic!(),
        })
        .collect()
}

fn get_node_number(node: &[u8]) -> usize {
    node.iter().fold(0, |acc, c| acc * 26 + (c - b'A') as usize)
}

fn is_start_node(node: usize) -> bool {
    (node % (26 * 26)) % 26 == 0
}

fn is_end_node(node: usize) -> bool {
    (node % (26 * 26)) % 26 == 25
}

fn part1(input: &[u8]) -> usize {
    let mut lines = input.split(|c| *c == b'\n');
    let instructions: Vec<u8> = parse_instructions(lines.next().unwrap());

    let mut map = [[0_usize; 2]; 26 * 26 * 26];

    lines.skip(1).for_each(|line| {
        let node = get_node_number(&line[0..=2]);
        let left = get_node_number(&line[7..=9]);
        let right = get_node_number(&line[12..=14]);

        map[node] = [left, right];
    });

    let mut node = 0;
    let mut count = 0;
    let target_node = get_node_number(b"ZZZ");

    for direction in instructions.iter().cycle() {
        count += 1;
        node = map[node][*direction as usize];
        if node == target_node {
            return count;
        }
    }

    panic!()
}

fn part2(input: &[u8]) -> usize {
    let mut lines = input.split(|c| *c == b'\n');
    let instructions: Vec<u8> = parse_instructions(lines.next().unwrap());

    let mut map = [[0_usize; 2]; 26 * 26 * 26];
    let mut nodes: Vec<usize> = vec![];

    lines.skip(1).for_each(|line| {
        let node = get_node_number(&line[0..=2]);
        let left = get_node_number(&line[7..=9]);
        let right = get_node_number(&line[12..=14]);

        map[node] = [left, right];

        if is_start_node(node) {
            nodes.push(node);
        }
    });

    nodes
        .iter()
        .map(|start_node| {
            let mut count = 0_usize;
            let mut node = *start_node;

            for direction in instructions.iter().cycle() {
                count += 1;
                node = map[node][*direction as usize];
                if is_end_node(node) {
                    break;
                }
            }

            count
        })
        .fold(1, num::integer::lcm)
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day08.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 6);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 6);
    }

    #[test]
    fn test_is_start_node() {
        assert_eq!(is_end_node(get_node_number(b"BBA")), false);
        assert_eq!(is_end_node(get_node_number(b"BBZ")), true);
    }

    #[test]
    fn test_is_end_node() {
        assert_eq!(is_end_node(get_node_number(b"BBA")), false);
        assert_eq!(is_end_node(get_node_number(b"BBZ")), true);
        assert_eq!(is_end_node(get_node_number(b"ZZZ")), true);
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
