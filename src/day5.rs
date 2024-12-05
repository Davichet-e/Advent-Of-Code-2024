use std::collections::HashMap;

type Rules = HashMap<u8, Vec<u8>>;
type Update = Vec<u8>;

fn is_ordered(update: &Update, rules: &Rules) -> bool {
    update.iter().enumerate().all(|(i, n)| {
        if let Some(numbers_to_check) = rules.get(n) {
            numbers_to_check.iter().all(|number| {
                update
                    .iter()
                    .position(|past_number| past_number == number)
                    .map_or(true, |j| j < i)
            })
        } else {
            true
        }
    })
}

fn part1(updates: &[Update], rules: &Rules) {
    let result = updates.iter().fold(0, |acc, update| {
        let is_ordered = is_ordered(update, rules);
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
        let is_ordered_ = is_ordered(update, rules);
        let length = update.len();

        if !is_ordered_ {
            let mut new_update = update.clone();
            let mut curr_index = 0;
            while curr_index < update.len() {
                if let Some(numbers_to_check) = rules.get(&new_update[curr_index]) {
                    let furthest_index = numbers_to_check
                        .iter()
                        .filter_map(|number| {
                            new_update
                                .iter()
                                .position(|past_number| past_number == number)
                        })
                        .max();

                    if let Some(position) = furthest_index {
                        if curr_index < position {
                            // https://stackoverflow.com/a/62152815
                            new_update[curr_index..=position].rotate_left(1);
                            continue;
                        }
                    }
                }
                curr_index += 1;
            }
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
            .entry(r.parse().unwrap())
            .or_default()
            .push(l.parse().unwrap());
    });

    let updates: Vec<Update> = updates_string
        .lines()
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    part1(&updates, &rules);
    part2(&updates, &rules);
}
