use crate::solution::Solution;
use itertools::Itertools;
use std::clone::Clone;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

pub struct Rucksacks {
    input: String,
}

impl Rucksacks {
    pub fn new(input: String) -> Rucksacks {
        Rucksacks { input }
    }

    fn split_line(line: &str) -> (&str, &str) {
        let index = line.len() / 2;

        line.split_at(index)
    }

    fn get_score(c: char) -> u32 {
        if c.is_ascii_lowercase() {
            return c as u32 - 96;
        }

        if c.is_ascii_uppercase() {
            return c as u32 - 38;
        }

        panic!("Unknown character!");
    }

    fn find_intersectors<T>(sources: Vec<Vec<T>>) -> Option<T>
    where
        T: Eq + Hash + Clone + Debug,
    {
        let mut sets: Vec<HashSet<T>> = Vec::with_capacity(sources.len());

        for source in sources {
            let mut set = HashSet::new();

            for item in source {
                set.insert(item);
            }

            sets.push(set);
        }

        let mut result: HashSet<T> = sets[0].clone();

        for set in sets.iter().skip(1) {
            result = &result & &set;
        }

        return result.into_iter().next();
    }

    fn score_sources(sources: Vec<Vec<char>>) -> u32 {
        let intersectors = Self::find_intersectors(sources);

        Self::get_score(intersectors.unwrap())
    }
}

impl Solution for Rucksacks {
    fn part_one(&self) -> String {
        let mut score: u32 = 0;

        for line in self.input.lines() {
            let compartments: (&str, &str) = Self::split_line(line);
            let left_chars = compartments.0.chars().collect::<Vec<char>>();
            let right_chars = compartments.1.chars().collect::<Vec<char>>();

            let sources = vec![left_chars, right_chars];

            score += Self::score_sources(sources);
        }

        return score.to_string();
    }

    fn part_two(&self) -> String {
        let mut score: u32 = 0;

        for (one, two, three) in self.input.lines().tuples() {
            let sources = vec![
                one.chars().collect::<Vec<char>>(),
                two.chars().collect::<Vec<char>>(),
                three.chars().collect::<Vec<char>>(),
            ];

            score += Self::score_sources(sources);
        }

        return score.to_string();
    }
}
