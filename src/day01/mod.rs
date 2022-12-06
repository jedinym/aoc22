use crate::solution::Solution;

pub struct Calories {
    records: String,
}

impl Calories {
    pub fn new(input: String) -> Calories {
        return Calories { records: input };
    }
}

impl Solution for Calories {
    fn part_one(&self) -> String {
        let per_elf = self
            .records
            .split("\n\n")
            .map(|x| x.lines().map(|ln| ln.parse::<i32>().unwrap()).sum::<i32>());

        let per_elf_vec: Vec<i32> = per_elf.collect();
        let result = per_elf_vec.iter().max().unwrap();

        return result.to_string();
    }

    fn part_two(&self) -> String {
        let per_elf = self
            .records
            .split("\n\n")
            .map(|x| x.lines().map(|ln| ln.parse::<i32>().unwrap()).sum::<i32>());

        let mut per_elf_vec: Vec<i32> = per_elf.collect();

        // reverse sort
        per_elf_vec.sort_by(|a, b| b.cmp(a));

        let result = per_elf_vec.iter().take(3).sum::<i32>();
        return result.to_string();
    }
}
