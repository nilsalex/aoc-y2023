extern crate test;

const INPUT: &str = include_str!("../inputs/day05.txt");

fn part1(input: &str) -> usize {
    let mut groups = input.split("\n\n");
    let seeds: Vec<usize> = groups
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    let maps: Vec<Vec<(usize, usize, usize)>> = groups
        .map(|group| {
            let mut map: Vec<(usize, usize, usize)> = group
                .lines()
                .skip(1)
                .map(|line| {
                    let mut nums = line.split_whitespace().map(|num| num.parse().unwrap());
                    let to = nums.next().unwrap();
                    let from = nums.next().unwrap();
                    let count = nums.next().unwrap();
                    (from, to, count)
                })
                .collect();
            map.sort_by_key(|(a, _, _)| *a);
            if map[0].0 != 0 {
                map.push((0, 0, map[0].0));
                map.sort_by_key(|(a, _, _)| *a);
            }
            map
        })
        .collect();

    seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed, |acc, map| {
                let entry = match map.binary_search_by(|(a, _, _)| a.cmp(&acc)) {
                    Ok(index) => index,
                    Err(index) => {
                        if index == 0 {
                            0
                        } else {
                            index - 1
                        }
                    }
                };
                let (from, to, count) = map[entry];
                let diff = acc - from;
                if diff < count {
                    to + diff
                } else {
                    acc
                }
            })
        })
        .min()
        .unwrap()
}

fn process_seed_range(
    from: usize,
    to: usize,
    map: &[(usize, isize, usize)],
) -> Vec<(usize, usize)> {
    let from_index = match map.binary_search_by(|(a, _, _)| a.cmp(&from)) {
        Ok(index) => index,
        Err(index) => {
            if index == 0 {
                0
            } else {
                index - 1
            }
        }
    };

    let to_index = match map.binary_search_by(|(a, _, _)| a.cmp(&to)) {
        Ok(index) => index,
        Err(index) => {
            if index == 0 {
                0
            } else {
                index - 1
            }
        }
    };

    let mut result: Vec<(usize, usize)> = vec![];

    for index in from_index..=to_index {
        let map_entry = map[index];
        let map_range_start = map_entry.0;
        let diff = map_entry.1;
        let map_range_count = map_entry.2;
        let map_range_end = map_range_start + map_range_count - 1;

        if from <= map_range_end {
            let this_range_start = std::cmp::max(from, map_range_start);
            let this_range_end = std::cmp::min(map_range_end, to);
            result.push((
                (this_range_start as isize + diff) as usize,
                (this_range_end as isize + diff) as usize,
            ));
        }

        if to > map_range_end {
            let this_range_start = std::cmp::max(map_range_end + 1, from);
            let this_range_end = if index + 1 < map.len() {
                std::cmp::min(to, map[index + 1].0 - 1)
            } else {
                to
            };
            if this_range_end >= this_range_start {
                result.push((this_range_start, this_range_end));
            }
        }
    }

    result.sort_by_key(|(a, _)| *a);

    result
}

fn part2(input: &str) -> usize {
    let mut groups = input.split("\n\n");
    let seeds_input: Vec<usize> = groups
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    let mut seed_ranges: Vec<(usize, usize)> = seeds_input
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1] - 1))
        .collect();
    seed_ranges.sort_by_key(|(start, _)| *start);
    let maps: Vec<Vec<(usize, isize, usize)>> = groups
        .map(|group| {
            let mut map: Vec<(usize, isize, usize)> = group
                .lines()
                .skip(1)
                .map(|line| {
                    let mut nums = line.split_whitespace().map(|num| num.parse().unwrap());
                    let to = nums.next().unwrap();
                    let from = nums.next().unwrap();
                    let count = nums.next().unwrap();
                    (from, (to as isize - from as isize), count)
                })
                .collect();
            map.sort_by_key(|(a, _, _)| *a);
            if map[0].0 != 0 {
                map.push((0, 0, map[0].0));
                map.sort_by_key(|(a, _, _)| *a);
            }
            map
        })
        .collect();

    for map in maps {
        let mut new_seed_ranges: Vec<(usize, usize)> = seed_ranges
            .iter()
            .flat_map(|range| process_seed_range(range.0, range.1, &map))
            .collect();
        std::mem::swap(&mut seed_ranges, &mut new_seed_ranges);
    }

    seed_ranges.iter().map(|range| range.0).min().unwrap()
}

pub fn main() {
    let input = INPUT.trim_end();

    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &str = include_str!("../test_inputs/day05.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_end();
        assert_eq!(part1(input), 35);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_end();
        assert_eq!(part2(input), 46);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = INPUT.trim_end();
        b.iter(|| part1(input))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = INPUT.trim_end();
        b.iter(|| part2(input))
    }
}
