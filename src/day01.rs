const INPUT: &[u8] = include_bytes!("../input01.txt");

fn get_first_digit_in_line(line: &[u8]) -> (usize, u32) {
    for (i, c) in line.iter().enumerate() {
        match c {
            b'0' => return (i, 0),
            b'1' => return (i, 1),
            b'2' => return (i, 2),
            b'3' => return (i, 3),
            b'4' => return (i, 4),
            b'5' => return (i, 5),
            b'6' => return (i, 6),
            b'7' => return (i, 7),
            b'8' => return (i, 8),
            b'9' => return (i, 9),
            _ => (),
        }
    }
    panic!();
}

fn get_last_digit_in_line(line: &[u8]) -> (usize, u32) {
    for (i, c) in line.iter().rev().enumerate() {
        match c {
            b'0' => return (i, 0),
            b'1' => return (i, 1),
            b'2' => return (i, 2),
            b'3' => return (i, 3),
            b'4' => return (i, 4),
            b'5' => return (i, 5),
            b'6' => return (i, 6),
            b'7' => return (i, 7),
            b'8' => return (i, 8),
            b'9' => return (i, 9),
            _ => (),
        }
    }
    panic!();
}

const CANDIDATES: [(&[u8], u32); 9] = [
    (b"one", 1),
    (b"two", 2),
    (b"three", 3),
    (b"four", 4),
    (b"five", 5),
    (b"six", 6),
    (b"seven", 7),
    (b"eight", 8),
    (b"nine", 9),
];

const REV_CANDIDATES: [(&[u8], u32); 9] = [
    (b"eno", 1),
    (b"owt", 2),
    (b"eerht", 3),
    (b"ruof", 4),
    (b"evif", 5),
    (b"xis", 6),
    (b"neves", 7),
    (b"thgie", 8),
    (b"enin", 9),
];

fn get_first_string_digit_in_line(line: &[u8]) -> Option<(usize, u32)> {
    CANDIDATES
        .into_iter()
        .flat_map(|(candidate, value)| {
            let index = line
                .windows(candidate.len())
                .position(|window| window == candidate)?;
            Some((index, value))
        })
        .min_by_key(|(i, _)| *i)
}

fn get_last_string_digit_in_line(line: &[u8]) -> Option<(usize, u32)> {
    REV_CANDIDATES
        .into_iter()
        .flat_map(|(candidate, value)| {
            let mut rev_line = Vec::from(line);
            rev_line.reverse();
            let index = rev_line
                .windows(candidate.len())
                .position(|window| window == candidate)?;
            Some((index, value))
        })
        .min_by_key(|(i, _)| *i)
}

pub fn main() {
    let lines = INPUT.trim_ascii_end().split(|c| *c == b'\n');

    let result: u32 = lines
        .clone()
        .map(|line| {
            let (_, a) = get_first_digit_in_line(line);
            let (_, b) = get_last_digit_in_line(line);

            10 * a + b
        })
        .sum();

    println!("{}", result);

    let result: u32 = lines
        .clone()
        .map(|line| {
            let (i1, d1) = get_first_digit_in_line(line);
            let a = if let Some((i2, d2)) = get_first_string_digit_in_line(line) {
                if i1 < i2 {
                    d1
                } else {
                    d2
                }
            } else {
                d1
            };

            let (i3, d3) = get_last_digit_in_line(line);
            let b = if let Some((i4, d4)) = get_last_string_digit_in_line(line) {
                if i3 < i4 {
                    d3
                } else {
                    d4
                }
            } else {
                d3
            };

            10 * a + b
        })
        .sum();

    println!("{}", result);
}
