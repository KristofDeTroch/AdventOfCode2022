use std::collections::HashSet;

fn main() {
    let contents = include_str!("input.txt");
    println!("Part 1: {:?}", part_1(contents));
    println!("Part 2: {:?}", part_2(contents));
}

fn part_1(contents: &str) -> i32 {
    let marker_size = 4;
    let chararcters = contents.chars().collect::<Vec<char>>();
    for index in marker_size..chararcters.len() {
        let current_marker = &contents[index - marker_size..index];
        if is_marker(current_marker) {
            return index as i32;
        }
    }
    return 0;
}

fn part_2(contents: &str) -> i32 {
    let message_size = 14;
    let chararcters = contents.chars().collect::<Vec<char>>();
    for index in message_size..chararcters.len() {
        let current_marker = &contents[index - message_size..index];
        if is_marker(current_marker) {
            return index as i32;
        }
    }
    return 0;
}

fn is_marker(marker: &str) -> bool {
    let mut a = marker.chars().collect::<Vec<char>>();
    let mut uniques = HashSet::new();
    a.retain(|e| uniques.insert(e.clone()));
    return a.len() == marker.len();
}
