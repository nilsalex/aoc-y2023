#![feature(test)]
#![feature(byte_slice_trim_ascii)]

use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() {
    if let Some(day) = env::args().nth(1) {
        match day.as_str() {
            "01" => day01::main(),
            "02" => day02::main(),
            "03" => day03::main(),
            "04" => day04::main(),
            "05" => day05::main(),
            "06" => day06::main(),
            "07" => day07::main(),
            "08" => day08::main(),
            "09" => day09::main(),
            "10" => day10::main(),
            "11" => day11::main(),
            "12" => day12::main(),
            "13" => day13::main(),
            "14" => day14::main(),
            "15" => day15::main(),
            "16" => day16::main(),
            "17" => day17::main(),
            "18" => day18::main(),
            "19" => day19::main(),
            "20" => day20::main(),
            "21" => day21::main(),
            "22" => day22::main(),
            "23" => day23::main(),
            "24" => day24::main(),
            "25" => day25::main(),
            _ => {
                panic!("invalid argument for \"day\": {}", day)
            }
        }
    } else {
        day01::main();
        day02::main();
        day03::main();
        day04::main();
        day05::main();
        day06::main();
        day07::main();
        day08::main();
        day09::main();
        day10::main();
        day11::main();
        day12::main();
        day13::main();
        day14::main();
        day15::main();
        day16::main();
        day17::main();
        day18::main();
        day19::main();
        day20::main();
        day21::main();
        day22::main();
        day23::main();
        day24::main();
        day25::main();
    }
}
