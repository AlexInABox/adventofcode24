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
        if safe_ascending(&line) || safe_descending(&line) {
            score += 1;
            continue;
        }
        for n in 0..line.len() {
            let mut cut_line = line.clone();
            cut_line.remove(n);
            if safe_ascending(&cut_line) || safe_descending(&cut_line) {
                score += 1;
                break;
            }
        }
    }
    println!("{}", score);
}

fn safe_descending(line: &Vec<u32>) -> bool {
    let mut prev_digit: u32 = line[0];
    let line_length_cap = line.len() - 1;
    let mut index = 0;
    while index < line_length_cap {
        index += 1;

        if prev_digit > line[index] && prev_digit.abs_diff(line[index]) < 4 {
            prev_digit = line[index];
            continue;
        }

        return false;
    }
    return true;
}

fn safe_ascending(line: &Vec<u32>) -> bool {
    let mut prev_digit: u32 = line[0];
    let line_length_cap = line.len() - 1;
    let mut index = 0;
    while index < line_length_cap {
        index += 1;

        if prev_digit < line[index] && prev_digit.abs_diff(line[index]) < 4 {
            prev_digit = line[index];
            continue;
        }

        return false;
    }
    return true;
}
