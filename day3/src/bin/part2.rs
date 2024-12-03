use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let file_path: &str = "input.txt";
    let mut value: u32 = 0;
    let file = read_to_string(file_path).unwrap();
    let re = Regex::new(r"(?m)don't\(\)|do\(\)|mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut do_flag = true;
    for cap in re.captures_iter(&file) {
        let match_text = cap.get(0).unwrap().as_str();
        if match_text == "do()" {
            do_flag = true;
            continue;
        } else if match_text == "don't()" {
            do_flag = false;
            continue;
        }

        // Parse the captured groups as u32
        let num1 = cap.get(1).unwrap().as_str().parse::<u32>();
        let num2 = cap.get(2).unwrap().as_str().parse::<u32>();

        // Handle the Result from parse
        match (num1, num2) {
            (Ok(val1), Ok(val2)) => {
                if do_flag {
                    value += val1 * val2;
                }
            }
            _ => eprintln!("Failed to parse one of the numbers."),
        }
    }

    println!("{}", value);
}
