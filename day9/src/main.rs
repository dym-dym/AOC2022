use std::fs;

type Coord = (i32, i32);

#[derive(Debug)]
struct Rope {
    hd: Coord,
    tl: Coord,
    tl_coords: Vec<Coord>,
}

impl Rope {
    pub fn new() -> Rope {
        Rope {
            hd: (0, 0),
            tl: (0, 0),
            tl_coords: vec![(0, 0)],
        }
    }

    pub fn is_adjacent(&self) -> bool {
        (self.hd.0 - self.tl.0).abs() <= 1 && (self.hd.1 - self.tl.1).abs() <= 1
    }

    pub fn move_head(&mut self, direction: char, value: i32) {
        match direction {
            'R' => self.hd.0 = self.hd.0 + value,
            'L' => self.hd.0 = self.hd.0 - value,
            'U' => self.hd.1 = self.hd.1 + value,
            'D' => self.hd.1 = self.hd.1 - value,
            _ => {}
        }
        self.update_tail();
        // println!("head after move : {:?}", self.hd);
        println!("tail after move : {:?}", self.tl);
    }

    pub fn update_tail(&mut self) {
        if !self.is_adjacent() {
            let (xt, yt) = self.tl;
            let (x, y) = self.hd;
            if x != xt && y != yt {
                if x > xt && y > yt {
                    self.tl.0 = self.tl.0 + 1;
                    self.tl.1 = self.tl.1 + 1;
                }
                if x < xt && y > yt {
                    self.tl.0 = self.tl.0 - 1;
                    self.tl.1 = self.tl.1 + 1;
                }
                if x > xt && y < yt {
                    self.tl.0 = self.tl.0 + 1;
                    self.tl.1 = self.tl.1 - 1;
                }
                if x < xt && y < yt {
                    self.tl.0 = self.tl.0 - 1;
                    self.tl.1 = self.tl.1 - 1;
                }
            } else if x < xt {
                self.tl.0 = self.tl.0 - 1;
            } else if x > xt {
                self.tl.0 = self.tl.0 + 1;
            } else if y < yt {
                self.tl.1 = self.tl.1 - 1;
            } else if y > yt {
                self.tl.1 = self.tl.1 + 1;
            }

            self.tl_coords.push(self.tl);
        }
    }
}

fn part1(input: &str) -> u32 {
    let contents = fs::read_to_string(input).unwrap();
    let contents = contents.split("\n");

    let mut rope = Rope::new();

    let mut seen: Vec<Coord> = vec![];

    let _contents: Vec<(char, u32)> = contents
        .map(|movement| {
            let movement = movement.split(" ").collect::<Vec<&str>>();

            let mut res: (char, u32) = (0.into(), 0);

            if movement[0] != "" {
                res = (
                    movement[0].chars().collect::<Vec<char>>()[0],
                    movement.last().unwrap().parse().unwrap(),
                );
            }

            for _ in 0..res.1 {
                rope.move_head(res.0, 1);
            }
            res
        })
        .collect();

    let _rope_travel = rope
        .tl_coords
        .iter()
        .map(|elem| {
            if !seen.contains(&elem) {
                seen.push(elem.clone());
            }

            elem
        })
        .collect::<Vec<&Coord>>();

    println!("{:?}", seen);

    seen.iter().count().try_into().unwrap()
}

fn part2(input: &str) -> u32 {
    let contents = fs::read_to_string(input).unwrap();
    let contents = contents.split("\n");

    0
}

fn main() {
    println!("Part1: {}", part1("inputday9"));
}
