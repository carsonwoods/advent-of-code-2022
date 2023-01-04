// Advent of Code 2022 -- Day 2
// Written by Carson Woods

// represents possible Rock, Paper, Scissors Choices
// Error available in case of input file issue
enum Choice {
    Rock,
    Paper,
    Scissors,
    Error,
}

// represents possible final game states
// Error available in case of input file issue
enum GameState {
    Win,
    Lose,
    Draw,
    Error,
}

fn main() {
    // read input from file contents
    let contents = std::fs::read_to_string("input.txt").expect("Error: could not read input file");

    // split the contents of the file by blank lines (consecutive newline characters)
    let rounds: Vec<&str> = contents.split('\n').collect();

    part_a(&rounds);
    part_b(&rounds);
}

// solves the solution to part A
fn part_a(rounds: &[&str]) {
    let mut score: u32 = 0;

    // iterates over rounds of the games
    for round in rounds.iter() {
        // splits round into opponents move and my choice
        let split_round: Vec<&str> = round.split(' ').collect();

        // if the round is incomplete/invalid skip the round
        if split_round.len() < 2 {
            continue;
        }

        // matches an the opponent choice to a Choice value
        let opponents_choice = match split_round[0] {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            _ => Choice::Error,
        };

        // maps my required choice to a Choice value
        let my_choice = match split_round[1] {
            "X" => Choice::Rock,
            "Y" => Choice::Paper,
            "Z" => Choice::Scissors,
            _ => Choice::Error,
        };

        // determines win status based on
        let win_status = determine_winner(&opponents_choice, &my_choice);

        // adds value to score based on final state of game
        // at the end of each round
        score += match win_status {
            GameState::Win => 6,
            GameState::Draw => 3,
            GameState::Lose => 0,
            GameState::Error => 0,
        };

        // adds value to score based on my choice
        score = score
            + match my_choice {
                Choice::Rock => 1,
                Choice::Paper => 2,
                Choice::Scissors => 3,
                Choice::Error => score,
            };
    }
    println!("The solution to part A: {}", score);
}

fn part_b(rounds: &[&str]) {
    // value to track total score
    let mut score: u64 = 0;

    // iterates over game rounds
    for round in rounds.iter() {
        // splits rounds up into their choice and if I need to win/lose/draw
        let split_round: Vec<&str> = round.split(' ').collect();

        // if round format is invalid, skip round
        if split_round.len() < 2 {
            continue;
        }

        // match opponents choice to a Choice value
        let opponents_choice = match split_round[0] {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            _ => Choice::Error,
        };

        // matches needed game state from file to GameState value
        let win_status = match split_round[1] {
            "X" => GameState::Lose,
            "Y" => GameState::Draw,
            "Z" => GameState::Win,
            _ => GameState::Error,
        };

        // add points according based on needed GameState
        score += match win_status {
            GameState::Win => 6,
            GameState::Draw => 3,
            GameState::Lose => 0,
            GameState::Error => 0,
        };

        // determine my choice based on strategy guide (input file)
        let my_choice: Choice = determine_choice(&opponents_choice, win_status);

        // add to score based on my Choice value from determine_choice()
        score = score
            + match my_choice {
                Choice::Rock => 1,
                Choice::Paper => 2,
                Choice::Scissors => 3,
                Choice::Error => score,
            };
    }
    println!("The solution to part B: {}", score);
}

// determines winner based on opponent input and my input
fn determine_winner(opponents_choice: &Choice, my_choice: &Choice) -> GameState {
    // matches the opponents choice, my choice to determine game state
    // An error for opponents_choice bubbles up a loss to the calling function
    match opponents_choice {
        Choice::Rock => match my_choice {
            Choice::Rock => GameState::Draw,
            Choice::Paper => GameState::Win,
            Choice::Scissors => GameState::Lose,
            Choice::Error => GameState::Lose,
        },
        Choice::Paper => match my_choice {
            Choice::Rock => GameState::Lose,
            Choice::Paper => GameState::Draw,
            Choice::Scissors => GameState::Win,
            Choice::Error => GameState::Lose,
        },
        Choice::Scissors => match my_choice {
            Choice::Rock => GameState::Win,
            Choice::Paper => GameState::Lose,
            Choice::Scissors => GameState::Draw,
            Choice::Error => GameState::Lose,
        },
        Choice::Error => GameState::Lose,
    }
}

// determines what I need to choose based on an opponents choice and the desired
// game state at the end of a round
fn determine_choice(opponents_choice: &Choice, game_state: GameState) -> Choice {
    // matches the opponents choice, then chooses my choice based on the game_state param
    // An error for opponents_choice bubbles up error to calling function
    match opponents_choice {
        Choice::Rock => match game_state {
            GameState::Win => Choice::Paper,
            GameState::Draw => Choice::Rock,
            GameState::Lose => Choice::Scissors,
            GameState::Error => Choice::Error,
        },
        Choice::Paper => match game_state {
            GameState::Win => Choice::Scissors,
            GameState::Draw => Choice::Paper,
            GameState::Lose => Choice::Rock,
            GameState::Error => Choice::Error,
        },
        Choice::Scissors => match game_state {
            GameState::Win => Choice::Rock,
            GameState::Draw => Choice::Scissors,
            GameState::Lose => Choice::Paper,
            GameState::Error => Choice::Error,
        },
        Choice::Error => Choice::Error,
    }
}
