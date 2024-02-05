extern crate test;

use std::collections::HashMap;
use std::collections::VecDeque;

const INPUT: &[u8] = include_bytes!("../inputs/day20.txt");

struct Broadcast {
    id: usize,
    destinations: Vec<usize>,
}

impl Module for Broadcast {
    fn add_input(&mut self, _id: usize) {}

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_destinations(&self) -> &[usize] {
        &self.destinations
    }

    fn process(
        &mut self,
        _input_id: usize,
        signal: Signal,
    ) -> Either<Signal, (Signal, Vec<usize>)> {
        Either::Right((signal, self.destinations.clone()))
    }
}

enum FlipFlopState {
    On,
    Off,
}

struct FlipFlop {
    id: usize,
    destinations: Vec<usize>,
    state: FlipFlopState,
}

impl Module for FlipFlop {
    fn add_input(&mut self, _id: usize) {}

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_destinations(&self) -> &[usize] {
        &self.destinations
    }

    fn process(
        &mut self,
        _input_id: usize,
        signal: Signal,
    ) -> Either<Signal, (Signal, Vec<usize>)> {
        match signal {
            Signal::High => Either::Right((signal, Vec::new())),
            Signal::Low => match self.state {
                FlipFlopState::Off => {
                    self.state = FlipFlopState::On;
                    return Either::Right((Signal::High, self.destinations.clone()));
                }
                FlipFlopState::On => {
                    self.state = FlipFlopState::Off;
                    return Either::Right((Signal::Low, self.destinations.clone()));
                }
            },
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Signal {
    Low,
    High,
}

struct Conjunction {
    id: usize,
    destinations: Vec<usize>,
    inputs: Vec<(usize, Signal)>,
}

impl Module for Conjunction {
    fn add_input(&mut self, id: usize) {
        self.inputs.push((id, Signal::Low));
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_destinations(&self) -> &[usize] {
        &self.destinations
    }

    fn process(&mut self, input_id: usize, signal: Signal) -> Either<Signal, (Signal, Vec<usize>)> {
        let pos = self
            .inputs
            .iter()
            .position(|&(id, _)| id == input_id)
            .unwrap();
        self.inputs[pos].1 = signal;

        if self.inputs.iter().all(|(_, s)| matches!(s, Signal::High)) {
            Either::Right((Signal::High, self.destinations.clone()))
        } else {
            Either::Right((Signal::Low, self.destinations.clone()))
        }
    }
}

struct Output {
    id: usize,
}

impl Module for Output {
    fn add_input(&mut self, _id: usize) {}

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_destinations(&self) -> &[usize] {
        &[]
    }

    fn process(
        &mut self,
        _input_id: usize,
        signal: Signal,
    ) -> Either<Signal, (Signal, Vec<usize>)> {
        Either::Left(signal)
    }
}

enum Either<S, T> {
    Left(S),
    Right(T),
}

trait Module {
    fn add_input(&mut self, id: usize);
    fn get_id(&self) -> usize;
    fn get_destinations(&self) -> &[usize];
    fn process(
        &mut self,
        input_id: usize,
        input_value: Signal,
    ) -> Either<Signal, (Signal, Vec<usize>)>;
}

fn parse_module(bytes: &[u8], id_map: &IdMap) -> Box<dyn Module> {
    let id_start = bytes
        .iter()
        .position(|&b| matches!(b, b'a'..=b'z'))
        .unwrap();
    let id_end = bytes.iter().position(|&b| b == b' ').unwrap();

    let id = id_map.get_id(&bytes[id_start..id_end]);

    let arrow_pos = bytes.iter().position(|&b| b == b'>').unwrap();

    let destinations = bytes[arrow_pos + 1..]
        .split(|&b| b == b',')
        .map(|group| group.trim_ascii())
        .map(|group| id_map.get_id(group))
        .collect::<Vec<usize>>();

    match bytes[0] {
        b'%' => Box::new(FlipFlop {
            id,
            destinations,
            state: FlipFlopState::Off,
        }),
        b'&' => Box::new(Conjunction {
            id,
            destinations,
            inputs: Vec::new(),
        }),
        _ => Box::new(Broadcast { id, destinations }),
    }
}

#[derive(Debug)]
struct IdMap {
    id_to_bytes: Vec<Vec<u8>>,
    bytes_to_id: HashMap<Vec<u8>, usize>,
}

impl IdMap {
    fn parse(input: &[u8]) -> Self {
        let mut vec = Vec::new();
        let mut map = HashMap::new();

        let mut buf = Vec::new();

        for byte in input {
            if matches!(byte, b'a'..=b'z') {
                buf.push(*byte);
                continue;
            }

            if buf.is_empty() {
                continue;
            }

            if map.contains_key(&buf) {
                buf.clear();
                continue;
            }

            let position = vec.len();
            vec.push(Vec::new());
            map.insert(buf.clone(), position);

            std::mem::swap(&mut buf, &mut vec[position]);
        }

        if !buf.is_empty() && !map.contains_key(&buf) {
            let position = vec.len();
            vec.push(Vec::new());
            map.insert(buf.clone(), position);

            std::mem::swap(&mut buf, &mut vec[position]);
        }

        Self {
            id_to_bytes: vec,
            bytes_to_id: map,
        }
    }

    fn get_id(&self, bytes: &[u8]) -> usize {
        *self.bytes_to_id.get(bytes).unwrap()
    }
}

fn part1(input: &[u8]) -> usize {
    let id_map = IdMap::parse(input);

    let mut modules: Vec<_> = input
        .split(|&b| b == b'\n')
        .map(|line| parse_module(line, &id_map))
        .collect();

    for id in 0..id_map.id_to_bytes.len() {
        let position = modules.iter().position(|module| module.get_id() == id);
        if position.is_none() {
            modules.push(Box::new(Output { id }));
            break;
        }
    }

    modules.sort_by(|a, b| a.get_id().cmp(&b.get_id()));

    for module in modules.iter() {
        println!("{}", module.get_id());
    }

    let mut inputs = vec![Vec::new(); id_map.id_to_bytes.len()];

    for id in 0..modules.len() {
        for destination in modules[id].get_destinations() {
            inputs[*destination].push(id);
        }
    }

    for id in 0..modules.len() {
        for &input in inputs[id].iter() {
            modules[id].add_input(input);
        }
    }

    let broadcast_id = id_map.get_id(b"broadcaster");

    let mut low_count = 0;
    let mut high_count = 0;

    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back((broadcast_id, (broadcast_id, Signal::Low)));
        while let Some((module_id, (input_id, signal))) = queue.pop_front() {
            let module = &mut modules[module_id];
            match module.process(input_id, signal) {
                Either::Left(signal) => match signal {
                    Signal::Low => low_count += 1,
                    Signal::High => high_count += 1,
                },
                Either::Right((signal, destinations)) => {
                    for id in destinations {
                        queue.push_back((id, (module_id, signal)));
                    }
                }
            }
        }
    }

    low_count * high_count
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day20.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 11687500);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 0);
    }

    // #[bench]
    // fn bench_part1(b: &mut Bencher) {
    //     let input = INPUT.trim_ascii_end();
    //     b.iter(|| part1(input))
    // }
    //
    // #[bench]
    // fn bench_part2(b: &mut Bencher) {
    //     let input = INPUT.trim_ascii_end();
    //     b.iter(|| part2(input))
    // }
}
