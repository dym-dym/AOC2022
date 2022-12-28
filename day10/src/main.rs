use std::fs;

fn part1(input: &str) -> i32 {
    let signal_check = vec![20, 60, 100, 140, 180, 220];
    let mut cycle: i32 = 1;
    let mut x_register: i32 = 1;

    fs::read_to_string(input)
        .unwrap()
        .split_whitespace()
        .map(|input| match input {
            "noop" => 0,
            "addx" => 0,
            x => x.parse().unwrap(),
        })
        .map(|value| {
            let mut res = 0;
            x_register = x_register + value;
            cycle = cycle + 1;
            if signal_check.contains(&cycle) {
                res = cycle * x_register;
            }
            res
        })
        .sum()
}

fn part2(input: &str) {
    // let signal_check = vec![20, 60, 100, 140, 180, 220];
    let mut cycle: i32 = 1;
    let mut x_register: i32 = 1;

    let drawing: Vec<char> = fs::read_to_string(input)
        .unwrap()
        .split_whitespace()
        .map(|input| match input {
            "noop" => 0,
            "addx" => 0,
            x => x.parse().unwrap(),
        })
        .map(|value| {
            let res =
                if vec![x_register - 1, x_register, x_register + 1].contains(&((cycle % 40) - 1)) {
                    '#'
                } else {
                    '.'
                };
            println!("{x_register}");
            x_register = x_register + value;
            cycle = cycle + 1;
            res
        })
        .collect();

    let mut x = 0;
    for character in drawing {
        if x % 40 == 0 {
            print!("\n");
        }
        print!("{character}");
        x = x + 1;
    }
}

fn main() {
    // println!("{}", part1("input"));
    part2("input");
}
