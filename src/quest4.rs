use std::iter::{from_fn, zip};

use itertools::{chain, Itertools};
use nom::{
    character::complete::{digit1, multispace0},
    combinator::map_res,
    multi::many1,
    sequence::preceded,
    IResult,
};

fn parse(input: &str) -> Vec<u64> {
    fn num(input: &str) -> IResult<&str, u64> {
        map_res(digit1, str::parse)(input)
    }
    many1(preceded(multispace0, num))(input).unwrap().1
}

pub fn solve_1(input: &str) -> u64 {
    let nums = parse(input);
    let &min = nums.iter().min().unwrap();
    nums.into_iter().map(|n| n - min).sum()
}

pub fn solve_2(input: &str) -> u64 {
    let nums = parse(input);
    let &min = nums.iter().min().unwrap();
    nums.into_iter().map(|n| n - min).sum()
}

fn sum_bands<'a>(nums: impl IntoIterator<Item = &'a u64> + 'a) -> impl Iterator<Item = u64> + 'a {
    let mut total = Some(0);
    let mut nums = nums.into_iter().enumerate().tuple_windows();
    from_fn(move || {
        let Some(((_, a), (i, b))) = nums.next() else {
            return total.take();
        };
        let last_total = total.unwrap();
        total = Some(last_total + i as u64 * a.abs_diff(*b));
        Some(last_total)
    })
}

pub fn solve_3(input: &str) -> u64 {
    let mut nums = parse(input);
    nums.sort();
    let pulls: Vec<_> = sum_bands(&nums).collect();
    let mut hits: Vec<_> = sum_bands(nums.iter().rev()).collect();
    hits.reverse();
    zip(pulls, hits).map(|(a, b)| a + b).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            solve_1(
                "3
                4
                7
                8"
            ),
            10
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            solve_3(
                "2
                4
                5
                6
                8"
            ),
            8
        );
    }
}
