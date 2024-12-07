use std::fs::read_to_string;

fn main() {
    let file_path: &str = "input.txt";

    let mut value: u64 = 0;
    let mut expected_results: Vec<u64> = vec![];
    let mut calculation_candidates: Vec<Vec<u64>> = vec![];

    let file = read_to_string(file_path).unwrap();
    for line in file.lines() {
        let mut split_line = line.splitn(2, ": ");
        expected_results.push(split_line.next().unwrap().parse().unwrap());

        let split_candidates = split_line.next().unwrap().split(' ');
        let mut candidates_in_line: Vec<u64> = vec![];
        for candidate in split_candidates {
            candidates_in_line.push(candidate.parse().unwrap());
        }
        calculation_candidates.push(candidates_in_line);
    }

    for calculation_index in 0..calculation_candidates.len() {
        let variations = generate_variations(calculation_candidates[calculation_index].len() - 1);
        for variation in variations {
            if calculate_with_variation(&variation, &calculation_candidates[calculation_index])
                == expected_results[calculation_index]
            {
                value += expected_results[calculation_index];
                break;
            }
        }
    }

    println!("{}", value);
}

fn calculate_with_variation(variation: &Vec<u64>, numbers: &Vec<u64>) -> u64 {
    let mut sum: u64 = numbers[0];
    for number_index in 1..numbers.len() {
        let temp_last_sum = sum;
        if variation[number_index - 1] == 0 {
            sum += numbers[number_index];
        }
        if variation[number_index - 1] == 1 {
            sum *= numbers[number_index];
        }
        if variation[number_index - 1] == 2 {
            let concentated_numbers: u64 =
                format!("{}{}", sum, numbers[number_index]).parse().unwrap();
            sum = concentated_numbers;
        }
    }
    return sum;
}

fn generate_variations(length: usize) -> Vec<Vec<u64>> {
    if length == 0 {
        return vec![vec![]];
    }

    let shorter_variations: Vec<Vec<u64>> = generate_variations(length - 1);
    let mut result: Vec<Vec<u64>> = vec![];

    for variation in shorter_variations {
        let mut plus_varation = variation.clone();
        plus_varation.push(0);
        let mut mult_varation = variation.clone();
        mult_varation.push(1);
        let mut combine_varation = variation.clone();
        combine_varation.push(2);

        result.push(plus_varation);
        result.push(mult_varation);
        result.push(combine_varation);
    }

    return result;
}
