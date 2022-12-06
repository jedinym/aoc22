use itertools::Itertools;

use crate::solution::Solution;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
use std::str::FromStr;

lazy_static! {
    static ref CONTAINER_RGX: Regex = Regex::new(r#"\[(.)\] ?"#).unwrap();
    static ref INS_RGX: Regex = Regex::new(r#"move (\d*) from (\d*) to (\d*)"#).unwrap();
}

type Columns = Vec<VecDeque<char>>;

#[derive(Debug)]
struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

fn get_number_unsafe(s: &str) -> usize {
    return s.parse::<usize>().expect("Cannot parse number");
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = INS_RGX.captures(s).unwrap();

        let count = get_number_unsafe(r.get(1).unwrap().as_str());
        let from = get_number_unsafe(r.get(2).unwrap().as_str());
        let to = get_number_unsafe(r.get(3).unwrap().as_str());

        Ok(Instruction { from, to, count })
    }
}

#[derive(Debug)]
pub struct Shipping {
    columns: Columns,
    instructions: Vec<Instruction>,
}

impl Shipping {
    pub fn new(input: String) -> Shipping {
        let mut columns = Vec::new();
        let first_line = input.lines().next().unwrap();

        if (first_line.len() + 1) % 4 != 0 {
            panic!("Invalid input");
        }

        for _ in 0..((first_line.len() + 1) / 4) {
            let column = VecDeque::new();
            columns.push(column);
        }

        Self::parse_containers(&input, &mut columns);
        let instructions = Self::parse_instructions(&input);

        Shipping {
            columns,
            instructions,
        }
    }

    fn parse_instructions(input: &String) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        for line in input.lines().skip_while(|ln| !ln.contains("move")) {
            let ins = Instruction::from_str(line).expect("Could not parse instruction");
            instructions.push(ins);
        }

        return instructions;
    }

    fn parse_containers(input: &String, columns: &mut Columns) {
        for line in input.lines() {
            let mut col_index = 0;

            for container_raw in &line.chars().chunks(4) {
                let text = String::from(container_raw.collect::<String>());
                let captured_char = CONTAINER_RGX.captures(&text);

                if let Some(captured) = captured_char {
                    let chr = captured.get(1).unwrap().as_str();
                    columns[col_index].push_back(chr.chars().next().unwrap());
                }

                col_index += 1;
            }
        }
    }

    fn executor_one(ins: &Instruction, columns: &mut Columns) {
        for _ in 0..ins.count {
            let container = columns[ins.from - 1].pop_front().expect("Empty column!");
            columns[ins.to - 1].push_front(container);
        }
    }

    fn executor_two(ins: &Instruction, columns: &mut Columns) {
        let buffer = {
            let mut bf: Vec<char> = Vec::new();

            for _ in 0..ins.count {
                bf.push(columns[ins.from - 1].pop_front().expect("Empty column!"));
            }

            bf
        };

        for container in buffer.into_iter().rev() {
            columns[ins.to - 1].push_front(container)
        }
    }

    fn rearrange_crates(
        columns: &mut Columns,
        instructions: &Vec<Instruction>,
        executor: fn(&Instruction, &mut Columns),
    ) {
        for instruction in instructions {
            executor(instruction, columns);
        }
    }

    fn get_tops(columns: &Vec<VecDeque<char>>) -> String {
        let mut tops = String::with_capacity(columns.len());

        for column in columns {
            tops.push(*column.iter().next().expect("Empty column after organzing!"));
        }

        tops
    }
}

impl Solution for Shipping {
    fn part_one(&self) -> String {
        let mut columns = self.columns.clone();
        Self::rearrange_crates(&mut columns, &self.instructions, Self::executor_one);

        Self::get_tops(&columns)
    }

    fn part_two(&self) -> String {
        let mut columns = self.columns.clone();
        Self::rearrange_crates(&mut columns, &self.instructions, Self::executor_two);

        Self::get_tops(&columns)
    }
}
