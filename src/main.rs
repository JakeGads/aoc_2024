mod file_manager;
mod day2;
fn main() {
    let x = file_manager::read("inputs/day2".to_string(), None);
    day2::part1(x.clone());
    day2::part2(x.clone());
}
