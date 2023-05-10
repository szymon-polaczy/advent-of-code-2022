use std::fs::File;
use std::io::{ self, BufRead, BufReader };
use std::convert::TryFrom;

fn main() {
    let file_name = "./input.txt";
    let lines = read_lines(file_name.to_string());

    let mut number_of_rows = 0;
    let mut number_of_columns = 0;

    for line in lines {
        let text: String = line.unwrap();

        if ! text.contains('[') && ! text.is_empty() {
            let second_to_last_character = text.chars().rev().nth(1).unwrap();
            number_of_columns = second_to_last_character.to_digit(10).unwrap();

            break;
        }

        number_of_rows += 1;
    }

    let mut input_map: Vec<Vec<char>> = Vec::new();

    for column in 0..number_of_columns {
        let mut column_array: Vec<char> = Vec::new();

        let lines = read_lines(file_name.to_string());
        let important_lines = lines.take(number_of_rows);

        for row in important_lines {
            let character_column: usize = usize::try_from(1 + ((column) * 4)).unwrap();

            let text: String = row.unwrap();

            let character = text.chars().nth(character_column).unwrap();

            if character != ' ' {
                column_array.push(character);
            }
        }

        input_map.push(column_array);
    }

    let mut index = 0;
    let lines = read_lines(file_name.to_string());
    for line in lines {
        index += 1;

        if index <= number_of_rows + 2 {
            continue;
        }

        let mut text = line.unwrap();
        text = text.replace("move ", "");
        text = text.replace(" from ", " ");
        text = text.replace(" to ", " ");

        let digits: Vec<usize> = text.split(' ').map(|x| x.parse::<usize>().unwrap()).collect();

        let how_many_to_move: usize = digits[0];       
        let from_which_column: usize = digits[1];
        let place_in_this_column: usize = digits[2];

        for _ in 1..=how_many_to_move {
            let moved_value = input_map[from_which_column - 1].pop().unwrap();
            input_map[place_in_this_column - 1].push(moved_value);
        }
    }

    let mut end_value: String = String::new();

    for column in input_map.iter() {
        end_value.push(column.last().unwrap().clone());
    }

    println!("{}", end_value);
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap(); 
    return io::BufReader::new(file).lines(); 
}

