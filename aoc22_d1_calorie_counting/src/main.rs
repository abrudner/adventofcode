use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error>{

    // Input data handle
    let path = "src/input.txt";    
    let input_file = File::open(path)?;
    let file_buffer = BufReader::new(input_file);

    let mut running_total = 0;
    let mut third_place = 0;
    let mut second_place = 0;
    let mut highest_total = 0;

    for line in file_buffer.lines() {
        match line?.parse::<i32>() {
            Ok(calories) => running_total += calories,
            Err(_) => {
                match running_total {
                    total if total > highest_total => {
                        third_place = second_place;
                        second_place = highest_total;
                        highest_total = total;
                        running_total = 0;
                    },
                    total if total > second_place => {
                        third_place = second_place;
                        second_place = total;
                        running_total = 0;
                    },
                    total if total > third_place => {
                        third_place = total;
                        running_total = 0;
                    },
                    _ => running_total = 0
                }
            },
        }
    }

    println!("The 3 Elves carrying the highest calories are carrying {} calories in total", highest_total+second_place+third_place);

    Ok(())
}
