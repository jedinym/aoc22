use clap::Parser;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod solution;

use solution::Solution;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    week: u8,
    input_file: String,
}

fn get_solver(input_file: &String, week: u8) -> Box<dyn Solution> {
    let contents = std::fs::read_to_string(input_file).unwrap();

    match week {
        1 => Box::new(day01::Calories::new(contents)),
        2 => Box::new(day02::RPS::new(contents)),
        3 => Box::new(day03::Rucksacks::new(contents)),
        4 => Box::new(day04::Cleanup::new(contents)),
        5 => Box::new(day05::Shipping::new(contents)),
        6 => Box::new(day06::Comms::new(contents)),
        _ => panic!("Unknown week"),
    }
}

fn print_solutions(solver: Box<dyn Solution>) {
    let res_part_one = solver.part_one();
    let res_part_two = solver.part_two();

    println!("Part 1: {}", res_part_one);
    println!("Part 2: {}", res_part_two);
}

fn main() {
    let args = Args::parse();
    let solver = get_solver(&args.input_file, args.week);

    print_solutions(solver);
}
