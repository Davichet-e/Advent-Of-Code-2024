use grouping_by::GroupingBy;

const HEIGHT: i32 = 103;
const WIDTH: i32 = 101;

pub fn day14() {
    let string = include_str!("../inputs/day14");

    let positions: Vec<((i32, i32), (i32, i32))> = string
        .lines()
        .filter_map(|line| {
            let (p_x, rest) = line.strip_prefix("p=")?.split_once(',')?;
            let (p_y, rest) = rest.split_once(' ')?;

            let (v_x, v_y) = rest.strip_prefix("v=")?.split_once(',')?;

            Some((
                (p_x.parse().unwrap(), p_y.parse().unwrap()),
                (v_x.parse().unwrap(), v_y.parse().unwrap()),
            ))
        })
        .collect();

    part1(&positions);

    part2(&positions);
}

fn part2(positions: &[((i32, i32), (i32, i32))]) {
    let mut positions_iter: Vec<_> = positions.to_vec();

    let mut r = 0;
    loop {
        r += 1;
        positions_iter = positions_iter
            .iter()
            .map(|((p_x, p_y), (v_x, v_y))| {
                let new_x = (p_x + v_x).rem_euclid(WIDTH);

                let new_y = (p_y + v_y).rem_euclid(HEIGHT);
                ((new_x, new_y), (*v_x, *v_y))
            })
            .collect();
        if positions_iter
            .iter()
            .counter(|((x, _), _)| x)
            .values()
            .any(|v| *v > 20)
            && positions_iter
                .iter()
                .counter(|((_, y), _)| y)
                .values()
                .any(|v| *v > 20)
        {
            break;
        }
    }
    println!("{r}");
}

fn part1(positions: &[((i32, i32), (i32, i32))]) {
    let positions: Vec<_> = positions
        .iter()
        .map(|((p_x, p_y), (v_x, v_y))| {
            let new_x = (p_x + 100 * v_x).rem_euclid(WIDTH);

            let new_y = (p_y + 100 * v_y).rem_euclid(HEIGHT);
            ((new_x, new_y), (v_x, v_y))
        })
        .collect();

    let result = positions
        .into_iter()
        .filter(|((x, y), _)| *x != WIDTH / 2 && *y != HEIGHT / 2)
        .counter(|((x, y), _)| {
            let mut cuadrant = 1;
            if *x > WIDTH / 2 {
                cuadrant += 1;
            }

            if *y > HEIGHT / 2 {
                cuadrant += 2;
            }

            cuadrant
        })
        .values()
        .product::<usize>();

    println!("{result:?}");
}
