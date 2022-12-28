use regex::Regex;
use std::fs;

#[derive(Debug)]
enum FileType<'a> {
    Dir(&'a str),
    File((&'a str, u32)),
}

#[derive(Debug)]
struct FileSystem<'a> {
    dir: FileType<'a>,
    child: Option<Box<FileSystem<'a>>>,
}

impl<'a> FileSystem<'a> {
    pub fn new(dir: &'a str) -> FileSystem<'a> {
        FileSystem {
            dir: FileType::Dir(dir),
            child: None,
        }
    }
}

fn part2(input: &str) -> u32 {
    let contents = fs::read_to_string(input).unwrap();
    0
}

fn part1(input: &str) -> u32 {
    let contents = fs::read_to_string(input).unwrap();
    let contents = contents.split("\n");

    let re_dir = Regex::new(r"^dir");
    let cmd_cd = Regex::new(r"^\$ cd");

    for line in contents {
        println!("{line}");
    }

    0
}

fn main() {}
