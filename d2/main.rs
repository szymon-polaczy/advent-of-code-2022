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

        let hand = text.chars().nth(0).unwrap();
        
        let round_score = round_score(text);
        let hand_score = hand_score(hand, round_score);

        let sum = round_score + hand_score;

        score += sum;
    }

    println!("{}", score);
}

fn hand_score(hand: char, round_score: i32) -> i32 {
    if round_score == 0 {
        if hand == 'A' {
            return 3;
        } else if hand == 'B' {
            return 1;
        } else {
            return 2;
        }
    } else if round_score == 3 {
        if hand == 'A' {
            return 1;
        } else if hand == 'B' {
            return 2;
        } else {
            return 3;
        }
    } else {
        if hand == 'A' {
            return 2;
        } else if hand == 'B' {
            return 3;
        } else {
            return 1;
        }
    }
}

fn round_score(line: String) -> i32 {
    let ending = line.chars().nth(2).unwrap();

    if ending == 'Y' {
        return 3;
    }

    if ending == 'Z' {
        return 6;
    }

    return 0;
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap(); 
    return io::BufReader::new(file).lines(); 
}

