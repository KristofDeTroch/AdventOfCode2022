use std::collections::HashSet;

fn main(){
  let contents = include_str!("../input.txt");
  part1(contents);
  part2(contents);
}

fn part1(contents: &str){
  let mut all_mistakes = Vec::<u32>::new();
  for line in contents.split('\n') {
    if line.is_empty(){ continue;}
    let lhs = &line[..line.len()/2];
    let rhs = &line[line.len()/2..];

    let set: HashSet<char> = lhs.chars().collect();
    let mistake: Vec<char> = rhs.chars().into_iter().filter(|c| set.contains(c)).collect();
    all_mistakes.push(get_priority(mistake.first().unwrap().clone()));
  }
  println!("Part 1 solution: {}", all_mistakes.iter().sum::<u32>())
}

fn part2(contents: &str){
  let mut all_badges = Vec::<u32>::new();
  let lines:Vec<&str> = contents.split('\n').collect();
  for index in (0..lines.len()-1).step_by(3){
    let first = lines[index];
    let second = lines[index+1];
    let third = lines[index+2];
    let set: HashSet<char> = first.chars().collect();
    let temp: Vec<char> = second.chars().into_iter().filter(|c| set.contains(c)).collect();
    let badge: Vec<char> = third.chars().into_iter().filter(|c| temp.contains(c)).collect();
    all_badges.push(get_priority(badge.first().unwrap().clone()));
  }
  println!("Part 2 solution: {}", all_badges.iter().sum::<u32>())
}



fn get_priority(letter:char)->u32{
  if letter.is_lowercase(){
    return letter as u32 - 96;
  }else{
    return letter as u32 - 38;
  }
}