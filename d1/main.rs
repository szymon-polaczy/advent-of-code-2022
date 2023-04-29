use std::fs::File;
use std::io::{ self, BufRead, BufReader };

fn main() {
    let reinders: Vec<Vec<i32>> = read_input_file();
    
    let mut reinders_summed: Vec<i32> = Vec::new();

    for singular_reinder in &reinders {
        let sum = singular_reinder.iter().sum();
        reinders_summed.push(sum);
    }

    reinders_summed.sort();
    reinders_summed.reverse();

    let top_3 = &reinders_summed[0..3];
    let sum: i32 = top_3.iter().sum();

    println!("{}", sum);
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap(); 
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines(); 
}

fn read_input_file() -> Vec<Vec<i32>> {
    let file_name = "./input.txt";

    // Stores the iterator of lines of the file in lines variable.
    let lines = read_lines(file_name.to_string());

    let mut reinders: Vec<Vec<i32>> = Vec::new();
    let mut singular_reinder: Vec<i32> = Vec::new();

    // Iterate over the lines of the file, and in this case print them.
    for line in lines {
        let text = line.unwrap();
        if text == "" {
            reinders.push(singular_reinder);
            singular_reinder = Vec::new();
        } else { 
            singular_reinder.push(text.parse::<i32>().unwrap());
        }
    }

    return reinders;
}
