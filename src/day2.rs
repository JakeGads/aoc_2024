pub fn part1(inputs: Vec<String>){
    let parsed = parse_input(inputs); // make the output into a Vector of Vectors of ints
    let mut safe = 0; // create a counter
    for line in parsed{
        if is_safe(line) { safe += 1; } // test and increment
    }
    println!("Part 1: {:?} are safe", safe);

}

pub fn part2(inputs: Vec<String>){
    let parsed = parse_input(inputs); // parse
    let mut safe = 0; // create counter
    for line in parsed{
        // iterate over the line, removing one and only one number and testing if it's safe
        // we don't have to test the full line because if
        // the full line is correct so is the line without the 0th item
        for i in 0..line.len(){
            let mut temp = line.clone();
            temp.remove(i);
            if is_safe(temp) { safe += 1; break;}
        }
    }
    println!("Part 2: {:?} are safe", safe);
}
fn parse_input(inputs: Vec<String>) -> Vec<Vec<i32>> {
    let mut master = Vec::new();
    for line in inputs {
        let mut temp = Vec::new();
        for num in line.split(" ") {
            temp.push(num.parse::<i32>().unwrap());
        }
        master.push(temp);
    }
    master
}
fn is_safe(line: Vec<i32>) -> bool {
    let mut last = 0; // variable to store the last number
    let mut is_safe = true; // variable to store if the line is safe
    let direction_is_up = (line[1] - line[0]) > 0; // detects the order
    for num in line{ // loop
        if last == 0{ // first run set the init last
            last = num;
            continue;
        }
        // if we've already found a number that's not safe, break
        if !is_safe { break; }
        // get the diff
        let diff = num - last;
        // test the abs value to ignore those pesky negatives
        if diff.abs() < 1 || diff.abs() > 3{ is_safe = false }

        // test if the direction is correct
        if direction_is_up {
            if diff < 0 { is_safe = false; }
        } else {
            if diff > 0 { is_safe = false; }
        }

        // update the value of last
        last = num;
    }
    // return the safety-ness
    is_safe
}
