use itertools::Itertools;

fn score(s: impl Iterator<Item = char>) -> u32 {
    let mut result: u32 = 0;
    let mut count: u32 = 0;
    for c in s {
        count += 1;
        match c {
            'x' => count -= 1,
            'A' => result += 0,
            'B' => result += 1,
            'C' => result += 3,
            'D' => result += 5,
            _ => panic!(),
        }
    }
    result + count * count.saturating_sub(1)
}

pub fn solve_1(input: &str) -> u32 {
    input.chars().chunks(1).into_iter().map(score).sum()
}

pub fn solve_2(input: &str) -> u32 {
    input.chars().chunks(2).into_iter().map(score).sum()
}

pub fn solve_3(input: &str) -> u32 {
    input.chars().chunks(3).into_iter().map(score).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1("ABBAC"), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2("AxBCDDCAxD"), 28);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve_3("xBxAAABCDxCC"), 30);
    }
}
