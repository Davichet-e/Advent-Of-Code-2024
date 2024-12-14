use std::collections::HashMap;

pub fn day11() {
    let string = include_str!("../inputs/day11");

    let mut stone_counter: HashMap<usize, usize> = string
        .split_ascii_whitespace()
        .map(|n| (n.parse().unwrap(), 1))
        .collect();

    // Change 75 by 25 for part 1
    for _ in 0..75 {
        let mut new_stone_counter = HashMap::new();
        for (&value, &number) in &stone_counter {
            if value == 0 {
                *new_stone_counter.entry(1).or_insert(0) += number;
            } else {
                let n = number_of_digits(value);
                if n % 2 == 0 {
                    let left_digits = value / 10usize.pow(n / 2);
                    let right_digits = value % 10usize.pow(n / 2);

                    *new_stone_counter.entry(left_digits).or_insert(0) += number;
                    *new_stone_counter.entry(right_digits).or_insert(0) += number;
                } else {
                    *new_stone_counter.entry(value * 2024).or_insert(0) += number;
                }
            }
        }
        // println!("{new_list:?}");
        stone_counter.clone_from(&new_stone_counter);
    }

    println!("{}", stone_counter.values().sum::<usize>());
}

fn number_of_digits(n: usize) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}
