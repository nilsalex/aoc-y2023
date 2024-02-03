extern crate test;

use std::collections::HashMap;

const INPUT: &[u8] = include_bytes!("../inputs/day19.txt");

const POWERS_OF_TEN: [u32; 6] = [1, 10, 100, 1000, 10000, 100000];

fn u32_from_bytes(bytes: &[u8]) -> u32 {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, x)| {
        acc + (x & 0x0f) as u32 * POWERS_OF_TEN[ix]
    })
}
#[derive(Debug, Clone, Copy)]
enum Property {
    X,
    M,
    A,
    S,
}

impl Property {
    fn parse(byte: u8) -> Self {
        match byte {
            b'x' => Self::X,
            b'm' => Self::M,
            b'a' => Self::A,
            b's' => Self::S,
            _ => panic!(),
        }
    }
}

#[derive(Copy, Clone)]
enum Result {
    Accept,
    Reject,
    Goto(u32),
}

impl Result {
    fn parse(bytes: &[u8]) -> Self {
        match bytes[0] {
            b'A' => Self::Accept,
            b'R' => Self::Reject,
            _ => Self::Goto(Workflow::parse_id(bytes)),
        }
    }
}

enum Instruction {
    Less(Property, u32, Result),
    Greater(Property, u32, Result),
    Result(Result),
}

impl Instruction {
    fn parse(bytes: &[u8]) -> Self {
        if let Some(colon_pos) = bytes.iter().position(|&b| b == b':') {
            let comp_bytes = &bytes[0..colon_pos];
            let property = Property::parse(comp_bytes[0]);
            let comp_value = u32_from_bytes(&comp_bytes[2..]);
            let result = Result::parse(&bytes[colon_pos + 1..]);
            match comp_bytes[1] {
                b'<' => Self::Less(property, comp_value, result),
                b'>' => Self::Greater(property, comp_value, result),
                _ => panic!(),
            }
        } else {
            Self::Result(Result::parse(bytes))
        }
    }

    fn eval(&self, part: &Part) -> Option<Result> {
        match self {
            Self::Result(result) => Some(*result),
            Self::Less(property, value, result) => {
                if part.get(property) < *value {
                    Some(*result)
                } else {
                    None
                }
            }
            Self::Greater(property, value, result) => {
                if part.get(property) > *value {
                    Some(*result)
                } else {
                    None
                }
            }
        }
    }
}

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn parse(bytes: &[u8]) -> Self {
        let trimmed = &bytes[1..bytes.len() - 1];
        let mut groups = trimmed.split(|&b| b == b',');
        let x = u32_from_bytes(&groups.next().unwrap()[2..]);
        let m = u32_from_bytes(&groups.next().unwrap()[2..]);
        let a = u32_from_bytes(&groups.next().unwrap()[2..]);
        let s = u32_from_bytes(&groups.next().unwrap()[2..]);
        Part { x, m, a, s }
    }

    fn get(&self, property: &Property) -> u32 {
        match property {
            Property::X => self.x,
            Property::M => self.m,
            Property::A => self.a,
            Property::S => self.s,
        }
    }

    fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

struct Workflow {
    id: u32,
    instructions: Vec<Instruction>,
}

impl Workflow {
    pub fn parse_id(bytes: &[u8]) -> u32 {
        let mut offset = 0;
        let mut id = 0;
        for (index, byte) in bytes.iter().enumerate() {
            id *= 26;
            id += (byte - b'a') as u32;
            offset += 26_u32.pow(index as u32);
        }
        offset + id
    }

    fn parse(bytes: &[u8]) -> Self {
        let mut groups = bytes.split(|&b| b == b'{' || b == b'}');
        let id = Workflow::parse_id(groups.next().unwrap());
        let instructions_bytes = groups.next().unwrap();
        let instructions = instructions_bytes
            .split(|&b| b == b',')
            .map(Instruction::parse)
            .collect();

        Workflow { id, instructions }
    }

    fn eval(&self, part: &Part) -> Result {
        for instruction in self.instructions.iter() {
            if let Some(result) = instruction.eval(part) {
                return result;
            }
        }

        panic!()
    }
}

fn part1(input: &[u8]) -> usize {
    let mut lines = input.split(|&b| b == b'\n');

    let mut workflows = HashMap::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let workflow = Workflow::parse(line);
        workflows.insert(workflow.id, workflow);
    }

    let mut parts = Vec::new();
    for line in lines {
        parts.push(Part::parse(line));
    }

    let start_id = Workflow::parse_id(b"in");

    parts
        .iter()
        .filter(|part| {
            let mut workflow_id = start_id;

            loop {
                let workflow = workflows.get(&workflow_id).unwrap();
                match workflow.eval(part) {
                    Result::Reject => return false,
                    Result::Accept => return true,
                    Result::Goto(next_id) => workflow_id = next_id,
                }
            }
        })
        .map(|part| part.sum() as usize)
        .sum()
}

#[derive(Debug, Clone, Copy)]
enum Comparison {
    Less,
    Greater,
}

#[derive(Debug, Clone)]
struct Cube {
    x_min: u32,
    x_max: u32,
    m_min: u32,
    m_max: u32,
    a_min: u32,
    a_max: u32,
    s_min: u32,
    s_max: u32,
}

