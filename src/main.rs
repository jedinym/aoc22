use clap::Parser;

mod solution;
mod week01;
mod week02;
mod week03;
mod day04;


use solution::Solution;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
   week: u8,
   input_file: String,
}

enum Solver {
    Week01(week01::Calories),
    Week02(week02::RPS),
    Week03(week03::Rucksacks),
    Day04(day04::Cleanup),
}

fn get_solver(input_file: &String, week: u8) -> Solver {
    let contents = std::fs::read_to_string(input_file).unwrap();

    let solver = match week {
        1 => Solver::Week01(week01::Calories::new(contents)),
        2 => Solver::Week02(week02::RPS::new(contents)),
        3 => Solver::Week03(week03::Rucksacks::new(contents)),
        4 => Solver::Day04(day04::Cleanup::new(contents)),
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
        Solver::Week01(sv) => print_solutions(sv),
        Solver::Week02(sv) => print_solutions(sv),
        Solver::Week03(sv) => print_solutions(sv),
        Solver::Day04(sv) => print_solutions(sv),
    }
}
