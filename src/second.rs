use crate::utils;

fn get_score(player: &str, enemy: &str) -> i32 {
    let mut shape_score = 0;
    if player == "X" {
        shape_score = 1;
    }
    if player == "Y" {
        shape_score = 2;
    }
    if player == "Z" {
        shape_score = 3;
    }

    let mut outcome_score = 0;
    if player == "X" && enemy == "A" {
        outcome_score = 3;
    }
    if player == "X" && enemy == "B" {
        outcome_score = 0;
    }
    if player == "X" && enemy == "C" {
        outcome_score = 6;
    }

    if player == "Y" && enemy == "A" {
        outcome_score = 6;
    }
    if player == "Y" && enemy == "B" {
        outcome_score = 3;
    }
    if player == "Y" && enemy == "C" {
        outcome_score = 0;
    }

    if player == "Z" && enemy == "A" {
        outcome_score = 0;
    }
    if player == "Z" && enemy == "B" {
        outcome_score = 6;
    }
    if player == "Z" && enemy == "C" {
        outcome_score = 3;
    }

    return shape_score + outcome_score;
}

pub fn execute() {
    let data = utils::get_data(2);
    let split_data = data.split("\n");
    let mut total_score = 0;
    for s in split_data {
        if !s.is_empty() {
            let mut gestures = s.split_whitespace();
            let enemy = gestures.next().unwrap();
            let player = gestures.next().unwrap();
            let score = get_score(player, enemy);
            total_score = total_score + score;
        }
    }
    println!("{:?}", total_score);
}

// pub fn execute_two() {

// }
