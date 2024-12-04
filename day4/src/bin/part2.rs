use std::fs::read_to_string;

fn main() {
    let file_path: &str = "input.txt";

    let mut input_array = vec![];

    let mut count_of_occurrences: u32 = 0;

    let file = read_to_string(file_path).unwrap();
    for line in file.lines() {
        input_array.push(line.as_bytes()); //using "as_bytes" we convert the string into an array of bytes. one byte representing one char
    }

    //At this point "input_array" is a 2D "array" filled with each line

    for row in 0..input_array.len() {
        for column in 0..input_array[0].len() {
            if input_array[row][column] as char != 'A' {
                continue;
            }

            if !(two_m_two_s(row, column, &input_array)) {
                continue;
            }

            if (point_symetric(row, column, &input_array)) {
                continue;
            }
            count_of_occurrences += 1;
        }
    }

    println!("{}", count_of_occurrences);
}

fn two_m_two_s(row: usize, column: usize, input_array: &Vec<&[u8]>) -> bool {
    if !(column >= 1
        && row >= 1
        && row + 1 < input_array.len()
        && column + 1 < input_array[row + 1].len())
    {
        return false;
    }

    let mut amount_of_m = 0;
    let mut amount_of_s = 0;

    if input_array[row + 1][column + 1] as char == 'M' {
        amount_of_m += 1;
    }

    if input_array[row + 1][column + 1] as char == 'S' {
        amount_of_s += 1;
    }

    if input_array[row + 1][column - 1] as char == 'M' {
        amount_of_m += 1;
    }

    if input_array[row + 1][column - 1] as char == 'S' {
        amount_of_s += 1;
    }

    if input_array[row - 1][column - 1] as char == 'M' {
        amount_of_m += 1;
    }

    if input_array[row - 1][column - 1] as char == 'S' {
        amount_of_s += 1;
    }

    if input_array[row - 1][column + 1] as char == 'M' {
        amount_of_m += 1;
    }

    if input_array[row - 1][column + 1] as char == 'S' {
        amount_of_s += 1;
    }

    return amount_of_m == 2 && amount_of_s == 2;
}

fn point_symetric(row: usize, column: usize, input_array: &Vec<&[u8]>) -> bool {
    return input_array[row + 1][column + 1] as char == input_array[row - 1][column - 1] as char;
}
