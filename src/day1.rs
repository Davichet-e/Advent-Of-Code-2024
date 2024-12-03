fn part1(first_group: &[usize], second_group: &[usize]) {
    let sum_of_distances = first_group
        .iter()
        .zip(second_group.iter())
        .fold(0, |acc, (&n1, &n2)| acc + n1.abs_diff(n2));

    println!("{sum_of_distances}");
}

fn part2(first_group: &[usize], second_group: &[usize]) {
    let mut similarity_score = 0;
    for n in first_group {
        // Could be improved using a HM as a cache
        similarity_score += n * second_group.iter().filter(|&n2| n == n2).count();
    }
    println!("{similarity_score}");
}

pub fn day1() {
    let string = include_str!("../inputs/day1");

    let (mut first_group, mut second_group): (Vec<usize>, Vec<usize>) = string
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(' ').unwrap();
            let l: usize = l.trim().parse().unwrap();
            let r: usize = r.trim().parse().unwrap();
            (l, r)
        })
        .unzip();
    first_group.sort_unstable();
    second_group.sort_unstable();

    part1(&first_group, &second_group);
    part2(&first_group, &second_group);
}
