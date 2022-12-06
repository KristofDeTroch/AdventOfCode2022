fn main() {
    let contents = include_str!("input.txt");
    let stacks = create_stacks(contents);
    println!("Part 1: {:?}", part_1(contents, stacks.clone()));
    println!("Part 2: {:?}", part_2(contents, stacks.clone()));
}

fn part_1(contents: &str, mut stacks: Vec<Vec<String>>) -> String {
    let mut status = false;
    for line in contents.split('\n') {
        if line.is_empty() {
            status = !status;
            continue;
        }
        if status {
            let command = line.split(" ").collect::<Vec<&str>>();
            let amount = command.get(1).unwrap().to_string().parse::<i32>().unwrap();
            let from = command.get(3).unwrap().to_string().parse::<i32>().unwrap() - 1;
            let to = command.get(5).unwrap().to_string().parse::<i32>().unwrap() - 1;
            for _ in 0..amount {
                let current_crate = stacks.get_mut(from as usize).unwrap().pop().unwrap();
                stacks.get_mut(to as usize).unwrap().push(current_crate);
            }
        }
    }
    let mut result = "".to_string();
    stacks
        .iter()
        .for_each(|v| result.push_str(v.last().unwrap()));
    return result;
}

fn part_2(contents: &str, mut stacks: Vec<Vec<String>>) -> String {
    let mut status = false;
    for line in contents.split('\n') {
        if line.is_empty() {
            status = !status;
            continue;
        }
        if status {
            let command = line.split(" ").collect::<Vec<&str>>();
            let amount = command.get(1).unwrap().to_string().parse::<i32>().unwrap();
            let from = command.get(3).unwrap().to_string().parse::<i32>().unwrap() - 1;
            let to = command.get(5).unwrap().to_string().parse::<i32>().unwrap() - 1;
            let mut temp_stack = Vec::<String>::new();
            for _ in 0..amount {
                let current_crate = stacks.get_mut(from as usize).unwrap().pop().unwrap();
                temp_stack.push(current_crate);
            }
            for _ in 0..amount {
                let current_crate = temp_stack.pop().unwrap();
                stacks.get_mut(to as usize).unwrap().push(current_crate);
            }
        }
    }
    let mut result = "".to_string();
    stacks
        .iter()
        .for_each(|v| result.push_str(v.last().unwrap()));
    return result;
}

fn create_stacks(contents: &str) -> Vec<Vec<String>> {
    let mut crates = Vec::<Vec<String>>::new();
    let index_of_crates = [1, 5, 9, 13, 17, 21, 25, 29, 33];

    for _ in 0..9 {
        crates.push(Vec::<String>::new())
    }

    for line in contents.split('\n') {
        if line.is_empty() {
            break;
        }
        let current_line_of_crates: Vec<String> = index_of_crates
            .iter()
            .map(|i| line.chars().nth(*i).unwrap().to_string())
            .collect();
        for i in 0..9 {
            if current_line_of_crates[i] != " " {
                crates[i].push(current_line_of_crates.get(i).unwrap().to_string())
            }
        }
    }
    let mut result = Vec::<Vec<String>>::new();
    crates.into_iter().for_each(|mut v| {
        v.pop();
        v.reverse();
        result.push(v)
    });
    return result;
}
