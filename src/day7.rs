use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct FileStructure {
    fs: Vec<HashMap<String, FileType>>,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum FileType {
    Dir(usize),
    File(usize),
}

use FileType::{Dir, File};

impl FileStructure {
    fn new() -> Self {
        Self {
            fs: vec![HashMap::new()],
        }
    }

    fn mkdir(&mut self, wd: usize, name: String) {
        if let Some(files) = self.fs.get(wd) {
            if !files.contains_key("name") {
                let next_index = self.fs.len();
                self.fs.push(HashMap::new());
                assert!(self.fs[wd].insert(name, Dir(next_index)).is_none());
            }
        }
    }

    fn add_file(&mut self, wd: usize, name: String, size: usize) {
        assert!(self.fs[wd].insert(name, File(size)).is_none())
    }

    fn get_index(&self, wd: usize, name: &str) -> Option<usize> {
        self.fs[wd]
            .get(name)
            .map(|n| match n {
                Dir(d) => Some(*d),
                File(_) => None,
            })
            .flatten()
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> FileStructure {
    let mut current_dir = vec![0];
    let mut fs = FileStructure::new();
    for line in input.lines() {
        if line == "$ ls" {
            continue;
        } else if let Some((_, folder)) = line.split_once("$ cd ") {
            if folder == ".." {
                current_dir.pop();
            } else if folder == "/" {
                current_dir = vec![0];
            } else {
                current_dir.push(fs.get_index(*current_dir.last().unwrap(), folder).unwrap());
            }
        } else {
            let (fd, name) = line.split_once(' ').unwrap();
            if fd == "dir" {
                fs.mkdir(*current_dir.last().unwrap(), name.to_owned());
            } else {
                fs.add_file(
                    *current_dir.last().unwrap(),
                    name.to_owned(),
                    fd.parse().unwrap(),
                );
            }
        }
    }
    fs
}

fn get_dir_sizes(input: &FileStructure) -> Vec<usize> {
    let mut known_folder = vec![None; input.fs.len()];
    for i in (0..input.fs.len()).rev() {
        let mut unknowns = VecDeque::new();
        unknowns.push_back(i);
        while let Some(index) = unknowns.pop_back() {
            if known_folder[index].is_some() {
                continue;
            }
            let mut size = 0;
            for v in input.fs[index].values() {
                match v {
                    Dir(n) => match known_folder[*n] {
                        Some(s) => size += s,
                        None => {
                            unknowns.push_back(*n);
                            unknowns.push_front(index);
                        }
                    },
                    File(s) => size += s,
                }
            }
            known_folder[index] = Some(size);
        }
    }
    known_folder.into_iter().map(Option::unwrap).collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &FileStructure) -> usize {
    get_dir_sizes(input)
        .into_iter()
        .filter(|n| *n <= 100000)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &FileStructure) -> usize {
    let currently_used: usize = input
        .fs
        .iter()
        .flat_map(|n| n.values())
        .filter_map(|n| match n {
            File(n) => Some(*n),
            Dir(_) => None,
        })
        .sum();
    let max_size = 70000000;
    let needed_size = 30000000;
    let minimum_to_be_freed = needed_size + currently_used - max_size;

    let input = get_dir_sizes(input);

    input
        .into_iter()
        .filter(|n| *n >= minimum_to_be_freed)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test_inputs/day7.1.txt");
        let input = input_generator(&input);
        assert_eq!(95437, part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../test_inputs/day7.1.txt");
        let input = input_generator(&input);
        assert_eq!(24933642, part2(&input))
    }
}
