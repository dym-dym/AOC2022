use std::fs;

type Compartment = Vec<char>;

#[derive(Debug)]
struct Rucksack {
    first_compartment: Compartment,
    second_compartment: Compartment,
}

impl Rucksack {
    pub fn new(contents: &str) -> Option<Rucksack> {
        if contents == "" {
            return None;
        }

        let first_compartment = contents[..contents.len() / 2].chars().collect();
        let second_compartment = contents[contents.len() / 2..].chars().collect();

        Some(Rucksack {
            first_compartment,
            second_compartment,
        })
    }

    pub fn get_common_item(&self) -> char {
        let res: Vec<&char> = self
            .first_compartment
            .iter()
            .filter(|item| self.second_compartment.contains(item))
            .collect();

        *res[0]
    }
}

fn get_item_priority(item: char) -> u32 {
    if item.is_uppercase() {
        return item as u32 - 64 + 26;
    } else if item.is_lowercase() {
        return item as u32 - 96;
    } else {
        panic!("Unable to parse item {item}");
    }
}

fn part1(input: &str) -> u32 {
    let contents = fs::read_to_string(input).unwrap();

    let contents = contents.split("\n");

    contents
        .map(|line| {
            if let Some(sack) = Rucksack::new(line) {
                get_item_priority(sack.get_common_item())
            } else {
                0
            }
        })
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
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
        assert_eq!(part1("inputtest.txt"), 157)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("inputtest.txt"), 0)
    }
}
