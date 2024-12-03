use std::fs::read_to_string;

fn main() {
    let file_path: &str = "input.txt";

    let mut all_lines = vec![];

    let mut score: u32 = 0;

    let file = read_to_string(file_path).unwrap();
    for line in file.lines() {
        let line: Vec<u32> = line.split(" ").map(|s| s.parse().unwrap()).collect();
        all_lines.push(line);
    }

    for line in all_lines {
        if safe_descending(&line) {
            score += 1;
        } else if safe_ascending(&line) {
            score += 1;
        } else {
            continue;
        }
    }

    println!("{}", score);
}

fn safe_descending(line: &Vec<u32>) -> bool {
    let mut prev_digit = line[0];
    for n in 1..line.len() {
        if prev_digit > line[n] && prev_digit.abs_diff(line[n]) < 4 {
            prev_digit = line[n];
            continue;
        }
        return false;
    }
    return true;
}

fn safe_ascending(line: &Vec<u32>) -> bool {
    let mut prev_digit = line[0];
    for n in 1..line.len() {
        if prev_digit < line[n] && prev_digit.abs_diff(line[n]) < 4 {
            prev_digit = line[n];
            continue;
        }
        return false;
    }
    return true;
}
