use std::fs::File;
use std::io::{ self, BufRead, BufReader };

fn main() {
     let file_name = "./input.txt";

    let lines = read_lines(file_name.to_string());

    let mut score = 0;

    let mut index = 0;
    let mut line_1: String = String::new();
    let mut line_2: String = String::new();
    for line in lines {
        index += 1;

        println!("{}", index);

        let text: String = line.unwrap();

        if index < 3 {
            if line_1.is_empty() {
                line_1 = text;
                continue;
            }

            line_2 = text;
            continue;
        }
        
        for character in text.chars(){
            if line_1.contains( character ) && line_2.contains(character) {
                score += get_character_score(character);
                break;
            }
        }

        if index == 3 {
            index = 0;
            line_1 = String::new();
            line_2 = String::new();
        }
    }

    println!("{}", score);
}

fn get_character_score(character: char) -> u32 {
    let ascii_values = character as u32;

    if ascii_values >= 97 {
        return ascii_values - 96;
    }

    if ascii_values >= 65 {
        return ascii_values - 38;
    }

    return 1;
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap(); 
    return io::BufReader::new(file).lines(); 
}

