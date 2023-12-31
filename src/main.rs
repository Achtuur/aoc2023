#![allow(unused_imports, non_snake_case)]

use std::time::Instant;

use day::Day;
use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;

use crate::{day5::Day5, day6::Day6, day7::Day7, day8::Day8, day9::Day9, day10::Day10, day11::Day11};

pub mod day;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod util;
pub mod day10;
pub mod day11;

fn main() {
    let mut day = Day11::default();
    day.read_input().unwrap();

    let timer = Instant::now();
    let res_a = day.A();
    println!("res_a: {0:?} ({1:?})", res_a, timer.elapsed());


    let timer = Instant::now();
    let res_b = day.B();    
    println!("res_b: {0:?} ({1:?})", res_b, timer.elapsed());
}
