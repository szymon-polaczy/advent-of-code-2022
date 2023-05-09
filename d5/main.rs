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

        let mut lines = read_lines(file_name.to_string());
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

    println!("{:?}", input_map);

    let mut input_map: Vec<char> = Vec::new();
  
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap(); 
    return io::BufReader::new(file).lines(); 
}

