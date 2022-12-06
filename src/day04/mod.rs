use crate::solution::Solution;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::str::FromStr;

#[derive(Debug)]
struct Range {
    left: u32,
    right: u32,
}

impl Range {
    fn new(left: u32, right: u32) -> Option<Range> {
        if left > right {
            return None;
        }

        Some(Range { left, right })
    }
}

impl BitAnd for Range {
    type Output = bool;

    fn bitand(self, rhs: Self) -> Self::Output {
        if self.left <= rhs.left && self.right >= rhs.left
            || rhs.left <= self.left && rhs.right >= self.left
        {
            return true;
        }

        if self.right >= rhs.right && self.left <= rhs.right
            || rhs.right >= self.right && rhs.left <= self.right
        {
            return true;
        }

        return self | rhs;
    }
}

impl BitOr for Range {
    type Output = bool;
    fn bitor(self, rhs: Self) -> Self::Output {
        if self.left >= rhs.left && self.right <= rhs.right {
            return true;
        }

        if rhs.left >= self.left && rhs.right <= self.right {
            return true;
        }

        return false;
    }
}

impl FromStr for Range {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_once("-");
        if let None = split {
            return Err("No dash found in input string");
        }

        let (leftstr, rightstr) = split.unwrap();
        let left = leftstr.parse::<u32>();
        let right = rightstr.parse::<u32>();

        let range = match (left, right) {
            (Ok(l), Ok(r)) => Range::new(l, r),
            _ => None,
        };

        if let Some(rng) = range {
            return Ok(rng);
        }

        return Err("Couldn't parse string");
    }
}

pub struct Cleanup {
    input: String,
}

impl Cleanup {
    pub fn new(input: String) -> Cleanup {
        Cleanup { input }
    }

    fn get_ranges(&self) -> Vec<(Range, Range)> {
        let mut ranges = Vec::new();

        for line in self.input.lines() {
            let split = line.split_once(",").unwrap();
            let first_r = Range::from_str(split.0);
            let second_r = Range::from_str(split.1);

            match (first_r, second_r) {
                (Ok(first), Ok(second)) => ranges.push((first, second)),
                _ => panic!("Parse error"),
            };
        }

        return ranges;
    }
}

impl Solution for Cleanup {
    fn part_one(&self) -> String {
        let ranges = self.get_ranges();
        let count_crossing = ranges
            .into_iter()
            .map(|(fst, snd)| fst | snd)
            .filter(|b| *b)
            .count();

        return count_crossing.to_string();
    }

    fn part_two(&self) -> String {
        let ranges = self.get_ranges();

        let count_overlapping = ranges
            .into_iter()
            .map(|(fst, snd)| fst & snd)
            .filter(|b| *b)
            .count();

        return count_overlapping.to_string();
    }
}
