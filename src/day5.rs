use std::{cmp::Ordering, collections::HashMap};

type Rules = HashMap<u8, Vec<u8>>;
type Update = Vec<u8>;

fn part1(updates: &[Update], rules: &Rules) {
    let result = updates.iter().fold(0, |acc, update| {
        let is_ordered = update.is_sorted_by(|a, b| rules.get(a).unwrap().contains(b));
        if is_ordered {
            let length = update.len();
            acc + update[(length - 1) / 2] as usize
        } else {
            acc
        }
    });

    println!("{result:?}");
}

fn part2(updates: &[Update], rules: &Rules) {
    let result = updates.iter().fold(0, |acc, update| {
        let ordered = update.is_sorted_by(|a, b| rules.get(a).unwrap().contains(b));
        let length = update.len();

        if !ordered {
            let mut new_update = update.clone();
            new_update.sort_by(|a, b| {
                if rules.get(a).unwrap().contains(b) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });
            acc + new_update[(length - 1) / 2] as usize
        } else {
            acc
        }
    });

    println!("{result:?}");
}

pub fn day5() {
    let string = include_str!("../inputs/day5");
    let (rules_string, updates_string) = string.split_once("\n\n").unwrap();

    let mut rules: Rules = HashMap::new();

    rules_string.lines().for_each(|l| {
        let (l, r) = l.split_once('|').unwrap();
        rules
            .entry(l.parse().unwrap())
            .or_default()
            .push(r.parse().unwrap());
    });

    let updates: Vec<Update> = updates_string
        .lines()
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    part1(&updates, &rules);
    part2(&updates, &rules);
}
