use std::collections::{HashMap, HashSet};

fn is_in_grid((x, y): (i16, i16), input_width: usize, input_height: usize) -> bool {
    x >= 0 && y >= 0 && (x as usize) < input_width && (y as usize) < input_height
}

fn calculate_addition((x, y): (usize, usize), (dx, dy): (i16, i16)) -> (i16, i16) {
    (x as i16 + dx as i16, y as i16 + dy as i16)
}

fn part1(input: &[&str]) {
    let input_height = input.len();
    let input_width = input[0].len();

    let mut map: HashMap<char, Vec<(usize, usize)>> = Default::default();
    let mut antiantennas = HashSet::new();

    input.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c.is_ascii_alphanumeric() {
                let current_position = (x, y);
                let antennas = map.entry(c).or_default();

                antennas.iter().for_each(|(antenna_x, antenna_y)| {
                    let (dx, dy) = calculate_addition(
                        current_position,
                        (-(*antenna_x as i16), -(*antenna_y as i16)),
                    );
                    let potential_antiantena = calculate_addition(current_position, (dx, dy));
                    if is_in_grid(potential_antiantena, input_width, input_height) {
                        antiantennas.insert(potential_antiantena);
                    }

                    let potential_antiantena =
                        calculate_addition((*antenna_x, *antenna_y), (-dx, -dy));
                    if is_in_grid(potential_antiantena, input_width, input_height) {
                        antiantennas.insert(potential_antiantena);
                    }
                });

                antennas.push(current_position);
            }
        });
    });
    let result = antiantennas.len();
    println!("{result}");
}

fn part2(input: &[&str]) {
    let input_height = input.len();
    let input_width = input[0].len();

    let mut map: HashMap<char, Vec<(usize, usize)>> = Default::default();
    let mut antiantennas = HashSet::new();

    input.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c.is_ascii_alphanumeric() {
                let current_position = (x, y);
                let antennas = map.entry(c).or_default();

                antennas.iter().for_each(|(antenna_x, antenna_y)| {
                    let (dx, dy) = calculate_addition(
                        current_position,
                        (-(*antenna_x as i16), -(*antenna_y as i16)),
                    );
                    antiantennas.insert(current_position);
                    let mut potential_antiantena = calculate_addition(current_position, (dx, dy));
                    while is_in_grid(potential_antiantena, input_width, input_height) {
                        let antiantenna_as_usize = (
                            potential_antiantena.0 as usize,
                            potential_antiantena.1 as usize,
                        );
                        antiantennas.insert(antiantenna_as_usize);
                        potential_antiantena = calculate_addition(antiantenna_as_usize, (dx, dy));
                    }

                    antiantennas.insert((*antenna_x, *antenna_y));
                    let mut potential_antiantena =
                        calculate_addition((*antenna_x, *antenna_y), (-dx, -dy));
                    while is_in_grid(potential_antiantena, input_width, input_height) {
                        let antiantenna_as_usize = (
                            potential_antiantena.0 as usize,
                            potential_antiantena.1 as usize,
                        );
                        antiantennas.insert(antiantenna_as_usize);
                        potential_antiantena = calculate_addition(antiantenna_as_usize, (-dx, -dy));
                    }
                });

                antennas.push(current_position);
            }
        });
    });
    let result = antiantennas.len();
    println!("{result}");
}

pub fn day8() {
    let string = include_str!("../inputs/day8");
    let input: Vec<&str> = string.lines().collect();

    part1(&input);
    part2(&input);
}
