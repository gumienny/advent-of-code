static puzzle: &'static str = include_str!("input");

fn santa(s: &str) -> i32 {
    if s.len() == 0 {
        return 0;
    }

    s.chars()
        .map(|ch| match ch {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }).sum()
}

fn main() {
    println!("{}", santa(puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(0, santa(""));
        assert_eq!(0, santa("(())"));
        assert_eq!(0, santa("()()"));
        assert_eq!(3, santa("((("));
        assert_eq!(3, santa("(()(()("));
        assert_eq!(3, santa("))((((("));
        assert_eq!(-1, santa("())"));
        assert_eq!(-1, santa("))("));
        assert_eq!(-3, santa(")))"));
        assert_eq!(-3, santa(")())())"));
    }
}
