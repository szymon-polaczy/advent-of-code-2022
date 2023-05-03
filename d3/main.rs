use std::fs::File;
use std::io::{ self, BufRead, BufReader };

fn main() {
     let file_name = "./input.txt";

    let lines = read_lines(file_name.to_string());

    let mut score = 0;

    for line in lines {
        let text: String = line.unwrap();
        let text_length = text.chars().count();
        
        let first_part: String = (&text[0..(text_length/2)]).to_string();
        let second_part: String = (&text[(text_length/2)..text_length]).to_string();

        for character in first_part.chars(){
            if second_part.contains( character ) {
                score += get_character_score(character);
                break;
            }
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

