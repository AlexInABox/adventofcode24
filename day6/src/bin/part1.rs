use std::fs::read_to_string;

fn main() {
    let file_path: &str = "input.txt";

    let mut input: Vec<Vec<char>> = vec![];

    let file = read_to_string(file_path).unwrap();
    for line in file.lines() {
        input.push(line.chars().collect());
    }

    loop {
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                if input[y][x] == '^' {
                    if y == 0 {
                        input[y][x] = 'X';
                        println!("{}", calculate_distinct_locations(&input));
                        return;
                    }
                    if input[y - 1][x] == '#' {
                        input[y][x] = '>';
                        continue;
                    }

                    input[y][x] = 'X';
                    input[y - 1][x] = '^';
                }
                if input[y][x] == '>' {
                    if x == input[y].len() - 1 {
                        input[y][x] = 'X';
                        println!("{}", calculate_distinct_locations(&input));
                        return;
                    }
                    if input[y][x + 1] == '#' {
                        input[y][x] = 'v';
                        continue;
                    }

                    input[y][x] = 'X';
                    input[y][x + 1] = '>';
                }
                if input[y][x] == 'v' {
                    if y == input.len() - 1 {
                        input[y][x] = 'X';
                        println!("{}", calculate_distinct_locations(&input));
                        return;
                    }
                    if input[y + 1][x] == '#' {
                        input[y][x] = '<';
                        continue;
                    }

                    input[y][x] = 'X';
                    input[y + 1][x] = 'v';
                }
                if input[y][x] == '<' {
                    if x == 0 {
                        input[y][x] = 'X';
                        println!("{}", calculate_distinct_locations(&input));
                        return;
                    }
                    if input[y][x - 1] == '#' {
                        input[y][x] = '^';
                        continue;
                    }

                    input[y][x] = 'X';
                    input[y][x - 1] = '<';
                }
            }
        }
    }
}

fn calculate_distinct_locations(input: &Vec<Vec<char>>) -> u32 {
    let mut distinct_locations: u32 = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == 'X' {
                distinct_locations += 1;
            }
        }
    }

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            print!("{}", input[y][x]);
        }
        println!("");
    }
    return distinct_locations;
}
