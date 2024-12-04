use std::collections::HashSet;

use nom::{
    bytes::complete::{tag, take},
    character::complete::{alpha1, multispace0, satisfy, space0},
    combinator::{map, recognize},
    error::Error,
    multi::{many1, many_till, separated_list1},
    sequence::{preceded, separated_pair},
    FindSubstring,
};

fn parse(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let words = preceded(
        tag::<&str, &str, Error<&str>>("WORDS:"),
        separated_list1(
            tag(","),
            preceded(space0, map(alpha1, |s: &str| s.as_bytes().to_owned())),
        ),
    );
    let word = recognize(many1(satisfy(|c| c.is_alphabetic() || c == '_')));
    let text = many1(map(
        many_till(take(1usize), map(word, |s: &str| s.as_bytes().to_owned())),
        |(_, g)| g,
    ));
    separated_pair(words, multispace0, text)(input).unwrap().1
}

pub fn solve_1(input: &str) -> usize {
    let (words, text) = parse(input);
    let mut count = 0;
    for t in text {
        for w in &words {
            if t.as_slice().find_substring(w.as_slice()).is_some() {
                count += 1;
            }
        }
    }
    count
}

fn covered(words: &[Vec<u8>], text: &[Vec<u8>]) -> HashSet<(usize, usize)> {
    let words: HashSet<Vec<u8>> = words
        .iter()
        .flat_map(|w| [w.to_owned(), w.iter().copied().rev().collect()])
        .collect();
    let mut covered = HashSet::new();
    for w in words {
        for (i, t) in text.iter().enumerate() {
            let mut j = 0;
            while j + w.len() <= t.len() {
                if t[j..].starts_with(&w) {
                    covered.extend((j..j + w.len()).map(|j| (i, j)));
                }
                j += 1;
            }
        }
    }
    covered
}

pub fn solve_2(input: &str) -> usize {
    let (words, text) = parse(input);
    covered(&words, &text).len()
}

fn transposed(m: &[Vec<u8>]) -> Vec<Vec<u8>> {
    (0..m[0].len())
        .map(|x| (0..m.len()).map(|y| m[y][x]).collect())
        .collect()
}

pub fn solve_3(input: &str) -> usize {
    let (words, text) = parse(input);
    let word_length = words.iter().map(|w| w.len()).max().unwrap();

    // Do left/right
    let mut wrapped_text = Vec::new();
    for t in &text {
        let mut t = t.clone();
        t.extend_from_within(..word_length - 1);
        wrapped_text.push(t);
    }
    let horizontal = covered(&words, &wrapped_text)
        .into_iter()
        .map(|(i, j)| (i, j % text[0].len()));

    // Now do up/down
    let transposed_text = transposed(&text);
    let vertical = covered(&words, &transposed_text)
        .into_iter()
        .map(|(i, j)| (j, i));

    // Combine them and return the result
    HashSet::<(usize, usize)>::from_iter(horizontal.chain(vertical)).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            solve_1(
                "WORDS:THE,OWE,MES,ROD,HER
                AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE"
            ),
            4
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            solve_2(
                "WORDS:THE,OWE,MES,ROD,HER,QAQ
                AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
                THE FLAME SHIELDED THE HEART OF THE KINGS
                POWE PO WER P OWE R
                THERE IS THE END
                QAQAQ"
            ),
            42
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            solve_3(
                "WORDS:THE,OWE,MES,ROD,RODEO
                HELWORLT
                ENIGWDXL
                TRODEOAL"
            ),
            10
        );
    }
}
