use std::sync::LazyLock;

use regex::Regex;

static REGEX_PART_1: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());
static REGEX_PART_2: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(don't\(\)|do\(\)|mul\((\d+),(\d+)\))").unwrap());

fn part1(string: &str) {
    let result = REGEX_PART_1.captures_iter(string).fold(0, |acc, c| {
        let (_, [l, r]) = c.extract();
        let (l, r): (usize, usize) = (l.parse().unwrap(), r.parse().unwrap());
        acc + l * r
    });
    println!("{result}");
}

fn part2(string: &str) {
    let result = REGEX_PART_2
        .captures_iter(string)
        .fold((0, true), |(acc, flag), c| {
            if flag && c.get(0).unwrap().as_str().starts_with("mul") {
                let (l, r) = (c.get(2).unwrap().as_str(), c.get(3).unwrap().as_str());
                let (l, r): (usize, usize) = (l.parse().unwrap(), r.parse().unwrap());
                (acc + l * r, true)
            } else if c.get(0).unwrap().as_str().starts_with("do()") {
                (acc, true)
            } else {
                (acc, false)
            }
        });
    println!("{result:?}");
}

pub fn day3() {
    let string = include_str!("../inputs/day3");

    part1(string);
    part2(string);
}
