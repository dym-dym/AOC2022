use std::fs;

#[derive(Debug, Copy, Clone)]
struct Range {
    beginning: u32,
    end: u32,
}

#[derive(Debug, Clone, Copy)]
struct ElfPair {
    first_elf: Range,
    second_elf: Range,
}

impl Range {
    pub fn parse(input: &str) -> Range {
        let mut input = input.split("-");

        let beginning = input.next().unwrap().parse().unwrap();
        let end = input.last().unwrap().parse().unwrap();

        Range { beginning, end }
    }

    pub fn fully_contains(&self, pal: &Range) -> bool {
        self.beginning <= pal.beginning && self.end >= pal.end
    }

    pub fn overlaps_with(&self, pal: &Range) -> bool {
        (self.beginning <= pal.end && self.beginning >= pal.beginning)
            || (self.end <= pal.end && self.end >= pal.beginning)
    }
}

impl ElfPair {
    pub fn parse(input: &str) -> ElfPair {
        // Accounting for split generating a last empty value for some reason
        if input == "" {
            return ElfPair {
                first_elf: Range {
                    beginning: 0,
                    end: 0,
                },
                second_elf: Range {
                    beginning: 0,
                    end: 0,
                },
            };
        }
        let mut input = input.split(",");

        let first_elf = Range::parse(input.next().unwrap());
        let second_elf = Range::parse(input.last().unwrap());

        ElfPair {
            first_elf,
            second_elf,
        }
    }
}

fn part1(input: &str) -> u32 {
    let contents = fs::read_to_string(input).unwrap();

    let contents = contents.split("\n");

    let contents: Vec<ElfPair> = contents.map(|line| ElfPair::parse(line)).collect();

    let res: u32 = contents
        .into_iter()
        .filter(|pair| {
            pair.first_elf.fully_contains(&pair.second_elf)
                || pair.second_elf.fully_contains(&pair.first_elf)
        })
        .count()
        .try_into()
        .unwrap();

    // Returning our result with the last empty value excluded
    res - 1
}

fn part2(input: &str) -> u32 {
    let contents = fs::read_to_string(input).unwrap();

    let contents = contents.split("\n");

    let contents: Vec<ElfPair> = contents.map(|line| ElfPair::parse(line)).collect();

    let res: u32 = contents
        .into_iter()
        .filter(|pair| {
            pair.first_elf.overlaps_with(&pair.second_elf)
                || pair.second_elf.overlaps_with(&pair.first_elf)
        })
        .count()
        .try_into()
        .unwrap();

    // Returning our result with the last empty value excluded
    res - 1
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
        assert_eq!(part1("inputtest.txt"), 2)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("inputtest.txt"), 4)
    }
}
