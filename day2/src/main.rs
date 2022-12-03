use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
enum RPSMove {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, Clone, Copy)]
enum RPSResults {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

fn score_of_game(opponent: RPSMove, myself: RPSMove) -> u32 {
    let res = match (opponent as i32 - myself as i32).rem_euclid(3) {
        0 => RPSResults::Draw,
        1 => RPSResults::Loss,
        2 => RPSResults::Win,
        x => panic!("Resultat impossible {x}",),
    };

    res as u32 + myself as u32
}

fn get_move(input: char) -> RPSMove {
    match input {
        'A' => RPSMove::Rock,
        'B' => RPSMove::Paper,
        'C' => RPSMove::Scissors,
        'X' => RPSMove::Rock,
        'Y' => RPSMove::Paper,
        'Z' => RPSMove::Scissors,
        x => panic!("Mouvement impossible {x}"),
    }
}

fn part1(input: &str) -> u32 {
    let contents = fs::read_to_string(input).unwrap();

    contents
        .lines()
        .map(|line| {
            score_of_game(
                get_move(line.chars().next().unwrap()),
                get_move(line.chars().last().unwrap()),
            )
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    0
}

fn main() {
    println!("{}", part1("input.txt"));
    println!("{}", part2("input.txt"));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1("inputtest.txt"), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("inputtest.txt"), 0);
    }
}
