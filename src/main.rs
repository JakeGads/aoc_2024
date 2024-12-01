mod file_manager;

fn main() {
    let x = file_manager::read("inputs/day1".to_string(), None);
    println!("{:?}", x);
}
