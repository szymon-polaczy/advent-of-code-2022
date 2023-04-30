use std::fs::File;
use std::io::{ self, BufRead, BufReader };

fn main() {
    // Lose - 0
    // Draw - 3
    // Win - 6

    // Rock - 1 - X - A
    // Paper - 2 - Y - B
    // Scissors - 3 - Z - C

    let file_name = "./input.txt";

    let lines = read_lines(file_name.to_string());

    let mut score = 0;

    for line in lines {
        let text = line.unwrap();

        let you = text.chars().nth(2).unwrap();
        
        let round_score = round_score(text);
        let hand_score = hand_score(you);

        let sum = round_score + hand_score;

        score += sum;
    }

    println!("{}", score);
}

fn hand_score(hand: char) -> i32 {
    if hand == 'X' { 
        return 1; 
    }

    if hand == 'Y' {
        return 2;
    }

    if hand == 'Z' {
        return 3;
    }

    return 0;
}

fn round_score(line: String) -> i32 {
    if line == "A X" || line == "B Y" || line == "C Z" {
        return 3;
    }

    if line == "A Y" || line == "B Z" || line == "C X" {
        return 6;
    }

    return 0;
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap(); 
    return io::BufReader::new(file).lines(); 
}

