// Day 1 - Calorie counting
//
// The jungle must be too overgrown and difficult to navigate in vehicles or
// access from the air; the Elves' expedition traditionally goes on foot. As
// your boats approach land, the Elves begin taking inventory of their 
// supplies. One important consideration is food - in particular, the number
// of Calories each Elf is carrying (your puzzle input).
//
// The Elves take turns writing down the number of Calories contained by the
// various meals, snacks, rations, etc. that they've brought with them, one 
// item per line. Each Elf separates their own inventory from the previous 
// Elf's inventory (if any) by a blank line.
// 
// For example, suppose the Elves finish writing their items' Calories and end
// up with the following list:
// 
// 1000
// 2000
// 3000
// 
// 4000
// 
// 5000
// 6000
// 
// 7000
// 8000
// 9000
// 
// 10000
// 
// This list represents the Calories of the food carried by five Elves:
// 
//     The first Elf is carrying food with 1000, 2000, and 3000 Calories, a 
//       total of 6000 Calories.
//     The second Elf is carrying one food item with 4000 Calories.
//     The third Elf is carrying food with 5000 and 6000 Calories, a total of 
//       11000 Calories.
//     The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a 
//       total of 24000 Calories.
//     The fifth Elf is carrying one food item with 10000 Calories.
// 
// In case the Elves get hungry and need extra snacks, they need to know which 
// Elf to ask: they'd like to know how many Calories are being carried by the 
// Elf carrying the most Calories. In the example above, this is 24000 
// (carried by the fourth Elf).
// 
// Find the Elf carrying the most Calories. How many total Calories is that Elf
// carrying?

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
