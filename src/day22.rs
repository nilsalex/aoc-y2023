extern crate test;

use std::collections::HashSet;

const INPUT: &[u8] = include_bytes!("../inputs/day22.txt");

const POWERS_OF_TEN: [isize; 4] = [1, 10, 100, 1000];

fn isize_from_bytes(bytes: &[u8]) -> isize {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, x)| {
        acc + (x & 0x0f) as isize * POWERS_OF_TEN[ix]
    })
}

#[derive(Debug, Clone)]
struct Block {
    x_range: (isize, isize),
    y_range: (isize, isize),
    z_range: (isize, isize),
}

impl Block {
    fn parse(bytes: &[u8]) -> Self {
        let mut numbers = bytes.split(|&b| b == b'~' || b == b',');

        let start = (
            isize_from_bytes(numbers.next().unwrap()),
            isize_from_bytes(numbers.next().unwrap()),
            isize_from_bytes(numbers.next().unwrap()),
        );
        let end = (
            isize_from_bytes(numbers.next().unwrap()),
            isize_from_bytes(numbers.next().unwrap()),
            isize_from_bytes(numbers.next().unwrap()),
        );

        if start.0 > end.0 || start.1 > end.1 || start.2 > end.2 {
            Self {
                x_range: (end.0, start.0),
                y_range: (end.1, start.1),
                z_range: (end.2, start.2),
            }
        } else {
            Self {
                x_range: (start.0, end.0),
                y_range: (start.1, end.1),
                z_range: (start.2, end.2),
            }
        }
    }

    fn can_fall(&self, covered: &HashSet<(isize, isize, isize)>) -> bool {
        if self.z_range.0 <= 1 {
            return false;
        }

        for x in self.x_range.0..=self.x_range.1 {
            for y in self.y_range.0..=self.y_range.1 {
                if covered.contains(&(x, y, self.z_range.0 - 1)) {
                    return false;
                }
            }
        }
        true
    }

    fn move_down(&mut self, covered: &mut HashSet<(isize, isize, isize)>) {
        uncover(self, covered);
        self.z_range.0 -= 1;
        self.z_range.1 -= 1;
        cover(self, covered);
    }
}

fn get_covering_set(blocks: &[Block]) -> HashSet<(isize, isize, isize)> {
    let mut covering_set = HashSet::new();

    for block in blocks {
        for x in block.x_range.0..=block.x_range.1 {
            for y in block.y_range.0..=block.y_range.1 {
                for z in block.z_range.0..=block.z_range.1 {
                    covering_set.insert((x, y, z));
                }
            }
        }
    }

    covering_set
}

fn get_first_falling_block(
    blocks: &[Block],
    covered: &HashSet<(isize, isize, isize)>,
) -> Option<usize> {
    for (ix, block) in blocks.iter().enumerate() {
        if block.can_fall(covered) {
            return Some(ix);
        }
    }

    None
}

fn uncover(block: &Block, covered: &mut HashSet<(isize, isize, isize)>) {
    for x in block.x_range.0..=block.x_range.1 {
        for y in block.y_range.0..=block.y_range.1 {
            for z in block.z_range.0..=block.z_range.1 {
                covered.remove(&(x, y, z));
            }
        }
    }
}

fn cover(block: &Block, covered: &mut HashSet<(isize, isize, isize)>) {
    for x in block.x_range.0..=block.x_range.1 {
        for y in block.y_range.0..=block.y_range.1 {
            for z in block.z_range.0..=block.z_range.1 {
                covered.insert((x, y, z));
            }
        }
    }
}

fn part1(input: &[u8]) -> usize {
    let mut blocks = input
        .split(|&b| b == b'\n')
        .map(Block::parse)
        .collect::<Vec<_>>();

    let mut covered = get_covering_set(&blocks);

    while let Some(ix) = get_first_falling_block(&blocks, &covered) {
        blocks[ix].move_down(&mut covered);
    }

    let mut removable = 0;

    for (ix, block) in blocks.iter().enumerate() {
        uncover(block, &mut covered);
        if blocks
            .iter()
            .enumerate()
            .filter(|&(jx, _)| jx != ix)
            .all(|(_, other)| !other.can_fall(&covered))
        {
            removable += 1;
        }
        cover(block, &mut covered);
    }

    removable
}

fn part2(input: &[u8]) -> usize {
    let mut blocks = input
        .split(|&b| b == b'\n')
        .map(Block::parse)
        .collect::<Vec<_>>();

    let mut covered = get_covering_set(&blocks);

    while let Some(ix) = get_first_falling_block(&blocks, &covered) {
        blocks[ix].move_down(&mut covered);
    }

    let mut result = 0;

    for (ix, block) in blocks.iter().enumerate() {
        let mut blocks = blocks
            .iter()
            .enumerate()
            .filter(|&(jx, _)| ix != jx)
            .map(|(_, b)| b)
            .cloned()
            .collect::<Vec<_>>();
        let mut covered = covered.clone();

        let mut moved = HashSet::new();

        uncover(block, &mut covered);
        while let Some(ix) = get_first_falling_block(&blocks, &covered) {
            blocks[ix].move_down(&mut covered);
            moved.insert(ix);
        }

        result += moved.len();
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day22.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 5);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 7);
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
