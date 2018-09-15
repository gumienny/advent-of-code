#![allow(non_upper_case_globals)]
static puzzle: &'static str = include_str!("input");

struct Santa {
    floor: i32,
    position: usize,
}

impl Santa {
    fn new(s: &str) -> Self {
        let mut acc = 0i32;
        let trace: Vec<_> = (1..)
            .zip(s.chars().map(|ch| match ch {
                '(' => {
                    acc += 1;
                    acc
                }
                ')' => {
                    acc -= 1;
                    acc
                }
                _ => acc,
            })).collect();

        Santa {
            floor: trace.iter().last().unwrap().1,
            position: trace.iter().position(|p| p.1 == -1).unwrap_or(0) + 1,
        }
    }
}

fn main() {
    println!("Part 1: {}", Santa::new(puzzle).floor);
    println!("Part 2: {}", Santa::new(puzzle).position);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(0, Santa::new("(())").floor);
        assert_eq!(0, Santa::new("()()").floor);
        assert_eq!(3, Santa::new("(((").floor);
        assert_eq!(3, Santa::new("(()(()(").floor);
        assert_eq!(3, Santa::new("))(((((").floor);
        assert_eq!(-1, Santa::new("())").floor);
        assert_eq!(-1, Santa::new("))(").floor);
        assert_eq!(-3, Santa::new(")))").floor);
        assert_eq!(-3, Santa::new(")())())").floor);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(1, Santa::new(")").position);
        assert_eq!(5, Santa::new("()())").position);
    }
}
