use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    // Input data handle
    let contents = fs::read_to_string("./src/input.txt").expect("Could not read file.");
    let list_of_games: Vec<Vec<&str>> = contents
        .lines()
        .map(|line| line.split(" ").collect())
        .collect();

    let mut score: i32 = 0;

    // Part 2 answer
    for game in list_of_games {
        if game.len() != 2 {
            println!("Error: Each game must have exactly 2 choices.");
            continue;
        }

        let p1 = game[0];
        let p2 = game[1];

        match p2 {
            "X" => {
                match p1 {
                    "A" => score += 3,
                    "B" => score += 1,
                    "C" => score += 2,
                    _ => score += 0,
                };
                score += 0;
            }
            "Y" => {
                match p1 {
                    "A" => score += 1,
                    "B" => score += 2,
                    "C" => score += 3,
                    _ => score += 0,
                };
                score += 3;
            }
            "Z" => {
                match p1 {
                    "A" => score += 2,
                    "B" => score += 3,
                    "C" => score += 1,
                    _ => score += 0,
                };
                score += 6;
            }
            _ => score += 0,
        }
    }

    // Part 1 answer
    // for game in list_of_games {
    //     if game.len() != 2 {
    //         println!("Error: Each game must have exactly 2 choices.");
    //         continue;
    //     }

    //     let p1 = game[0];
    //     let p2 = game[1];

    //     match p2 {
    //         "X" => score += 1,
    //         "Y" => score += 2,
    //         "Z" => score += 3,
    //         _ => score += 0,
    //     }

    //     match (p1,p2) {
    //         ("A", "Y") | ("B", "Z") | ("C", "X") => {
    //             score += 6;
    //         },
    //         ("A", "Z") | ("B", "X") | ("C", "Y") => {
    //             score += 0;
    //         },
    //         _ => score += 3,
    //     }
    // }

    println!("The total score is {} points.", score);

    Ok(())
}
