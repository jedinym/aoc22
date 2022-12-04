use crate::solution::Solution;
use std::str::FromStr;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

enum GameResult {
    Win(Play),
    Loss(Play),
    Draw(Play),
}

#[derive(Clone, Copy)]
enum ExpectedResult {
    Win,
    Loss,
    Draw,
}

impl FromStr for ExpectedResult {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err("aaah"),
        }
    }
}

impl Play {
    fn score(&self) -> u32 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    fn eval(me: Play, them: Play) -> GameResult {
        if me == them {
            return GameResult::Draw(me);
        }
        let win = GameResult::Win(me);

        match (me, them) {
            (Play::Rock, Play::Scissors) => win,
            (Play::Paper, Play::Rock) => win,
            (Play::Scissors, Play::Paper) => win,
            _ => GameResult::Loss(me),
        }
    }

    fn eval_result(them: Play, result: ExpectedResult) -> GameResult {
        match result {
            ExpectedResult::Win => match them {
                Play::Rock => GameResult::Win(Play::Paper),
                Play::Paper => GameResult::Win(Play::Scissors),
                Play::Scissors => GameResult::Win(Play::Rock),
            },
            ExpectedResult::Draw => match them {
                Play::Rock => GameResult::Draw(Play::Rock),
                Play::Paper => GameResult::Draw(Play::Paper),
                Play::Scissors => GameResult::Draw(Play::Scissors),
            },
            ExpectedResult::Loss => match them {
                Play::Rock => GameResult::Loss(Play::Scissors),
                Play::Paper => GameResult::Loss(Play::Rock),
                Play::Scissors => GameResult::Loss(Play::Paper),
            },
        }
    }
}

impl FromStr for Play {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Play::Rock),
            "B" => Ok(Play::Paper),
            "C" => Ok(Play::Scissors),
            "X" => Ok(Play::Rock),
            "Y" => Ok(Play::Paper),
            "Z" => Ok(Play::Scissors),
            _ => Err("Oh boy, this surely won't be unwrapped"),
        }
    }
}

impl GameResult {
    fn score(&self) -> u32 {
        match self {
            Self::Win(play) => 6 + play.score(),
            Self::Draw(play) => 3 + play.score(),
            Self::Loss(play) => 0 + play.score(),
        }
    }
}

pub struct RPS {
    split_lines: Vec<(String, String)>,
}

impl RPS {
    pub fn new(input: String) -> RPS {
        let lines = input.lines();

        // Get lines into Vec of String tuples
        let split_lines = lines
            .map(|x| x.split_once(" ").unwrap())
            .map(|y| (y.0.to_owned(), y.1.to_owned()))
            .collect::<Vec<(String, String)>>();

        RPS { split_lines }
    }
}

impl Solution for RPS {
    fn part_one(&self) -> String {
        let play_tuples: Vec<(Play, Play)> = self
            .split_lines
            .iter()
            .map(|x| {
                (
                    (Play::from_str(&x.1).unwrap()),
                    Play::from_str(&x.0).unwrap(),
                )
            })
            .collect::<Vec<(Play, Play)>>();

        let final_score = play_tuples
            .iter()
            .map(|x| Play::eval(x.0, x.1).score())
            .sum::<u32>();

        return final_score.to_string();
    }

    fn part_two(&self) -> String {
        let play_result_tuples: Vec<(Play, ExpectedResult)> = self
            .split_lines
            .iter()
            .map(|x| {
                (
                    (Play::from_str(&x.0).unwrap()),
                    ExpectedResult::from_str(&x.1).unwrap(),
                )
            })
            .collect::<Vec<(Play, ExpectedResult)>>();

        let final_score = play_result_tuples
            .iter()
            .map(|x| Play::eval_result(x.0, x.1).score())
            .sum::<u32>();

        return final_score.to_string();
    }
}