impl Cube {
    fn new() -> Self {
        Self {
            x_min: 1,
            x_max: 4000,
            m_min: 1,
            m_max: 4000,
            a_min: 1,
            a_max: 4000,
            s_min: 1,
            s_max: 4000,
        }
    }

    fn from_condition(comparison: Comparison, property: Property, value: u32) -> Self {
        let mut cube = Self::new();

        match (comparison, property) {
            (Comparison::Less, Property::X) => cube.x_max = value - 1,
            (Comparison::Less, Property::M) => cube.m_max = value - 1,
            (Comparison::Less, Property::A) => cube.a_max = value - 1,
            (Comparison::Less, Property::S) => cube.s_max = value - 1,
            (Comparison::Greater, Property::X) => cube.x_min = value + 1,
            (Comparison::Greater, Property::M) => cube.m_min = value + 1,
            (Comparison::Greater, Property::A) => cube.a_min = value + 1,
            (Comparison::Greater, Property::S) => cube.s_min = value + 1,
        }

        cube
    }

    fn volume(&self) -> usize {
        ((self.x_max + 1) - self.x_min) as usize
            * ((self.m_max + 1) - self.m_min) as usize
            * ((self.a_max + 1) - self.a_min) as usize
            * ((self.s_max + 1) - self.s_min) as usize
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        let result = Self {
            x_min: std::cmp::max(self.x_min, other.x_min),
            x_max: std::cmp::min(self.x_max, other.x_max),
            m_min: std::cmp::max(self.m_min, other.m_min),
            m_max: std::cmp::min(self.m_max, other.m_max),
            a_min: std::cmp::max(self.a_min, other.a_min),
            a_max: std::cmp::min(self.a_max, other.a_max),
            s_min: std::cmp::max(self.s_min, other.s_min),
            s_max: std::cmp::min(self.s_max, other.s_max),
        };

        if result.x_min > result.x_max {
            return None;
        }

        if result.m_min > result.m_max {
            return None;
        }

        if result.a_min > result.a_max {
            return None;
        }

        if result.s_min > result.s_max {
            return None;
        }

        Some(result)
    }
}

fn part2(input: &[u8]) -> usize {
    let mut lines = input.split(|&b| b == b'\n');

    let mut workflows = HashMap::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let workflow = Workflow::parse(line);
        workflows.insert(workflow.id, workflow);
    }

    let mut parts = Vec::new();
    for line in lines {
        parts.push(Part::parse(line));
    }

    let start_id = Workflow::parse_id(b"in");

    let mut stack = vec![(start_id, Cube::new())];
    let mut volume = 0;

    while let Some((node, mut cube)) = stack.pop() {
        let workflow = workflows.get(&node).unwrap();
        for instruction in workflow.instructions.iter() {
            match instruction {
                Instruction::Result(Result::Reject) => {
                    break;
                }
                Instruction::Result(Result::Accept) => {
                    volume += cube.volume();
                    break;
                }
                Instruction::Result(Result::Goto(next)) => {
                    stack.push((*next, cube.clone()));
                    break;
                }
                Instruction::Less(prop, val, Result::Accept) => {
                    if let Some(next_cube) =
                        cube.intersection(&Cube::from_condition(Comparison::Less, *prop, *val))
                    {
                        volume += next_cube.volume();
                    }

                    if let Some(next_cube) = cube.intersection(&Cube::from_condition(
                        Comparison::Greater,
                        *prop,
                        val - 1,
                    )) {
                        cube = next_cube;
                    } else {
                        break;
                    }
                }
                Instruction::Less(prop, val, Result::Goto(next)) => {
                    if let Some(next_cube) =
                        cube.intersection(&Cube::from_condition(Comparison::Less, *prop, *val))
                    {
                        stack.push((*next, next_cube));
                    }

                    if let Some(next_cube) = cube.intersection(&Cube::from_condition(
                        Comparison::Greater,
                        *prop,
                        val - 1,
                    )) {
                        cube = next_cube;
                    } else {
                        break;
                    }
                }
                Instruction::Less(prop, val, Result::Reject) => {
                    if let Some(next_cube) = cube.intersection(&Cube::from_condition(
                        Comparison::Greater,
                        *prop,
                        val - 1,
                    )) {
                        cube = next_cube;
                    } else {
                        break;
                    }
                }
                Instruction::Greater(prop, val, Result::Accept) => {
                    if let Some(next_cube) =
                        cube.intersection(&Cube::from_condition(Comparison::Greater, *prop, *val))
                    {
                        volume += next_cube.volume();
                    }

                    if let Some(next_cube) =
                        cube.intersection(&Cube::from_condition(Comparison::Less, *prop, val + 1))
                    {
                        cube = next_cube;
                    } else {
                        break;
                    }
                }
                Instruction::Greater(prop, val, Result::Goto(next)) => {
                    if let Some(next_cube) =
                        cube.intersection(&Cube::from_condition(Comparison::Greater, *prop, *val))
                    {
                        stack.push((*next, next_cube));
                    }

                    if let Some(next_cube) =
                        cube.intersection(&Cube::from_condition(Comparison::Less, *prop, val + 1))
                    {
                        cube = next_cube;
                    } else {
                        break;
                    }
                }
                Instruction::Greater(prop, val, Result::Reject) => {
                    if let Some(next_cube) =
                        cube.intersection(&Cube::from_condition(Comparison::Less, *prop, val + 1))
                    {
                        cube = next_cube;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    volume
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day19.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 19114);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 167409079868000);
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
