use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Move {
    ammount: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn new(ammount: usize, from: usize, to: usize) -> Self {
        Self {
            ammount,
            from: from - 1,
            to: to - 1,
        }
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split(' ').skip(1).step_by(2).map(|n| n.parse().unwrap());
        Ok(Self::new(
            values.next().unwrap(),
            values.next().unwrap(),
            values.next().unwrap(),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct State {
    state: Vec<Vec<char>>,
}

impl State {
    pub fn apply_p1(&mut self, m: &Move) {
        for _ in 0..m.ammount {
            let pop = self.state[m.from].pop().unwrap();
            self.state[m.to].push(pop);
        }
    }

    pub fn apply_p2(&mut self, m: &Move) {
        let mut stack = Vec::with_capacity(m.ammount);
        for _ in 0..m.ammount {
            let pop = self.state[m.from].pop().unwrap();
            stack.push(pop);
        }
        stack.reverse();
        self.state[m.to].extend(stack);
    }

    pub fn word(&self) -> String {
        self.state.iter().map(|n| *n.last().unwrap()).collect()
    }
}

impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spots = vec![vec![]; 9];
        s.lines()
            .rev()
            .skip(1) // numbers
            .for_each(|line| {
                line.chars()
                    .skip(1)
                    .step_by(4)
                    .enumerate()
                    .filter(|(_, c)| *c != ' ')
                    .for_each(|(n, c)| spots[n].push(c))
            });
        Ok(Self { state: spots })
    }
}

pub type AocType = (State, Vec<Move>);

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> AocType {
    let (state, moves) = input.split_once("\n\n").unwrap();
    (
        state.parse().unwrap(),
        moves.lines().map(|n| n.parse().unwrap()).collect(),
    )
}

#[aoc(day5, part1)]
pub fn part1(input: &AocType) -> String {
    let (state, moves) = input;

    let mut state = state.clone();
    for m in moves {
        state.apply_p1(m);
    }
    state.word()
}

#[aoc(day5, part2)]
pub fn part2(input: &AocType) -> String {
    let (state, moves) = input;

    let mut state = state.clone();
    for m in moves {
        state.apply_p2(m);
    }
    state.word()
}
