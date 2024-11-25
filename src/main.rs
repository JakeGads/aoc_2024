mod file_manager;
mod day1;

fn main() {
    let x = file_manager::read("inputs/day1".to_string(), None);
    day1::part1(x.clone());
    day1::part2(x.clone());
}
