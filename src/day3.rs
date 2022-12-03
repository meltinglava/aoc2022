use std::collections::HashSet;

use itertools::Itertools;

pub type AocType = (String, String);

fn to_score(c: char) -> usize {
    match c {
        c if c.is_ascii_lowercase() => (c as u8) - b'a' + 1,
        c if c.is_ascii_uppercase() => (c as u8) - b'A' + 27,
        n => panic!("unknown case: {}", n),
    }
    .into()
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| {
            (
                a.chars().collect::<HashSet<_>>(),
                b.chars().collect::<HashSet<_>>(),
            )
        })
        .map(|(a, b)| *a.intersection(&b).next().unwrap())
        .map(to_score)
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|c| {
            c.map(|line| line.chars().collect::<HashSet<_>>())
                .reduce(|accum, next| accum.intersection(&next).copied().collect())
                .unwrap()
                .into_iter()
                .next()
                .unwrap()
        })
        .map(to_score)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_EXAMPLE: &'static str = indoc!(
        "
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "
    );

    #[test]
    fn test_p1() {
        assert_eq!(part1(TEST_EXAMPLE), 157);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part2(TEST_EXAMPLE), 70);
    }
}
