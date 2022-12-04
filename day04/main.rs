fn main() {
    let contents = include_str!("input.txt");
    println!("Part 1: {}", part1(contents));
    println!("Part 2: {}", part2(contents));
}

fn part1(contents: &str) -> i32 {
    let mut count = 0;
    for line in contents.split("\n") {
        if line.is_empty() {
            continue;
        }
        let assignments = line
            .split(',')
            .map(|c| c.split('-').collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();
        if is_over_lapping(
            assignments.get(0).unwrap().to_vec(),
            assignments.get(1).unwrap().to_vec(),
            true,
        ) {
            count += 1;
        }
    }
    return count;
}

fn part2(contents: &str) -> i32 {
    let mut count = 0;
    for line in contents.split("\n") {
        if line.is_empty() {
            continue;
        }
        let assignments = line
            .split(',')
            .map(|c| c.split('-').collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();
        if is_over_lapping(
            assignments.get(0).unwrap().to_vec(),
            assignments.get(1).unwrap().to_vec(),
            false,
        ) {
            count += 1;
        }
    }
    return count;
}

fn is_over_lapping(lhs: Vec<&str>, rhs: Vec<&str>, is_strict: bool) -> bool {
    let lhs_low = lhs[0].parse::<i32>().unwrap();
    let lhs_high = lhs[1].parse::<i32>().unwrap();
    let rhs_low = rhs[0].parse::<i32>().unwrap();
    let rhs_high = rhs[1].parse::<i32>().unwrap();
    if is_strict {
        return (lhs_low <= rhs_low && rhs_high <= lhs_high)
            || (rhs_low <= lhs_low && lhs_high <= rhs_high);
    } else {
        return (lhs_low <= rhs_high && rhs_high <= lhs_high)
            || (rhs_low <= lhs_high && lhs_high <= rhs_high)
            || (lhs_low <= rhs_low && rhs_low <= lhs_high)
            || (rhs_low <= lhs_low && lhs_low <= rhs_high);
    }
}
