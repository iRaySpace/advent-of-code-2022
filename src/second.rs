use crate::utils;

fn get_score(player: &str, enemy: &str) -> i32 {
    // A -> Rock; B -> Paper; C -> Scissors
    let mut shape_score = 0;
    if player == "A" {
        shape_score = 1;
    }
    if player == "B" {
        shape_score = 2;
    }
    if player == "C" {
        shape_score = 3;
    }

    let mut outcome_score = 0;
    if player == "A" && enemy == "A" {
        outcome_score = 3;
    }
    if player == "A" && enemy == "B" {
        outcome_score = 0;
    }
    if player == "A" && enemy == "C" {
        outcome_score = 6;
    }

    if player == "B" && enemy == "A" {
        outcome_score = 6;
    }
    if player == "B" && enemy == "B" {
        outcome_score = 3;
    }
    if player == "B" && enemy == "C" {
        outcome_score = 0;
    }

    if player == "C" && enemy == "A" {
        outcome_score = 0;
    }
    if player == "C" && enemy == "B" {
        outcome_score = 6;
    }
    if player == "C" && enemy == "C" {
        outcome_score = 3;
    }

    return shape_score + outcome_score;
}

fn get_handpick<'a>(player: &'a str, enemy: &'a str) -> &'a str {
    // X lose; Y draw; Z win
    if enemy == "A" && player == "X" {
        return "C"
    }
    if enemy == "A" && player == "Y" {
        return "A";
    }
    if enemy == "A" && player == "Z" {
        return "B";
    }

    if enemy == "B" && player == "X" {
        return "A"
    }
    if enemy == "B" && player == "Y" {
        return "B";
    }
    if enemy == "B" && player == "Z" {
        return "C";
    }

    if enemy == "C" && player == "X" {
        return "B"
    }
    if enemy == "C" && player == "Y" {
        return "C";
    }
    if enemy == "C" && player == "Z" {
        return "A";
    }

    return "";
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
            let encoded_player = match player {
                "X" => "A",
                "Y" => "B",
                "Z" => "C",
                &_ => "",
            };
            let score = get_score(encoded_player, enemy);
            total_score = total_score + score;
        }
    }
    println!("{:?}", total_score);
}

pub fn execute_two() {
    let data = utils::get_data(2);
    let split_data = data.split("\n");
    let mut total_score = 0;
    for s in split_data {
        if !s.is_empty() {
            let mut gestures = s.split_whitespace();
            let enemy = gestures.next().unwrap();
            let player = gestures.next().unwrap();
            let handpick = get_handpick(player, enemy);
            let score = get_score(handpick, enemy);
            total_score = total_score + score;
        }
    }
    println!("{:?}", total_score);
}
