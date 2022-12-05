use clap::Parser;

mod solution;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;


use solution::Solution;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
   week: u8,
   input_file: String,
}

enum Solver {
    Day01(day01::Calories),
    Day02(day02::RPS),
    Day03(day03::Rucksacks),
    Day04(day04::Cleanup),
    Day05(day05::Shipping),
}

fn get_solver(input_file: &String, week: u8) -> Solver {
    let contents = std::fs::read_to_string(input_file).unwrap();

    let solver = match week {
        1 => Solver::Day01(day01::Calories::new(contents)),
        2 => Solver::Day02(day02::RPS::new(contents)),
        3 => Solver::Day03(day03::Rucksacks::new(contents)),
        4 => Solver::Day04(day04::Cleanup::new(contents)),
        5 => Solver::Day05(day05::Shipping::new(contents)),
        _ => panic!("Unknown week")
    };

    return solver;
}

fn print_solutions<T: Solution>(solver: T) {
    let res_part_one = solver.part_one();
    let res_part_two = solver.part_two();

    println!("Part 1: {}", res_part_one);
    println!("Part 2: {}", res_part_two);
}

fn main() {
    let args = Args::parse();
    match get_solver(&args.input_file, args.week) {
        Solver::Day01(sv) => print_solutions(sv),
        Solver::Day02(sv) => print_solutions(sv),
        Solver::Day03(sv) => print_solutions(sv),
        Solver::Day04(sv) => print_solutions(sv),
        Solver::Day05(sv) => print_solutions(sv),
    }
}
