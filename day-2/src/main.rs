// Advent of Code 2022 -- Day 2
// Written by Carson Woods

enum Choice {
    Rock,
    Paper,
    Scissors,
    Error,
}

enum GameState {
    Win,
    Lose,
    Draw,
    Error
}

fn main() {
    // read input from file contents
    let contents = std::fs::read_to_string("input.txt").expect("Error: could not read input file");

    // split the contents of the file by blank lines (consecutive newline characters)
    let rounds: Vec<&str> = contents.split('\n').collect();

    part_a(&rounds);
    part_b(&rounds);
}

fn part_a(rounds: &Vec<&str>) {
    let mut score: u64 = 0;

    for round in rounds.into_iter() {
        let split_round: Vec<&str> = round.split(' ').collect();

        if split_round.len() < 2 {
            continue;
        }

        let input = split_round[0];
        let output = split_round[1];

        let input_choice = match input {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            _ => Choice::Error,
        };

        let output_choice = match output {
            "X" => Choice::Rock,
            "Y" => Choice::Paper,
            "Z" => Choice::Scissors,
            _ => Choice::Error,
        };

        let win_status = determine_winner(&input_choice, &output_choice);

        score += match win_status {
            GameState::Win => 6,
            GameState::Draw => 3,
            GameState::Lose => 0,
            GameState::Error => 0
        };

        score = score
            + match output_choice {
                Choice::Rock => 1,
                Choice::Paper => 2,
                Choice::Scissors => 3,
                Choice::Error => score,
            };
    }
    println!("The solution to part A: {}", score);
}

fn part_b(rounds: &Vec<&str>) {
    let mut score: u64 = 0;

    for round in rounds.into_iter() {
        let split_round: Vec<&str> = round.split(' ').collect();

        if split_round.len() < 2 {
            continue;
        }

        let input = split_round[0];
        let output = split_round[1];

        let input_choice = match input {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            _ => Choice::Error,
        };

        let win_status = match output {
            "X" => GameState::Lose,
            "Y" => GameState::Draw,
            "Z" => GameState::Win,
            _ => GameState::Error,
        };

        score += match win_status {
            GameState::Win => 6,
            GameState::Draw => 3,
            GameState::Lose => 0,
            GameState::Error => 0
        };

        let output_choice: Choice = determine_choice(&input_choice, win_status);

        score = score
            + match output_choice {
                Choice::Rock => 1,
                Choice::Paper => 2,
                Choice::Scissors => 3,
                Choice::Error => score,
            };
    }
    println!("The solution to part B: {}", score);
}

// input choice is the opponents choice
// output choice is "my" choice
fn determine_winner(input_choice: &Choice, output_choice: &Choice) -> GameState {
    match input_choice {
        Choice::Rock => {
            match output_choice {
                Choice::Rock => GameState::Draw,
                Choice::Paper => GameState::Win,
                Choice::Scissors => GameState::Lose,
                Choice::Error => GameState::Lose,
            }
        },
        Choice::Paper => {
            match output_choice {
                Choice::Rock => GameState::Lose,
                Choice::Paper => GameState::Draw,
                Choice::Scissors => GameState::Win,
                Choice::Error => GameState::Lose,
            }
        },
        Choice::Scissors => {
            match output_choice {
                Choice::Rock => GameState::Win,
                Choice::Paper => GameState::Lose,
                Choice::Scissors => GameState::Draw,
                Choice::Error => GameState::Lose,
            }
        },
        Choice::Error => GameState::Lose
    }
}

fn determine_choice(input_choice: &Choice, game_state: GameState) -> Choice {
    match input_choice {
        Choice::Rock => {
            match game_state {
                GameState::Win => Choice::Paper,
                GameState::Draw => Choice::Rock,
                GameState::Lose => Choice::Scissors,
                GameState::Error => Choice::Error,
            }
        },
        Choice::Paper => {
            match game_state {
                GameState::Win => Choice::Scissors,
                GameState::Draw => Choice::Paper,
                GameState::Lose => Choice::Rock,
                GameState::Error => Choice::Error,
            }
        },
        Choice::Scissors => {
            match game_state {
                GameState::Win => Choice::Rock,
                GameState::Draw => Choice::Scissors,
                GameState::Lose => Choice::Paper,
                GameState::Error => Choice::Error,
            }
        },
        Choice::Error => Choice::Error
    }
}
