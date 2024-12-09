use std::{collections::VecDeque, ops::ControlFlow};

use itertools::Itertools;

fn part1(vd: &[i64]) {
    let mut vd_clone: VecDeque<&i64> = vd.iter().filter(|&&n| n >= 0).collect();
    let initial_clone_length = vd_clone.len();
    let result = vd.iter().enumerate().try_fold(0, |acc, (i, &id)| {
        if i >= initial_clone_length {
            ControlFlow::Break(acc)
        } else if id == -1 {
            ControlFlow::Continue(acc + (i as i64 * vd_clone.pop_back().unwrap()))
        } else {
            ControlFlow::Continue(acc + (i as i64 * id))
        }
    });
    let ControlFlow::Break(value) = result else {
        unreachable!()
    };
    println!("{value}");
}

fn part2(blocks: &[i64]) {
    let mut blocks_parsed = VecDeque::new();
    blocks.iter().for_each(|&id| {
        let (last_id, acc) = blocks_parsed.pop_back().unwrap_or((-2, 0));
        if last_id == id {
            blocks_parsed.push_back((id, acc + 1));
        } else {
            if last_id != -2 {
                blocks_parsed.push_back((last_id, acc));
            }
            blocks_parsed.push_back((id, 1));
        }
    });
    let blocks_reversed: VecDeque<(i64, i64)> = blocks_parsed
        .clone()
        .into_iter()
        .rev()
        .filter(|(id, _)| *id != -1)
        .collect();

    blocks_reversed.iter().for_each(|(new_id, number)| {
        let Some(position) = blocks_parsed.iter().position(|(id, _)| id == new_id) else {
            return;
        };
        let Some((_, (pos, item_to_edit))) = blocks_parsed
            .iter()
            .enumerate()
            .find_position(|(i, (id, n_times))| *id == -1 && n_times >= number && *i < position)
        else {
            return;
        };
        if item_to_edit.1 > *number {
            blocks_parsed.insert(pos + 1, (-1, item_to_edit.1 - *number));
            blocks_parsed[pos] = (*new_id, *number);
            blocks_parsed[position + 1] = (-1, *number);
        } else {
            blocks_parsed[pos] = (*new_id, *number);
            blocks_parsed[position] = (-1, *number);
        }
    });

    let (result, _) = blocks_parsed
        .iter()
        .fold((0, 0), |(acc, curr_index), (id, times)| {
            (
                if *id != -1 {
                    acc + (0i64..*times).fold(0i64, |acc, j| acc + id * (curr_index + j))
                } else {
                    acc
                },
                curr_index + times,
            )
        });

    println!("{result}");
}

pub fn day9() {
    let string = include_str!("../inputs/day9").trim_end();

    let mut vd = Vec::new();
    string
        .chars()
        .enumerate()
        .fold(0, |curr_id, (i, n_blocks)| {
            (0..n_blocks.to_digit(10).unwrap()).for_each(|_| {
                if i % 2 == 0 {
                    vd.push(curr_id);
                } else {
                    vd.push(-1);
                }
            });
            if i % 2 == 0 {
                curr_id + 1
            } else {
                curr_id
            }
        });
    part1(&vd);
    part2(&vd);
}
