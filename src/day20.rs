extern crate test;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;

const INPUT: &[u8] = include_bytes!("../inputs/day20.txt");

#[derive(Debug)]
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

#[derive(Debug)]
enum FlipFlopState {
    On,
    Off,
}

#[derive(Debug)]
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
                    Either::Right((Signal::High, self.destinations.clone()))
                }
                FlipFlopState::On => {
                    self.state = FlipFlopState::Off;
                    Either::Right((Signal::Low, self.destinations.clone()))
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

#[derive(Debug)]
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
            Either::Right((Signal::Low, self.destinations.clone()))
        } else {
            Either::Right((Signal::High, self.destinations.clone()))
        }
    }
}

#[derive(Debug)]
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

trait Module: std::fmt::Debug {
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
    let id_start = bytes.iter().position(|&b| b.is_ascii_lowercase()).unwrap();
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
            if byte.is_ascii_lowercase() {
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

    let mut modules: Vec<Box<dyn Module>> = input
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

    modules.sort_by_key(|a| a.get_id());

    let mut inputs = vec![Vec::new(); id_map.id_to_bytes.len()];

    for (id, module) in modules.iter().enumerate() {
        for destination in module.get_destinations() {
            inputs[*destination].push(id);
        }
    }

    for (id, module) in modules.iter_mut().enumerate() {
        for &input in inputs[id].iter() {
            module.add_input(input);
        }
    }

    let broadcast_id = id_map.get_id(b"broadcaster");

    let mut low_count = 0;
    let mut high_count = 0;

    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back((broadcast_id, (broadcast_id, Signal::Low)));
        while let Some((module_id, (input_id, signal))) = queue.pop_front() {
            match signal {
                Signal::Low => low_count += 1,
                Signal::High => high_count += 1,
            };
            let module = &mut modules[module_id];
            if let Either::Right((signal, destinations)) = module.process(input_id, signal) {
                for id in destinations {
                    queue.push_back((id, (module_id, signal)));
                }
            }
        }
    }

    low_count * high_count
}

fn part2(input: &[u8]) -> usize {
    let id_map = IdMap::parse(input);

    let mut modules: Vec<Box<dyn Module>> = input
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

    modules.sort_by_key(|a| a.get_id());

    let mut inputs = vec![Vec::new(); id_map.id_to_bytes.len()];

    for (id, module) in modules.iter().enumerate() {
        for destination in module.get_destinations() {
            inputs[*destination].push(id);
        }
    }

    for (id, module) in modules.iter_mut().enumerate() {
        for &input in inputs[id].iter() {
            module.add_input(input);
        }
    }

    let broadcast_id = id_map.get_id(b"broadcaster");
    let nd_id = id_map.get_id(b"nd");
    let pc_id = id_map.get_id(b"pc");
    let vd_id = id_map.get_id(b"vd");
    let tx_id = id_map.get_id(b"tx");

    let mut button_count = 0;

    let mut nd_count = None;
    let mut pc_count = None;
    let mut vd_count = None;
    let mut tx_count = None;

    loop {
        if nd_count.is_some() && pc_count.is_some() && vd_count.is_some() && tx_count.is_some() {
            break;
        }
        button_count += 1;
        let mut queue = VecDeque::new();
        queue.push_back((broadcast_id, (broadcast_id, Signal::Low)));
        while let Some((module_id, (input_id, signal))) = queue.pop_front() {
            let module = &mut modules[module_id];
            if module.get_id() == nd_id && matches!(signal, Signal::Low) && nd_count.is_none() {
                nd_count = Some(button_count);
            }
            if module.get_id() == pc_id && matches!(signal, Signal::Low) && pc_count.is_none() {
                pc_count = Some(button_count);
            }
            if module.get_id() == vd_id && matches!(signal, Signal::Low) && vd_count.is_none() {
                vd_count = Some(button_count);
            }
            if module.get_id() == tx_id && matches!(signal, Signal::Low) && tx_count.is_none() {
                tx_count = Some(button_count);
            }
            if let Either::Right((signal, destinations)) = module.process(input_id, signal) {
                for id in destinations {
                    queue.push_back((id, (module_id, signal)));
                }
            }
        }
    }

    nd_count.unwrap() * pc_count.unwrap() * vd_count.unwrap() * tx_count.unwrap()
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

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = INPUT.trim_ascii_end();
        b.iter(|| part1(input))
    }
}
