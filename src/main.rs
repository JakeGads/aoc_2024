mod file_manager;
mod day3;

fn main() {
    let x = file_manager::read("inputs/2024/day3".to_string(), None);
    day3::part1(x.clone());
    // println!("{:?}", day3::part2("inputs/2024/day3".to_string()));
    day3::part2_second_attempt(x.clone());
}
