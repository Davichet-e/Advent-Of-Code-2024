const SAMPLE: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

fn check_sequential(string: &str) -> usize {
    string.matches("XMAS").count()
}

const PATTERS_P1: [[[i8; 2]; 3]; 6] = [
    [[1, 1], [2, 2], [3, 3]],       // Diagonal left down
    [[1, -1], [2, -2], [3, -3]],    // Diagonal right down
    [[-1, 1], [-2, 2], [-3, 3]],    // Diagonal left up
    [[-1, -1], [-2, -2], [-3, -3]], // Diagonal right up
    [[1, 0], [2, 0], [3, 0]],       // Horiozontal down
    [[-1, 0], [-2, 0], [-3, 0]],    // Horizontal up
];

fn check_patterns_p1(input: &[&str], x: usize, y: usize) -> usize {
    PATTERS_P1
        .iter()
        .filter(|pattern| {
            pattern.iter().enumerate().all(|(i, [delta_y, delta_x])| {
                input
                    .get((y as i8 + delta_y) as usize)
                    .unwrap_or(&"")
                    .chars()
                    .nth((x as i8 + delta_x) as usize)
                    .unwrap_or('Z')
                    == "MAS".chars().nth(i).unwrap()
            })
        })
        .count()
}

fn part1(string: &str) {
    let input: Vec<&str> = string.lines().collect();
    let result = input.iter().enumerate().fold(0, |acc, (y, line)| {
        let count: usize = line
            .chars()
            .enumerate()
            .map(|(x, c)| {
                if c == 'X' {
                    check_patterns_p1(&input, x, y)
                } else {
                    0
                }
            })
            .sum();
        acc + count
            + check_sequential(line)
            + check_sequential(&line.chars().rev().collect::<String>())
    });

    println!("{result}");
}

const PATTERS_P2: [[(i8, i8, char); 4]; 4] = [
    [(-1, -1, 'M'), (1, 1, 'S'), (1, -1, 'M'), (-1, 1, 'S')],
    [(-1, -1, 'S'), (1, 1, 'M'), (1, -1, 'S'), (-1, 1, 'M')],
    [(-1, -1, 'M'), (1, 1, 'S'), (1, -1, 'S'), (-1, 1, 'M')],
    [(-1, -1, 'S'), (1, 1, 'M'), (1, -1, 'M'), (-1, 1, 'S')],
];

fn check_patterns_p2(input: &[&str], x: usize, y: usize) -> usize {
    PATTERS_P2
        .iter()
        .filter(|pattern| {
            pattern.iter().all(|(delta_y, delta_x, c)| {
                input
                    .get((y as i8 + delta_y) as usize)
                    .unwrap_or(&"")
                    .chars()
                    .nth((x as i8 + delta_x) as usize)
                    .unwrap_or('Z')
                    == *c
            })
        })
        .count()
}

fn part2(string: &str) {
    let input: Vec<&str> = string.lines().collect();
    let result = input.iter().enumerate().fold(0, |acc, (y, line)| {
        let count: usize = line
            .chars()
            .enumerate()
            .map(|(x, c)| {
                if c == 'A' {
                    check_patterns_p2(&input, x, y)
                } else {
                    0
                }
            })
            .sum();
        acc + count
    });

    println!("{result}");
}

pub fn day4() {
    let string = include_str!("../inputs/day4");

    part1(string);
    part2(string);
}
