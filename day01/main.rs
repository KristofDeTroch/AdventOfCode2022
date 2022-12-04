fn main() {
    let contents = include_str!("input.txt");
    let mut current_value = 0;
    let mut calories: Vec<i32> = Vec::new();
    for line in contents.lines() {
        if line.is_empty() {
            calories.push(current_value);
            current_value = 0;
        } else {
            current_value += line.parse::<i32>().unwrap();
        }
    }
    calories.sort();
    calories.reverse();
    println!("{}", calories[0]);
    println!("{}", calories[0..3].iter().sum::<i32>())
}
