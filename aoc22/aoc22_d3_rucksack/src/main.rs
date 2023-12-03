use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    // Input data handle
    let contents = fs::read_to_string("./src/input.txt").expect("Could not read file.");
    let rucksacks: Vec<&str> = contents.lines().collect();

    let mut priority_score: i32 = 0;

    // Generate hashmap for the alphabet
    let alphabet_string = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut alphabet: HashMap<char, i32> = HashMap::new();

    for (i, c) in alphabet_string.chars().enumerate() {
        let value = i32::try_from(i).ok().unwrap();
        alphabet.insert(c, value+1);
    }

    // Sort the rucksacks and find the common priority items, then add their value.
    for rucksack in rucksacks {
        let mid = rucksack.len() / 2;
        let (compartment1, compartment2) = rucksack.split_at(mid);

        let c1_char_map: HashSet<char> = compartment1.chars().collect();
        let c2_char_map: HashSet<char> = compartment2.chars().collect();

        let common: Vec<char> = c1_char_map.intersection(&c2_char_map).cloned().collect();

        priority_score += alphabet.get(&common[0]).unwrap();
    }

    println!("The priority score total is {}.", priority_score);

    Ok(())
}
