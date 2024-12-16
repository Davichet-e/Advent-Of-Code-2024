use std::collections::HashSet;

use itertools::Itertools;

pub fn day15() {
    let string = include_str!("../inputs/day15");

    let (map_str, moves_str) = string.split_once("\n\n").unwrap();

    let map: Vec<Vec<char>> = map_str
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let moves: Vec<[i32; 2]> = moves_str
        .split_ascii_whitespace()
        .join("")
        .chars()
        .map(|c| match c {
            '>' => [1, 0],
            '<' => [-1, 0],
            '^' => [0, -1],
            'v' => [0, 1],
            _ => unreachable!(),
        })
        .collect();

    part1(&moves, &map);
    part2(&moves, &map);
}

fn part1(moves: &[[i32; 2]], map: &[Vec<char>]) {
    let initial_position = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            if let Some(x) = l.iter().position(|&c| c == '@') {
                Some((x, y))
            } else {
                None
            }
        })
        .unwrap();

    let mut map = map.to_vec();
    let mut current_position = initial_position;
    for [dx, dy] in moves {
        let new_x = (current_position.0 as i32 + dx) as usize;
        let new_y = (current_position.1 as i32 + dy) as usize;

        let object_at_new_pos = map[new_y][new_x];
        match object_at_new_pos {
            '.' => {
                map[current_position.1][current_position.0] = '.';
                current_position = (new_x, new_y);
                map[new_y][new_x] = '@';
            }
            '#' => {}
            'O' => {
                let mut i = 2;
                loop {
                    let new_obj_x = (current_position.0 as i32 + i * dx) as usize;
                    let new_obj_y = (current_position.1 as i32 + i * dy) as usize;

                    let object_at_new_pos = map[new_obj_y][new_obj_x];
                    match object_at_new_pos {
                        '.' => {
                            map[current_position.1][current_position.0] = '.';
                            current_position = (new_x, new_y);
                            map[new_obj_y][new_obj_x] = 'O';
                            map[new_y][new_x] = '@';
                            break;
                        }
                        'O' => {
                            i += 1;
                        }
                        '#' => break,
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    let mut result = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, &char) in line.iter().enumerate() {
            if char == 'O' {
                result += 100 * y + x;
            }
        }
    }
    println!("{result:?}");
}

fn part2(moves: &[[i32; 2]], map: &[Vec<char>]) {
    let mut map: Vec<Vec<char>> = map
        .iter()
        .map(|l| {
            l.iter()
                .flat_map(|c| match c {
                    '#' => ['#'; 2],
                    'O' => ['[', ']'],
                    '.' => ['.'; 2],
                    '@' => ['@', '.'],
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let initial_position = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            if let Some(x) = l.iter().position(|&c| c == '@') {
                Some((x, y))
            } else {
                None
            }
        })
        .unwrap();

    let mut current_position = initial_position;
    for [dx, dy] in moves {
        let new_x = (current_position.0 as i32 + dx) as usize;
        let new_y = (current_position.1 as i32 + dy) as usize;

        let object_at_new_pos = map[new_y][new_x];
        match object_at_new_pos {
            '.' => {
                map[current_position.1][current_position.0] = '.';
                current_position = (new_x, new_y);
                map[new_y][new_x] = '@';
            }
            '#' => {}
            '[' | ']' => {
                if *dy == 0 {
                    let mut i = 2;
                    loop {
                        let new_obj_x = (current_position.0 as i32 + i * dx) as usize;
                        let new_obj_y = (current_position.1 as i32 + i * dy) as usize;

                        let object_at_new_pos = map[new_obj_y][new_obj_x];
                        match object_at_new_pos {
                            '.' => {
                                map[new_obj_y].remove(new_obj_x);
                                map[new_obj_y].insert(current_position.0, '.');
                                current_position = (new_x, new_y);
                                break;
                            }
                            '[' | ']' => {
                                i += 1;
                            }
                            '#' => break,
                            _ => unreachable!(),
                        }
                    }
                } else {
                    let neighbours =
                        get_vertical_push_neighbours((new_x, new_y), (*dx, *dy), &map, false);
                    if neighbours.is_none() {
                        continue;
                    }
                    for (n_x, n_y) in neighbours.unwrap().iter().sorted_by(|a, b| {
                        if *dy == 1 {
                            b.1.cmp(&a.1).then(a.0.cmp(&b.0))
                        } else {
                            a.1.cmp(&b.1).then(a.0.cmp(&b.0))
                        }
                    }) {
                        let new_obj_y = (*n_y as i32 + dy) as usize;
                        map[new_obj_y][*n_x] = map[*n_y][*n_x];
                        map[*n_y][*n_x] = '.';
                    }
                    map[current_position.1][current_position.0] = '.';
                    current_position = (new_x, new_y);
                    map[new_y][new_x] = '@';
                }
            }
            _ => unreachable!(),
        }
    }
    let mut result = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, &char) in line.iter().enumerate() {
            if char == '[' {
                result += 100 * y + x;
            }
        }
    }
    println!("{result:?}");
}

fn get_vertical_push_neighbours(
    position: (usize, usize),
    (dx, dy): (i32, i32),
    map: &[Vec<char>],
    is_neighbour: bool,
) -> Option<HashSet<(usize, usize)>> {
    let mut hs = HashSet::new();
    hs.insert((position.0, position.1));
    let is_left = map[position.1][position.0] == '[';

    let neigh_brack_x = if is_left {
        position.0 + 1
    } else {
        position.0 - 1
    };
    if !is_neighbour {
        if let Some(neighs) =
            &get_vertical_push_neighbours((neigh_brack_x, position.1), (dx, dy), map, true)
        {
            hs = hs.union(neighs).copied().collect();
        } else {
            return None;
        };
    }

    let new_obj_x = (position.0 as i32 + dx) as usize;
    let new_obj_y = (position.1 as i32 + dy) as usize;

    let next_obj = map[new_obj_y][new_obj_x];
    match next_obj {
        '.' => {}
        '[' | ']' => {
            if let Some(neighs) =
                &get_vertical_push_neighbours((new_obj_x, new_obj_y), (dx, dy), map, false)
            {
                hs = hs.union(neighs).copied().collect();
            } else {
                return None;
            };
        }
        _ => return None,
    }
    Some(hs)
}
