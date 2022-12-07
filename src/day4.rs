use std::{mem::swap, ops::RangeInclusive};

pub type AocType = (RangeInclusive<usize>, RangeInclusive<usize>);

fn rng_len(rng: &RangeInclusive<usize>) -> usize {
    rng.end() - rng.start()
}

fn rng_to_u128(rng: &RangeInclusive<usize>) -> u128 {
    let mut active = 0u128;
    for n in rng.clone() {
        active |= 1 << n;
    }
    active
}

fn s_to_range(input: &str) -> RangeInclusive<usize> {
    let (v0, v1) = input.split_once('-').unwrap();
    v0.parse().unwrap()..=v1.parse().unwrap()
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<AocType> {
    input
        .lines()
        .map(|n| n.split_once(',').unwrap())
        .map(|(a, b)| (s_to_range(a), s_to_range(b)))
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[AocType]) -> usize {
    input
        .iter()
        .filter(|(a, b)| {
            let a = rng_to_u128(&a);
            let b = rng_to_u128(&b);
            let c = a & b;
            c == a || c == b
        })
        .count()
}

#[aoc(day4, part1, part1b)]
pub fn part1b(input: &[AocType]) -> usize {
    input
        .iter()
        .cloned()
        .filter(|(a, b)| {
            let (mut a, mut b) = (a, b);
            if rng_len(&a) < rng_len(&b) {
                swap(&mut a, &mut b);
            }
            a.start() <= b.start() && b.end() <= a.end()
        })
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &[AocType]) -> usize {
    input
        .iter()
        .filter(|(a, b)| rng_to_u128(&a) & rng_to_u128(&b) != 0)
        .count()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const TESTINPUT: &'static str = indoc! {"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "};

    #[test]
    fn test_d4_p1() {
        let input = input_generator(TESTINPUT);
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_d4_p2() {
        let input = input_generator(TESTINPUT);
        assert_eq!(part2(&input), 4);
    }
}
