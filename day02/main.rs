use std::collections::HashMap;

fn main() {
    let mut opponent_move = HashMap::<char, i32>::new();
    opponent_move.insert('A', 1);
    opponent_move.insert('B', 2);
    opponent_move.insert('C', 3);

    let mut my_move = HashMap::<char, i32>::new();
    my_move.insert('X', 1);
    my_move.insert('Y', 2);
    my_move.insert('Z', 3);

    let mut outcome = HashMap::<i32, i32>::new();
    outcome.insert(-2, 0);
    outcome.insert(-1, 6);
    outcome.insert(0, 3);
    outcome.insert(1, 0);
    outcome.insert(2, 6);

    let mut my_move_part2 = HashMap::<char, i32>::new();
    my_move_part2.insert('X', 0);
    my_move_part2.insert('Y', 3);
    my_move_part2.insert('Z', 6);

    let mut outcome_part2 = HashMap::<i32, i32>::new();
    outcome_part2.insert(1, 3);
    outcome_part2.insert(2, 1);
    outcome_part2.insert(3, 2);
    outcome_part2.insert(4, 1);
    outcome_part2.insert(5, 2);
    outcome_part2.insert(6, 3);
    outcome_part2.insert(7, 2);
    outcome_part2.insert(8, 3);
    outcome_part2.insert(9, 1);

    let contents = include_str!("input.txt");
    let mut score = 0;
    let mut score_part2 = 0;
    for line in contents.split('\n') {
        if line.is_empty() {
            break;
        }
        let opponent_current_move = opponent_move.get(&line.chars().nth(0).unwrap()).unwrap();
        let my_current_move = my_move.get(&line.chars().nth(2).unwrap()).unwrap();
        let my_current_move_part2 = my_move_part2.get(&line.chars().nth(2).unwrap()).unwrap();
        score += my_current_move
            + outcome
                .get(&(opponent_current_move - my_current_move))
                .unwrap();
        let temp = my_current_move_part2
            + outcome_part2
                .get(&(opponent_current_move + my_current_move_part2))
                .unwrap();
        score_part2 += temp;
    }
    std::println!("{}", score);
    std::println!("{}", score_part2);
}
