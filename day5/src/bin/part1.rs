use std::collections::HashSet;
use std::fs::read_to_string;
use std::hash::Hash;

fn main() {
    let file_path: &str = "input.txt";

    let mut input_rules: Vec<(u32, u32)> = vec![];
    let mut input_updates: Vec<Vec<u32>> = vec![];
    let mut value: u32 = 0;

    let mut reading_rules: bool = true;
    let file = read_to_string(file_path).unwrap();
    for line in file.lines() {
        if line == "" {
            println!("Empty line!");
            reading_rules = false;
            continue;
        }
        if reading_rules {
            let mut iter = line.splitn(2, '|');
            input_rules.push((
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            ));
            continue;
        }

        let vec_of_u32: Vec<u32> = line
            .split(',')
            .filter_map(|s| s.parse().ok()) // Try parsing each part to u32, ignore invalid entries
            .collect();
        input_updates.push(vec_of_u32);
    }

    for update in input_updates {
        let mut update_valid: bool = true;
        for n in 0..update.len() {
            let mut list_of_forbidden_predecessors: Vec<u32> =
                get_list_of_forbidden_predecessors(&input_rules, &update[n]);
            let mut list_of_predecessors: Vec<u32> = get_list_of_predecessors(&update, n);
            let mut combined_list: Vec<u32> = vec![];

            combined_list.append(&mut list_of_forbidden_predecessors);

            combined_list.append(&mut list_of_predecessors);

            if !has_unique_elements(combined_list) {
                //Theres a duplicate in the combined_list meaning that theres a forbidden predecessor in the list of predecessors!
                update_valid = false;
                break;
            }
        }

        if update_valid {
            value += get_middle_page_number(&update);
        }
    }

    println!("{}", value);
}

fn get_list_of_forbidden_predecessors(input_rules: &Vec<(u32, u32)>, number: &u32) -> Vec<u32> {
    let mut list_of_forbidden_predecessors: Vec<u32> = vec![];
    for n in 0..input_rules.len() {
        if &input_rules[n].0 == number {
            list_of_forbidden_predecessors.push(input_rules[n].1);
        }
    }

    return list_of_forbidden_predecessors;
}

fn get_list_of_predecessors(update: &Vec<u32>, index: usize) -> Vec<u32> {
    let mut list_of_predecessors: Vec<u32> = vec![];
    for n in 0..index {
        list_of_predecessors.push(update[n]);
    }
    return list_of_predecessors;
}

fn get_middle_page_number(update: &Vec<u32>) -> u32 {
    return update[update.len() / 2];
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}
