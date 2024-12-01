pub fn part1(inputs: Vec<String> ){
    let (mut l1, mut l2) = gather(inputs);
    // Sort the lists
    l1.sort();
    l2.sort();
    let mut sum = 0;
    for i in 0..l1.len() {
        sum += (l1[i] - l2[i]).abs();
    }
    println!("The sum of the differences is: {}", sum);
}
pub fn part2(inputs: Vec<String> ){
    let (l1, l2) = gather(inputs);
    let mut similarity_score = 0;

    for i in 0..l1.len() {
        similarity_score += l1[i] * count_instances(&l2, l1[i]) as i32;
    }
    println!("The similarity score is: {}", similarity_score);
}
/**
 * Count the number of instances of a number in a list
 *
 * @param vec: &Vec<i32>
 * @param number: i32
 * @return usize
 */
fn count_instances(vec: &Vec<i32>, number: i32) -> usize {
    vec.iter().filter(|&&x| x == number).count()
}

/**
 * Gather the inputs into two lists
 * something weri d is going on with git adding this to hope to fix it
 * @param inputs: Vec<String>
 * @return (Vec<i32>, Vec<i32>)
 */
fn gather(inputs: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    for i in &inputs {
        let x = i.split("   ").collect::<Vec<&str>>();
        if x.len() != 2 {
            break;
        }
        list1.push(x[0].parse::<i32>().unwrap());
        list2.push(x[1].parse::<i32>().unwrap());
    }
    (list1, list2)
}