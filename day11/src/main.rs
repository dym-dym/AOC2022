struct Monkey<'a> {
    starting_items: Vec<u32>,
    operation: Box<dyn Fn(u32) -> u32 + 'a>,
    test: u32,
    if_true: u32,
    if_false: u32,
}

impl<'a> Monkey<'a> {
    pub fn new(
        starting_items: Vec<u32>,
        operation: Box<dyn Fn(u32) -> u32 + 'a>,
        test: u32,
        if_true: u32,
        if_false: u32,
    ) -> Self {
        Monkey {
            starting_items,
            operation,
            test,
            if_true,
            if_false,
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::Monkey;

    #[test]
    fn part1() {
        let mut monkey0 = Monkey::new(vec![78, 98], Box::new(|old| old * 19), 23, 2, 3);
        let mut monkey1 = Monkey::new(vec![54, 65, 75, 74], Box::new(|old| old + 6), 19, 2, 0);
        let mut monkey2 = Monkey::new(vec![79, 60, 97], Box::new(|old| old * old), 13, 1, 3);
        let mut monkey3 = Monkey::new(vec![74], Box::new(|old| old + 3), 17, 0, 1);

        let monkeys = vec![monkey0, monkey1, monkey2, monkey3];

        unimplemented!();
    }
}
