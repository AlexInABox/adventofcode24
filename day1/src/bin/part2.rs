use std::fs::read_to_string;

fn main() {
    let file_path: &str = "input.txt";

    let mut list1: Vec<u32> = vec![];
    let mut list2: Vec<u32> = vec![];

    let mut similarity_score: u32 = 0;

    for line in read_to_string(file_path).unwrap().lines() {
        let line: Vec<_> = line.split("   ").collect();
        list1.push(line[0].parse::<u32>().unwrap());
        list2.push(line[1].parse::<u32>().unwrap());
    }

    for n in 0..list1.len() {
        let num_to_check: u32 = list1[n];
        let count_of_num_to_check: u32 = list2
            .iter()
            .filter(|&n| *n == num_to_check)
            .count()
            .try_into()
            .unwrap();
        similarity_score += num_to_check * count_of_num_to_check;
    }
    println!("{}", similarity_score);
}
