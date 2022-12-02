use std::collections::HashMap;

fn main() {
    let mut opponentMove = HashMap::<char, i32>::new();
    opponentMove.insert('A', 1);
    opponentMove.insert('B', 2);
    opponentMove.insert('C', 3);

    let mut myMove = HashMap::<char, i32>::new();
    myMove.insert('X', 1);
    myMove.insert('Y', 2);
    myMove.insert('Z', 3);

    let mut outcome = HashMap::<i32, i32>::new();
    outcome.insert(-2, 0);
    outcome.insert(-1, 6);
    outcome.insert(0, 3);
    outcome.insert(1, 0);
    outcome.insert(2, 6);

    let mut myMovePart2 = HashMap::<char, i32>::new();
    myMovePart2.insert('X', 0);
    myMovePart2.insert('Y', 3);
    myMovePart2.insert('Z', 6);

    let mut outcomePart2 = HashMap::<i32, i32>::new();
    outcomePart2.insert(1, 3);
    outcomePart2.insert(2, 1);
    outcomePart2.insert(3, 2);
    outcomePart2.insert(4, 1);
    outcomePart2.insert(5, 2);
    outcomePart2.insert(6, 3);
    outcomePart2.insert(7, 2);
    outcomePart2.insert(8, 3);
    outcomePart2.insert(9, 1);

    let contents = include_str!("../input.txt");
    let mut score = 0;
    let mut scorePart2 = 0;
    for line in contents.split('\n') {
        if line.is_empty() {
            break;
        }
        let opponentCurrentMove = opponentMove.get(&line.chars().nth(0).unwrap()).unwrap();
        let myCurrentMove = myMove.get(&line.chars().nth(2).unwrap()).unwrap();
        let myCurrentMovePart2 = myMovePart2.get(&line.chars().nth(2).unwrap()).unwrap();
        score += myCurrentMove + outcome.get(&(opponentCurrentMove - myCurrentMove)).unwrap();
        let temp = myCurrentMovePart2
            + outcomePart2
                .get(&(opponentCurrentMove + myCurrentMovePart2))
                .unwrap();
        scorePart2 += temp;
    }
    std::println!("{}", score);
    std::println!("{}", scorePart2);
}
