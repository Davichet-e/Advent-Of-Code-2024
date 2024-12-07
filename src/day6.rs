use std::collections::HashSet;

fn part1(initial_position: (usize, usize), lines: &[&str]) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut actual_position = initial_position;

    let (input_width, input_height) = (lines[0].len(), lines.len());

    let mut deltas = (0, -1);
    while let Some((new_x, new_y)) =
        get_new_coordinates(actual_position, deltas, (input_width, input_height))
    {
        match lines[new_y].chars().nth(new_x).unwrap() {
            '#' => deltas = (-deltas.1, deltas.0),
            _ => {
                actual_position = (new_x, new_y);
                visited.insert((new_x, new_y));
            }
        }
    }
    visited
}

fn get_new_coordinates(
    actual_position: (usize, usize),
    (delta_x, delta_y): (i32, i32),
    (input_width, input_height): (usize, usize),
) -> Option<(usize, usize)> {
    let (new_x, new_y) = (
        actual_position.0 as i32 + delta_x,
        actual_position.1 as i32 + delta_y,
    );
    if new_x >= 0 && new_x < input_width as i32 && new_y >= 0 && new_y < input_height as i32 {
        Some((new_x as usize, new_y as usize))
    } else {
        None
    }
}

fn part2(
    seen_positions: HashSet<(usize, usize)>,
    initial_position: (usize, usize),
    input: Vec<&str>,
) {
    let result: usize = seen_positions
        .into_iter()
        .filter(|(x, y)| {
            if (*x, *y) == initial_position {
                return false;
            }

            is_there_loop(&input, initial_position, (*x, *y))
        })
        .count();
    println!("Part 2: {result}");
}

fn is_there_loop(
    lines: &[&str],
    initial_position: (usize, usize),
    new_obstacle: (usize, usize),
) -> bool {
    let mut visited_with_direction = HashSet::new();
    let mut actual_position = initial_position;

    let (input_width, input_height) = (lines[0].len(), lines.len());

    let mut deltas = (0, -1);
    while let Some((new_x, new_y)) =
        get_new_coordinates(actual_position, deltas, (input_width, input_height))
    {
        match lines[new_y].chars().nth(new_x).unwrap() {
            '#' => deltas = (-deltas.1, deltas.0),
            _ if (new_x, new_y) == new_obstacle => deltas = (-deltas.1, deltas.0),
            _ => {
                actual_position = (new_x, new_y);

                if visited_with_direction.contains(&(actual_position, deltas)) {
                    return true;
                }
                visited_with_direction.insert(((new_x, new_y), deltas));
            }
        }
    }
    false
}

pub fn day6() {
    let string = include_str!("../inputs/day6");
    let input: Vec<&str> = string.lines().collect();

    let initial_position = input
        .iter()
        .enumerate()
        .find_map(|(y, line)| line.find('^').map(|x| (x, y)))
        .unwrap();

    let seen_positions = part1(initial_position, &input);
    println!("Part 1: {}", seen_positions.len());

    part2(seen_positions, initial_position, input);
}
