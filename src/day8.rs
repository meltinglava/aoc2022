use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

pub struct Grid<const N: usize> {
    data: [[usize; N]; N],
}

impl<const N: usize> Grid<N> {
    fn parse_grid(input: &str) -> Self {
        let mut data = Self { data: [[0; N]; N] };
        for (y, line) in input.lines().enumerate() {
            for (x, val) in line.chars().enumerate() {
                data[(x, y)] = val.to_digit(10).unwrap() as usize
            }
        }
        data
    }
}

impl<const N: usize> Index<(usize, usize)> for Grid<N> {
    type Output = usize;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.data.index(index.0).index(index.1)
    }
}

impl<const N: usize> IndexMut<(usize, usize)> for Grid<N> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.data.index_mut(index.0).index_mut(index.1)
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Grid<99> {
    Grid::parse_grid(input)
}

fn check_sides<const N: usize, F, S>(
    grid: &Grid<N>,
    values: &mut HashSet<(usize, usize)>,
    side: S,
    get: F,
) where
    F: Fn((usize, usize)) -> (usize, usize),
    S: Fn(usize) -> (usize, usize),
{
    for a in 0..N {
        let mut last_value = grid[side(a)];
        values.insert(side(a));
        for b in 1..N {
            let coord = get((a, b));
            let tree = grid[coord];
            if last_value < tree {
                last_value = tree;
                values.insert(coord);
                if tree == 9 {
                    break;
                }
            }
        }
    }
}

fn check_direction<const N: usize, F>(grid: &Grid<N>, start_height: usize, get: F) -> usize
where
    F: Fn(usize) -> (usize, usize),
{
    let mut count = 0;
    let range = 1..N - 1;
    for offset in 1.. {
        let tree = get(offset);
        let tree_hight = grid[tree];
        count += 1;
        if tree_hight >= start_height {
            break;
        }
        if !range.contains(&tree.0) || !range.contains(&tree.1) {
            break;
        }
    }
    count
}

fn check_possition<const N: usize>(grid: &Grid<N>, coord: (usize, usize)) -> usize {
    let (x, y) = coord;
    check_direction(grid, grid[coord], |a| (x + a, y))
        * check_direction(grid, grid[coord], |a| (x - a, y))
        * check_direction(grid, grid[coord], |a| (x, y + a))
        * check_direction(grid, grid[coord], |a| (x, y - a))
}

#[aoc(day8, part1)]
pub fn part1<const N: usize>(grid: &Grid<N>) -> usize {
    let mut values = HashSet::new();
    let l = N - 1;
    check_sides(grid, &mut values, |a| (a, 0), |(a, b)| (a, b)); // Looking down
    check_sides(grid, &mut values, |a| (a, l), |(a, b)| (a, l - b)); // Looking up
    check_sides(grid, &mut values, |a| (0, a), |(a, b)| (b, a)); // Looking right
    check_sides(grid, &mut values, |a| (l, a), |(a, b)| (l - b, a)); // Looking left
    values.len()
}

#[aoc(day8, part2)]
pub fn part2<const N: usize>(grid: &Grid<N>) -> usize {
    (1..(N - 1))
        .into_iter()
        .flat_map(move |a| (1..(N - 1)).into_iter().map(move |b| (b, a)))
        .map(|c| check_possition(grid, c))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = include_str!("../test_inputs/day8.1.txt");
        let data = Grid::<5>::parse_grid(input);
        assert_eq!(part1(&data), 21)
    }

    #[test]
    fn test_example_part2() {
        let input = include_str!("../test_inputs/day8.1.txt");
        let data = Grid::<5>::parse_grid(input);
        assert_eq!(part2(&data), 8)
    }
}
