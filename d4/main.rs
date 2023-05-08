use std::fs::File;
use std::io::{ self, BufRead, BufReader };

fn main() {
    let file_name = "./input.txt";
    let lines = read_lines(file_name.to_string());

    let mut sum: i32 = 0;

    for line in lines {
        let text: String = line.unwrap();

        let elves: Vec<&str> = text.split(',').collect();

        let first_elve: Vec<i32> = elves[0].split('-').map(|x| x.parse::<i32>().unwrap()).collect();
        let second_elve: Vec<i32> = elves[1].split('-').map(|x| x.parse::<i32>().unwrap()).collect();

        if (first_elve[0] <= second_elve[0] && first_elve[1] >= second_elve[1])
            || (second_elve[0] <= first_elve[0] && second_elve[1] >= first_elve[1]) {
            sum += 1;
        }
    }

    println!("{}", sum);
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap(); 
    return io::BufReader::new(file).lines(); 
}

