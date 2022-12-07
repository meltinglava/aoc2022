use std::collections::HashSet;

pub type AocType = char;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<AocType> {
    input.chars().collect()
}

fn find_n(input: &[AocType], n: usize) -> usize {
    input
        .windows(n)
        .enumerate()
        .find(|(_, c)| c.iter().copied().collect::<HashSet<_>>().len() == n)
        .unwrap()
        .0
        + n
}

#[aoc(day6, part1)]
pub fn part1(input: &[AocType]) -> usize {
    find_n(input, 4)
}

#[aoc(day6, part2)]
pub fn part2(input: &[AocType]) -> usize {
    find_n(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: [(&'static str, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    #[test]
    fn test_data() {
        for (s, v) in DATA {
            let list = input_generator(s);
            assert_eq!(part1(&list), v);
        }
    }
}
