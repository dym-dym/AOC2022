use std::fs::read_to_string;

fn part1(input: &str) -> i32 {
    let contents = read_to_string(input).expect("Couldn't read from file");

    let contents = contents.split("\n");

    let mut max = 0;

    let mut sum: i32 = 0;

    for elem in contents.into_iter() {
        if elem == "" {
            if sum > max {
                max = sum;
            }
            sum = 0;
        } else {
            sum += elem.parse::<i32>().unwrap();
        }
    }

    max
}

fn part2(input: &str) -> i32 {
    let contents = read_to_string(input).expect("Couldn't read from file");

    let contents = contents.split("\n");

    let mut values = vec![];

    let mut sum = 0;
    for elem in contents {
        if elem == "" {
            values.push(sum);
            sum = 0;
        } else {
            sum += elem.parse::<i32>().unwrap();
        }
    }

    sum = 0;

    values.sort();

    println!("{:?}", values);

    for _ in 0..3 {
        sum += values.pop().unwrap();
    }
    sum
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
        assert_eq!(part1("inputtest.txt"), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("inputtest.txt"), 45000);
    }
}
