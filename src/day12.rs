use std::collections::HashMap;

const PATTERNS: [[i16; 2]; 4] = [
    [0, 1],  // Down
    [0, -1], // Up
    [1, 0],  // Sequential
    [-1, 0], // Reverse sequential
];

fn get_char(grid: &[&str], y: usize, x: usize) -> char {
    grid[y].chars().nth(x).unwrap()
}

fn is_in_grid(x: i16, y: i16, width: usize, height: usize) -> bool {
    x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height
}

pub fn day12() {
    let string = include_str!("../inputs/day12");

    let grid: Vec<&str> = string.lines().collect();

    let mut accumulator: HashMap<(usize, usize, char), (usize, usize)> = HashMap::new();
    let mut map_initial_pos: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut reverse_map_initial_pos: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    let height = grid.len();
    let width = grid[0].len();

    const PART2: bool = false;

    for (y, line) in grid.iter().enumerate() {
        let mut last_char: Option<char> = None;
        for (x, c) in line.chars().enumerate() {
            let mut entry = if last_char.is_some_and(|lc| lc == c) {
                let initial_pos = *map_initial_pos.get(&(x - 1, y)).unwrap();
                map_initial_pos.insert((x, y), initial_pos);
                reverse_map_initial_pos
                    .get_mut(&initial_pos)
                    .unwrap()
                    .push((x, y));

                (initial_pos.0, initial_pos.1, c)
            } else {
                accumulator.insert((x, y, c), (0, 0));
                map_initial_pos.insert((x, y), (x, y));
                reverse_map_initial_pos.insert((x, y), vec![(x, y)]);
                (x, y, c)
            };

            if let Some(neighbour) = get_top_neighbour((x, y), &grid) {
                if neighbour == c {
                    let initial_pos = *map_initial_pos.get(&(x, y - 1)).unwrap();
                    if initial_pos.0 != entry.0 || initial_pos.1 != entry.1 {
                        let (a, p) = accumulator.get(&entry).copied().unwrap();
                        let curr = accumulator
                            .get_mut(&(initial_pos.0, initial_pos.1, c))
                            .unwrap();
                        *curr = (curr.0 + a, curr.1 + p);
                        accumulator.remove(&entry);
                        let mut old = reverse_map_initial_pos
                            .get_mut(&(entry.0, entry.1))
                            .unwrap()
                            .clone();
                        for a in &old {
                            *map_initial_pos.get_mut(a).unwrap() = (initial_pos.0, initial_pos.1);
                        }
                        reverse_map_initial_pos
                            .get_mut(&(initial_pos.0, initial_pos.1))
                            .unwrap()
                            .append(&mut old);

                        *map_initial_pos.get_mut(&(x, y)).unwrap() = (initial_pos.0, initial_pos.1);

                        reverse_map_initial_pos.remove(&(entry.0, entry.1));

                        entry = (initial_pos.0, initial_pos.1, c);
                    }
                }
            }
            let new_entry = accumulator.get_mut(&entry).unwrap();
            let mut d_perimeter = 0;

            if PART2 {
                let new_x = x as i16;
                let new_y = y as i16 + 1;
                if !is_in_grid(new_x, new_y, width, height)
                    || get_char(&grid, new_y as usize, new_x as usize) != c
                {
                    let new_x = x as i16 - 1;
                    let new_y = y as i16;

                    if !is_in_grid(new_x, new_y, width, height)
                        || get_char(&grid, new_y as usize, new_x as usize) != c
                    {
                        d_perimeter += 1;
                    } else {
                        let new_x = x as i16 - 1;
                        let new_y = y as i16 + 1;

                        if is_in_grid(new_x, new_y, width, height)
                            && get_char(&grid, new_y as usize, new_x as usize) == c
                        {
                            d_perimeter += 1;
                        }
                    }
                }

                let new_x = x as i16;
                let new_y = y as i16 - 1;
                if !is_in_grid(new_x, new_y, width, height)
                    || get_char(&grid, new_y as usize, new_x as usize) != c
                {
                    if c == 'C' {
                        println!("2");
                    }
                    let new_x = x as i16 - 1;
                    let new_y = y as i16;

                    if !is_in_grid(new_x, new_y, width, height)
                        || get_char(&grid, new_y as usize, new_x as usize) != c
                    {
                        d_perimeter += 1;
                    } else {
                        let new_x = x as i16 - 1;
                        let new_y = y as i16 - 1;

                        if is_in_grid(new_x, new_y, width, height)
                            && get_char(&grid, new_y as usize, new_x as usize) == c
                        {
                            d_perimeter += 1;
                        } else {
                            let new_x = x as i16 - 1;
                            let new_y = y as i16 - 1;

                            if is_in_grid(new_x, new_y, width, height)
                                && get_char(&grid, new_y as usize, new_x as usize) == c
                            {
                                d_perimeter += 1;
                            }
                        }
                    }
                }

                let new_x = x as i16 + 1;
                let new_y = y as i16;
                if !is_in_grid(new_x, new_y, width, height)
                    || get_char(&grid, new_y as usize, new_x as usize) != c
                {
                    let new_x = x as i16;
                    let new_y = y as i16 - 1;

                    if !is_in_grid(new_x, new_y, width, height)
                        || get_char(&grid, new_y as usize, new_x as usize) != c
                    {
                        d_perimeter += 1;
                    } else {
                        let new_x = x as i16 + 1;
                        let new_y = y as i16 - 1;

                        if is_in_grid(new_x, new_y, width, height)
                            && get_char(&grid, new_y as usize, new_x as usize) == c
                        {
                            d_perimeter += 1;
                        }
                    }
                }

                let new_x = x as i16 - 1;
                let new_y = y as i16;
                if !is_in_grid(new_x, new_y, width, height)
                    || get_char(&grid, new_y as usize, new_x as usize) != c
                {
                    let new_x = x as i16;
                    let new_y = y as i16 - 1;

                    if !is_in_grid(new_x, new_y, width, height)
                        || get_char(&grid, new_y as usize, new_x as usize) != c
                    {
                        d_perimeter += 1;
                    } else {
                        let new_x = x as i16 - 1;
                        let new_y = y as i16 - 1;

                        if is_in_grid(new_x, new_y, width, height)
                            && get_char(&grid, new_y as usize, new_x as usize) == c
                        {
                            d_perimeter += 1;
                        }
                    }
                }
            } else {
                for [dx, dy] in PATTERNS {
                    let new_x = x as i16 + dx;
                    let new_y = y as i16 + dy;

                    if !is_in_grid(new_x, new_y, width, height)
                        || get_char(&grid, new_y as usize, new_x as usize) != c
                    {
                        d_perimeter += 1;
                    }
                }
            }
            *new_entry = (new_entry.0 + 1, new_entry.1 + d_perimeter);
            last_char = Some(c);
        }
    }

    dbg!(accumulator.values().map(|(a, p)| a * p).sum::<usize>());
}

fn get_top_neighbour((x, y): (usize, usize), grid: &[&str]) -> Option<char> {
    let height = grid.len();
    let width = grid[0].len();

    let new_x = x as i16;
    let new_y = y as i16 - 1;

    if is_in_grid(new_x, new_y, width, height) {
        Some(get_char(grid, new_y as usize, new_x as usize))
    } else {
        None
    }
}
