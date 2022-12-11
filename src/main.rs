#![allow(dead_code)]

use std::io;

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


fn main() {
    let stdin = io::stdin();
    // day11::part1(stdin);
    day11::part2(stdin);
}
