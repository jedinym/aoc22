use std::collections::HashSet;

use crate::solution::Solution;

pub struct Comms {
    signal: String,
}

impl Comms {
    pub fn new(input: String) -> Self {
        return Comms { signal: input };
    }

    fn has_duplicate(set: &mut HashSet<char>, slc: &[char]) -> bool {
        set.extend(slc);
        return set.len() == slc.len();
    }

    fn find_marker(&self, len: usize) -> usize {
        let mut index: usize = len;
        let chars = self.signal.chars().collect::<Vec<char>>();
        let mut set = HashSet::with_capacity(len);

        for window in chars[..].windows(len) {
            if Self::has_duplicate(&mut set, &window) {
                break;
            }

            set.clear();
            index += 1;
        }

        index
    }
}

impl Solution for Comms {
    fn part_one(&self) -> String {
        self.find_marker(4).to_string()
    }

    fn part_two(&self) -> String {
        self.find_marker(14).to_string()
    }
}
