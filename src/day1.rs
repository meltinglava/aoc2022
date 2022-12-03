#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split("\n\n")
        .map(|n| n.lines().map(|n| n.parse::<usize>().unwrap()).sum())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[usize]) -> usize {
    *input.iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &[usize]) -> usize {
    let mut input = input.to_vec();
    input.sort();
    input.reverse();
    input.iter().take(3).sum()
}
