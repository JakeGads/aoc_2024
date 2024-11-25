mod file_manager;

fn main() {
    println!("Hello, world!");
    let x = file_manager::read("src/main.rs".to_string(), Some(';'));
    println!("{:?}", x);
}
