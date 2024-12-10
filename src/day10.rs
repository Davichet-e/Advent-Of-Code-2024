use std::collections::{HashMap, HashSet};

const PATTERNS: [[i16; 2]; 4] = [
    [0, 1],  // Horizontal down
    [0, -1], // Horizontal up
    [1, 0],  // Sequential
    [-1, 0], // Reverse sequential
];

fn get_number(grid: &[&str], y: usize, x: usize) -> usize {
    grid[y].chars().nth(x).unwrap().to_digit(10).unwrap() as usize
}

fn is_in_grid(x: i16, y: i16, width: usize, height: usize) -> bool {
    x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height
}

fn part1(
    (x, y): (usize, usize),
    initial_position: (usize, usize),
    prev_number: Option<usize>,
    grid: &[&str],
    initial_to_final_pos: &mut HashMap<(usize, usize), HashSet<(usize, usize)>>,
) {
    let current_number = get_number(grid, y, x);
    if let Some(prev) = prev_number {
        if current_number != prev + 1 {
            return;
        }
    }

    if current_number == 9 {
        initial_to_final_pos
            .entry(initial_position)
            .or_default()
            .insert((x, y));
    } else {
        let height = grid.len();
        let width = grid[0].len();

        for &[dx, dy] in &PATTERNS {
            let new_x = x as i16 + dx;
            let new_y = y as i16 + dy;
            if is_in_grid(new_x, new_y, width, height) {
                part1(
                    (new_x as usize, new_y as usize),
                    initial_position,
                    Some(current_number),
                    grid,
                    initial_to_final_pos,
                );
            }
        }
    }
}

fn part2((x, y): (usize, usize), prev_number: Option<usize>, grid: &[&str]) -> usize {
    let current_number = get_number(grid, y, x);
    if let Some(prev) = prev_number {
        if current_number != prev + 1 {
            return 0;
        }
    }

    if current_number == 9 {
        1
    } else {
        let height = grid.len();
        let width = grid[0].len();

        PATTERNS
            .iter()
            .filter_map(|&[dx, dy]| {
                let new_x = x as i16 + dx;
                let new_y = y as i16 + dy;
                if is_in_grid(new_x, new_y, width, height) {
                    Some(part2(
                        (new_x as usize, new_y as usize),
                        Some(current_number),
                        grid,
                    ))
                } else {
                    None
                }
            })
            .sum()
    }
}

pub fn day10() {
    let input = include_str!("../inputs/day10");
    let grid: Vec<&str> = input.lines().collect();
    let height = grid.len();
    let width = grid[0].len();

    let mut initial_to_final_pos = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            if get_number(&grid, y, x) == 0 {
                part1((x, y), (x, y), None, &grid, &mut initial_to_final_pos);
            }
        }
    }

    let part1_result: usize = initial_to_final_pos.values().map(HashSet::len).sum();
    println!("{part1_result}");

    let part2_result: usize = (0..height)
        .map(|y| {
            (0..width)
                .filter(|&x| get_number(&grid, y, x) == 0)
                .map(|x| part2((x, y), None, &grid))
                .sum::<usize>()
        })
        .sum();

    println!("{part2_result}");
}
