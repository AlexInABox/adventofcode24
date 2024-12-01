use std::fs::read_to_string;

fn main() {
    let file_path: &str = "input.txt";

    let mut list1 = vec![];
    let mut list2 = vec![];

    let mut total_distance: u32 = 0;

    for line in read_to_string(file_path).unwrap().lines() {
        let line: Vec<_> = line.split("   ").collect();
        list1.push(line[0].parse::<i32>().unwrap());
        list2.push(line[1].parse::<i32>().unwrap());
    }

    list1.sort();
    list2.sort();

    for n in 0..list1.len() {
        total_distance += list1[n].abs_diff(list2[n]);
    }
    println!("{}", total_distance);
}
