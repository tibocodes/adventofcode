// #![allow(unused)]
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

// Exo 1
// fn main() -> std::io::Result<()> {
//     let f = File::open("input")?;
//     let f = BufReader::new(f);
//     let mut max = 0;
//     for line in f.lines() {
//         let id = get_id(line.unwrap());
//         if id > max {
//             max = id;
//         }
//         println!("{}", id);
//     }
//     println!("{}", max);

//     Ok(())
// }


// fn get_id(pass: String) -> u32 {
//     let mut row_min: u32 = 0;
//     let mut row_max: u32 = 127;
//     let mut col_min: u32 = 0;
//     let mut col_max: u32 = 7;
//     for (i, c) in pass.chars().enumerate() {
//         // println!("{}:{}", c, i);
//         match c {
//             // Adding +1 because the number of value is max num + 1 (128 values, max is 127)
//             'F' => row_max = row_max - (row_max - row_min + 1) / 2,
//             'B' => row_min = row_min + (row_max - row_min + 1) / 2,
//             'L' => col_max = col_max - (col_max - col_min + 1) / 2,
//             'R' => col_min = col_min + (col_max - col_min + 1) / 2,
//             _ => continue,
//         }
//     }
//     // println!("row max: {}", row_max);
//     // println!("row min: {}", row_min);
//     // println!("col max: {}", col_max);
//     // println!("ID: {}", row_max * 8 + col_max);

//     return row_max * 8 + col_max;
// }

// Exo 2
fn main() -> std::io::Result<()> {
    let f = File::open("input")?;
    let f = BufReader::new(f);
    let mut max = 0;
    let mut seats = [[0; 128] ; 8];
    
    for line in f.lines() {
        let (row, col) = get_row_col(line.unwrap());
        // if id > max {
        //     max = id;
        // }
        println!("row: {}", row);
        println!("col: {}", col);
        seats[col][row] = 1;
    }
    println!("{}", max);

    // display the flight
    for col in seats.iter() {
        for row in col.iter() {
            print!("{}", row);
        }
        println!("");
    }

    // Look for the empty seat
    for (icol, col) in seats.iter().enumerate() {
        for (irow, row) in col.iter().enumerate() {
            // This condition comes from looking at the flight map
            if irow >= 8 && irow <= 116 && *row == 0 {
                println!("row: {}", irow);
                println!("col: {}", icol);
                println!("ID: {}", irow * 8 + icol);
            }
        }
    }

    Ok(())
}


fn get_row_col(pass: String) -> (usize, usize) {
    let mut row_min: usize = 0;
    let mut row_max: usize = 127;
    let mut col_min: usize = 0;
    let mut col_max: usize = 7;
    for (i, c) in pass.chars().enumerate() {
        // println!("{}:{}", c, i);
        match c {
            // Adding +1 because the number of value is max num + 1 (128 values, max is 127)
            'F' => row_max = row_max - (row_max - row_min + 1) / 2,
            'B' => row_min = row_min + (row_max - row_min + 1) / 2,
            'L' => col_max = col_max - (col_max - col_min + 1) / 2,
            'R' => col_min = col_min + (col_max - col_min + 1) / 2,
            _ => continue,
        }
    }
    // println!("row max: {}", row_max);
    // println!("row min: {}", row_min);
    // println!("col max: {}", col_max);
    // println!("ID: {}", row_max * 8 + col_max);

    return (row_max, col_max);
}