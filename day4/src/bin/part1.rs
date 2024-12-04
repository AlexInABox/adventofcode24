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
            if input_array[row][column] as char != 'X' {
                continue;
            }
            if right(row, column, &input_array) {
                count_of_occurrences += 1;
            }
            if down_right(row, column, &input_array) {
                count_of_occurrences += 1;
            }
            if down(row, column, &input_array) {
                count_of_occurrences += 1;
            }
            if down_left(row, column, &input_array) {
                count_of_occurrences += 1;
            }
            if left(row, column, &input_array) {
                count_of_occurrences += 1;
            }
            if up_left(row, column, &input_array) {
                count_of_occurrences += 1;
            }
            if up(row, column, &input_array) {
                count_of_occurrences += 1;
            }
            if up_right(row, column, &input_array) {
                count_of_occurrences += 1;
            }
        }
    }

    println!("{}", count_of_occurrences);
}

fn right(row: usize, column: usize, input_array: &Vec<&[u8]>) -> bool {
    if !(column + 3 < input_array[row].len()) {
        return false;
    }

    let m_exists = input_array[row][column + 1] as char == 'M';
    let a_exists = input_array[row][column + 2] as char == 'A';
    let s_exists = input_array[row][column + 3] as char == 'S';
    return m_exists && a_exists && s_exists;
}

fn down_right(row: usize, column: usize, input_array: &Vec<&[u8]>) -> bool {
    if !(column + 3 < input_array[row].len() && row + 3 < input_array.len()) {
        return false;
    }

    let m_exists = input_array[row + 1][column + 1] as char == 'M';
    let a_exists = input_array[row + 2][column + 2] as char == 'A';
    let s_exists = input_array[row + 3][column + 3] as char == 'S';
    return m_exists && a_exists && s_exists;
}

fn down(row: usize, column: usize, input_array: &Vec<&[u8]>) -> bool {
    if !(row + 3 < input_array.len()) {
        return false;
    }

    let m_exists = input_array[row + 1][column] as char == 'M';
    let a_exists = input_array[row + 2][column] as char == 'A';
    let s_exists = input_array[row + 3][column] as char == 'S';
    return m_exists && a_exists && s_exists;
}

fn down_left(row: usize, column: usize, input_array: &Vec<&[u8]>) -> bool {
    if !(column >= 3 && row + 3 < input_array.len()) {
        return false;
    }

    let m_exists = input_array[row + 1][column - 1] as char == 'M';
    let a_exists = input_array[row + 2][column - 2] as char == 'A';
    let s_exists = input_array[row + 3][column - 3] as char == 'S';
    return m_exists && a_exists && s_exists;
}

fn left(row: usize, column: usize, input_array: &Vec<&[u8]>) -> bool {
    if !(column >= 3) {
        return false;
    }

    let m_exists = input_array[row][column - 1] as char == 'M';
    let a_exists = input_array[row][column - 2] as char == 'A';
    let s_exists = input_array[row][column - 3] as char == 'S';
    return m_exists && a_exists && s_exists;
}

fn up_left(row: usize, column: usize, input_array: &Vec<&[u8]>) -> bool {
    if !(column >= 3 && row >= 3) {
        return false;
    }

    let m_exists = input_array[row - 1][column - 1] as char == 'M';
    let a_exists = input_array[row - 2][column - 2] as char == 'A';
    let s_exists = input_array[row - 3][column - 3] as char == 'S';
    return m_exists && a_exists && s_exists;
}

fn up(row: usize, column: usize, input_array: &Vec<&[u8]>) -> bool {
    if !(row >= 3) {
        return false;
    }

    let m_exists = input_array[row - 1][column] as char == 'M';
    let a_exists = input_array[row - 2][column] as char == 'A';
    let s_exists = input_array[row - 3][column] as char == 'S';
    return m_exists && a_exists && s_exists;
}

fn up_right(row: usize, column: usize, input_array: &Vec<&[u8]>) -> bool {
    if !(column + 3 < input_array[row].len() && row >= 3) {
        return false;
    }

    let m_exists = input_array[row - 1][column + 1] as char == 'M';
    let a_exists = input_array[row - 2][column + 2] as char == 'A';
    let s_exists = input_array[row - 3][column + 3] as char == 'S';
    return m_exists && a_exists && s_exists;
}
