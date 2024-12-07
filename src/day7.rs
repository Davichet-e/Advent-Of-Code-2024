use itertools::{repeat_n, Itertools};

pub fn day7() {
    let string = include_str!("../inputs/day7");

    let result: usize = string
        .lines()
        .filter_map(|l| {
            let (test_value, numbers) = l.split_once(':').unwrap();
            let numbers: Vec<usize> = numbers
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            let test_value: usize = test_value.parse().unwrap();
            let operators = [
                '+', '*', '|', // toggle comment for part 1/2
            ];
            let yes = repeat_n(operators, numbers.len() - 1)
                .multi_cartesian_product()
                .any(|combination| {
                    let mut acc = numbers[0];
                    combination
                        .iter()
                        .zip(numbers.iter().skip(1))
                        .for_each(|(operator, &n)| match operator {
                            '+' => acc += n,
                            '*' => acc *= n,
                            '|' => {
                                acc = {
                                    let mut e = 1;
                                    // Concatenate 2 numbers without strings
                                    while n / e != 0 {
                                        e *= 10;
                                    }

                                    acc * e + n
                                }
                            }
                            _ => unreachable!(),
                        });

                    acc == test_value
                });
            if yes {
                Some(test_value)
            } else {
                None
            }
        })
        .sum();

    println!("{result:?}");
}
