#![allow(non_upper_case_globals)]
static puzzle: &'static str = include_str!("input");

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct House {
    x: i64,
    y: i64,
}

impl House {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn delta(&mut self, delta: char) -> () {
        match delta {
            '^' => {
                self.y += 1;
            }
            'v' => {
                self.y -= 1;
            }
            '<' => {
                self.x -= 1;
            }
            '>' => {
                self.x += 1;
            }
            _ => (),
        }
    }
}

fn trace(s: &str) -> (usize, usize) {
    // part 1
    let mut single: HashSet<House> = HashSet::new();

    let mut santa_single = House::new();

    single.insert(pointer.clone());

    // part 2
    let mut double: HashSet<House> = HashSet::new();

    let mut santa = House::new();
    let mut robot = House::new();

    double.insert(santa.clone());
    double.insert(robot.clone());

    for (i, direction) in s.chars().enumerate() {
        santa_single.delta(direction);

        single.insert(santa_single.clone());

        if i % 2 == 0 {
            santa.delta(direction);
            double.insert(santa.clone());
        } else {
            robot.delta(direction);
            double.insert(robot.clone());
        }
    }

    (single.len(), double.len())
}

fn main() {
    println!("Part 1: {}", trace(puzzle).0);
    println!("Part 2: {}", trace(puzzle).1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(2, trace(">").0);
        assert_eq!(4, trace("^>v<").0);
        assert_eq!(2, trace("^v^v^v^v^v").0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(3, trace("^v").1);
        assert_eq!(3, trace("^>v<").1);
        assert_eq!(11, trace("^v^v^v^v^v").1);
    }
}
