#![allow(non_upper_case_globals)]
static puzzle: &'static str = include_str!("input");

use std::str::FromStr;

fn paper(s: &str) -> (u64, u64) {
    let parsed: Vec<(u64, u64, u64)> = s
        .lines()
        .map(|line| {
            let mut dims: Vec<u64> = line
                .splitn(3, 'x')
                .map(|d| u64::from_str(d).unwrap())
                .collect();

            dims.sort();

            return (dims[0], dims[1], dims[2]);
        }).collect();

    let part_1: u64 = parsed
        .iter()
        .map(|(a, b, c)| {
            let min = a * b;

            return 2 * a * b + 2 * a * c + 2 * b * c + min;
        }).sum();

    let part_2: u64 = parsed
        .iter()
        .map(|(a, b, c)| {
            return 2 * a + 2 * b + a * b * c;
        }).sum();

    (part_1, part_2)
}

fn main() {
    let (part_1, part_2) = paper(puzzle);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(58, paper("2x3x4").0);
        assert_eq!(43, paper("1x1x10").0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(34, paper("2x3x4").1);
        assert_eq!(14, paper("1x1x10").1);
    }
}
