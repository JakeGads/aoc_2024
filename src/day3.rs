use regex::Regex;

pub fn part1(inputs: Vec<String> ){
    println!("Day 3 Part 1");
    println!("The sum of the multipliers is: {}", find_mul(inputs.join("").as_str()));
}
pub fn part2_second_attempt(inputs: Vec<String>){
    fn split_string_into_array_of_do_through_dont(input: &str) -> Vec<String> {
        // capture all text that happens between do() and don't(), that's what we'll be checking
        let re = Regex::new(r"do\(\)(.*?)don't\(\)").unwrap();
        let mut result = Vec::new();
        for cap in re.captures_iter(input) {
            result.push(cap[1].to_string());
        }
        result
    }
    // add a preceding do to capture any muls that happen before the first do
    // add a final don't to include the last set of do's to the total
    let mut input_string = inputs.join("");
    input_string = format!("do(){}don't()", input_string);
    let mut sum: i32 = 0;
    for i in split_string_into_array_of_do_through_dont(&input_string) {
        sum += find_mul(&i);
    }
    println!("Day 3 Part 2");
    println!("The sum of the multipliers is: {}", sum);
}


fn find_mul(text: &str) -> i32 {
    let outer_re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let inner_re = Regex::new(r"\d{1,3}").unwrap();
    let mut sum = 0;
    for cap in outer_re.captures_iter(text) {
        let mut multiplier = 1;
        for cap2 in inner_re.captures_iter(&cap[0]) {
            multiplier *= cap2[0].parse::<i32>().unwrap();
        }
        sum += multiplier;
    }
    sum
}