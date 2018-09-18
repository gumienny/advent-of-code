static puzzle: &'static str = include_str!("input");

#[derive(Debug, PartialEq, Copy, Clone)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct A {
    a: Action,
    f: (usize, usize),
    t: (usize, usize),
}

#[derive(Debug)]
struct Grid {
    field: Vec<Vec<i32>>,
}

impl Grid {
    fn new() -> Self {
        Self {
            field: vec![vec![0; 1000]; 1000],
        }
    }

    fn count_lights(&self) -> usize {
        self.field
            .iter()
            .fold(0, |acc, v| acc + v.iter().filter(|f| **f == 1).count())
    }

    fn get_total_brightness(&self) -> i32 {
        self.field
            .iter()
            .fold(0, |acc, v| acc + v.iter().sum::<i32>())
    }

    fn action(&mut self, a: A) {
        for x in a.f.0..a.t.0 + 1 {
            for y in a.f.1..a.t.1 + 1 {
                self.field[y][x] = match a.a {
                    Action::TurnOn => 1,
                    Action::TurnOff => 0,
                    Action::Toggle => if self.field[y][x] == 0 {
                        1
                    } else {
                        0
                    },
                }
            }
        }
    }

    fn action_2(&mut self, a: A) {
        for x in a.f.0..a.t.0 + 1 {
            for y in a.f.1..a.t.1 + 1 {
                self.field[y][x] += match a.a {
                    Action::TurnOn => 1,
                    Action::TurnOff => if self.field[y][x] > 0 {
                        -1
                    } else {
                        0
                    },
                    Action::Toggle => 2,
                }
            }
        }
    }
}

fn parse(input: &str) -> Vec<A> {
    input.lines().map(parse_line).collect::<Vec<A>>()
}

fn parse_line(input: &str) -> A {
    let mut iter = input.chars().skip(6);

    let a = match iter.next().unwrap() {
        'f' => Action::TurnOff,
        'n' => Action::TurnOn,
        ' ' => Action::Toggle,
        _ => unreachable!(),
    };

    let nums = iter
        .map(|ch| if ch.is_digit(10) { ch } else { ' ' })
        .collect::<String>();

    let nums: Vec<usize> = nums
        .trim()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    A {
        a,
        f: (nums[0], nums[1]),
        t: (nums[2], nums[3]),
    }
}

fn main() {
    let instructions = parse(puzzle);

    let mut grid = Grid::new();

    for a in instructions.iter() {
        grid.action(*a);
    }

    println!("Part 1: {}", grid.count_lights());

    let mut grid = Grid::new();

    for a in instructions.iter() {
        grid.action_2(*a);
    }

    println!("Part 2: {}", grid.get_total_brightness());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_parse() {
        let input = "turn on 489,959 through 759,964\n\
                     turn off 820,516 through 871,914\n\
                     toggle 427,423 through 929,502";

        let parsed = parse(input);

        assert_eq!(
            parsed,
            vec![
                A {
                    a: Action::TurnOn,
                    f: (489, 959),
                    t: (759, 964),
                },
                A {
                    a: Action::TurnOff,
                    f: (820, 516),
                    t: (871, 914),
                },
                A {
                    a: Action::Toggle,
                    f: (427, 423),
                    t: (929, 502),
                },
            ]
        );
    }

    #[test]
    fn test_part_1_grid() {
        let mut grid = Grid::new();

        assert_eq!(grid.count_lights(), 0);

        grid.action(A {
            a: Action::TurnOn,
            f: (0, 0),
            t: (999, 999),
        });

        assert_eq!(grid.count_lights(), 1000 * 1000);

        grid.action(A {
            a: Action::TurnOff,
            f: (0, 0),
            t: (499, 499),
        });

        assert_eq!(grid.count_lights(), 1000 * 1000 - 500 * 500);

        grid.action(A {
            a: Action::Toggle,
            f: (0, 0),
            t: (999, 999),
        });

        assert_eq!(grid.count_lights(), 500 * 500);
        assert_eq!(grid.get_total_brightness(), 500 * 500);
    }

    #[test]
    fn test_part_2() {
        let mut grid = Grid::new();

        for i in 0..99 + 1 {
            grid.action_2(A {
                a: Action::TurnOn,
                f: (0, 0),
                t: (i, i),
            })
        }

        assert_eq!(
            grid.get_total_brightness(),
            (1..101).map(|n| n * n).sum::<i32>()
        );

        grid.action_2(A {
            a: Action::Toggle,
            f: (0, 0),
            t: (999, 999),
        });

        grid.action_2(A {
            a: Action::TurnOff,
            f: (0, 0),
            t: (999, 999),
        });

        assert_eq!(
            grid.get_total_brightness(),
            1_000_000 + (1..101).map(|n| n * n).sum::<i32>()
        );
    }
}
