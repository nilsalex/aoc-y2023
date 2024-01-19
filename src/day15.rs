extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day15.txt");

fn get_hash(instruction: &[u8]) -> usize {
    instruction
        .iter()
        .fold(0, |acc, &b| ((acc + b as usize) * 17) % 256)
}

fn part1(input: &[u8]) -> usize {
    input.split(|&b| b == b',').map(get_hash).sum()
}

type Bucket = Vec<(Vec<u8>, u8)>;

fn part2(input: &[u8]) -> usize {
    let mut hash_map: Vec<Box<Bucket>> = vec![Box::default(); 256];

    for instruction in input.split(|&b| b == b',') {
        let mut groups = instruction.split(|&b| b == b'-' || b == b'=');
        let label = groups.next().unwrap();
        let hash = get_hash(label);
        let digit = groups.next().unwrap();

        if !digit.is_empty() {
            let value = digit[0] - 48;
            let bucket = &mut hash_map[hash];
            if let Some(index) = bucket.iter().position(|element| element.0 == label) {
                bucket[index] = (Vec::from(label), value);
            } else {
                bucket.push((Vec::from(label), value));
            }
        } else {
            let bucket = &mut hash_map[hash];
            if let Some(index) = bucket.iter().position(|element| element.0 == label) {
                bucket.remove(index);
            }
        }
    }

    hash_map
        .iter()
        .enumerate()
        .map(|(index, bucket)| {
            bucket
                .iter()
                .enumerate()
                .map(|(slot, (_, f))| (index + 1) * (slot + 1) * (*f as usize))
                .sum::<usize>()
        })
        .sum()
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day15.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 1320);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 145);
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
