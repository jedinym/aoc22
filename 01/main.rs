use std::fs;

fn main() {
    let records: String = fs::read_to_string("input").unwrap();

    let per_elf = records
        .split("\n\n")
        .map(|x| {
            x.lines()
             .map(|ln| ln.parse::<i32>().unwrap())
             .sum::<i32>()
        });

    let mut per_elf_vec: Vec<i32> = per_elf.collect();

    println!("Part 1: {}", per_elf_vec.iter().max().unwrap());

    // reverse sort
    per_elf_vec.sort_by(|a, b| b.cmp(a));

    println!("Part 2: {}", per_elf_vec.iter().take(3).sum::<i32>());
}
