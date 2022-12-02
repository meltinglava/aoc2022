use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Copy, Clone)]
pub enum RPSResult {
    Win,
    Draw,
    Loose,
}

use RPSResult::*;
use RPS::*;

impl FromStr for RPS {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Rock),
            "B" => Ok(Paper),
            "C" => Ok(Scissors),
            "X" => Ok(Rock),
            "Y" => Ok(Paper),
            "Z" => Ok(Scissors),
            _ => Err(format!("Does not work: {}", s)),
        }
    }
}

impl FromStr for RPSResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Loose),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err(format!("Does not work: {}", s)),
        }
    }
}

impl RPSResult {
    fn score(&self) -> usize {
        match self {
            Win => 6,
            Draw => 3,
            Loose => 0,
        }
    }
}

impl RPS {
    fn score(&self, other: &Self) -> usize {
        self.shape_score() + self.game_result(other).score()
    }

    fn score_from_outcome(&self, outcome: &RPSResult) -> usize {
        let choice = self.rps_from_outcome(outcome);
        choice.shape_score() + choice.game_result(self).score()
    }

    fn shape_score(&self) -> usize {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn game_result(&self, other: &Self) -> RPSResult {
        match self {
            Rock => match other {
                Rock => Draw,
                Paper => Loose,
                Scissors => Win,
            },
            Paper => match other {
                Rock => Win,
                Paper => Draw,
                Scissors => Loose,
            },
            Scissors => match other {
                Rock => Loose,
                Paper => Win,
                Scissors => Draw,
            },
        }
    }

    fn rps_from_outcome(&self, outcome: &RPSResult) -> Self {
        match self {
            Rock => match outcome {
                Draw => Rock,
                Win => Paper,
                Loose => Scissors,
            },
            Paper => match outcome {
                Loose => Rock,
                Draw => Paper,
                Win => Scissors,
            },
            Scissors => match outcome {
                Win => Rock,
                Loose => Paper,
                Draw => Scissors,
            },
        }
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|n| n.split_once(' ').unwrap())
        .map(|(a, b)| (b.parse::<RPS>().unwrap(), a.parse::<RPS>().unwrap()))
        .map(|(s, o)| s.score(&o))
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|n| n.split_once(' ').unwrap())
        .map(|(a, b)| (a.parse::<RPS>().unwrap(), b.parse::<RPSResult>().unwrap()))
        .map(|(s, o)| s.score_from_outcome(&o))
        .sum()
}
