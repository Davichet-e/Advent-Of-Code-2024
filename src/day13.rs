use regex::Regex;

pub fn day13() {
    let string = include_str!("../inputs/day13");

    let button_regex = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let sum: i64 = string
        .trim_end()
        .split("\n\n")
        .map(|cm| {
            let mut lines = cm.lines();
            let button1 = button_regex.captures(lines.next().unwrap()).unwrap();
            let button1_x = button1.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let button1_y = button1.get(2).unwrap().as_str().parse::<i64>().unwrap();

            let button2 = button_regex.captures(lines.next().unwrap()).unwrap();
            let button2_x = button2.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let button2_y = button2.get(2).unwrap().as_str().parse::<i64>().unwrap();

            let prize = prize_regex.captures(lines.next().unwrap()).unwrap();
            let prize_x = prize.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let prize_y = prize.get(2).unwrap().as_str().parse::<i64>().unwrap();

            (
                (button1_x, button1_y),
                (button2_x, button2_y),
                (prize_x + 10000000000000, prize_y + 10000000000000), // Delete the additions for part1
            )
        })
        .map(
            |((button1_x, button1_y), (button2_x, button2_y), (prize_x, prize_y))| {
                let det = button1_x * button2_y - button1_y * button2_x;

                let a = (prize_x * button2_y - prize_y * button2_x) / det;
                let b = (button1_x * prize_y - button1_y * prize_x) / det;

                let computed_prize = (button1_x * a + button2_x * b, button1_y * a + button2_y * b);

                if computed_prize == (prize_x, prize_y) {
                    3 * a + b
                } else {
                    0
                }
            },
        )
        .sum();

    println!("{sum}")
}
