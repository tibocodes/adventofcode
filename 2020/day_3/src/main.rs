// #![allow(unused)]
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

// Exo 1
// fn main() -> std::io::Result<()> {
//     let x_slope = 3;
//     let y_slope = 1;
//     let mut map = Vec::new();

//     let f = File::open("input")?;
//     let f = BufReader::new(f);
//     let mut valid = 0;

//     // read the map
//     for line in f.lines() {
//         let line = line.unwrap();
//         // let v: Vec<&str> = line.rsplit(' ').collect();
//         map.push(line)
//     }

//     for line in &map {
//         println!("{}", line);
//     }
//     // Calculate number of copies necessary - width
//     let nb_of_lines = map.len();
//     println!("nb of lines: {}", nb_of_lines);
//     let width = map[0].len();
//     println!("width: {}", width);
//     // Actually i dont need that, i can probably just move back to the left when necessary
//     // let number_of_slopes = nb_of_lines / slope[1];
//     // println!("number_of_slopes: {}", number_of_slopes);
//     // let max_width = number_of_slopes * slope[0];
//     // println!("max_width: {}", max_width);


//     let mut trees = 0;
//     let mut y_pos = 0;
//     let mut x_pos = 0;
//     while y_pos < nb_of_lines - 1 {
//         x_pos = x_pos + x_slope;
//         if x_pos > width - 1 {
//             x_pos = x_pos - width;
//         }
//         y_pos = y_pos + y_slope;
//         println!("{} {}",x_pos, y_pos);
//         if map[y_pos].as_bytes()[x_pos] == b'#' {
//             trees = trees + 1;
//         }
//     }
//     println!("{}", trees);
//     Ok(())
// }

fn main() -> std::io::Result<()> {
    let x_slope = 3;
    let y_slope = 1;
    let mut map = Vec::new();

    let f = File::open("input")?;
    let f = BufReader::new(f);
    let mut valid = 0;

    // read the map
    for line in f.lines() {
        let line = line.unwrap();
        // let v: Vec<&str> = line.rsplit(' ').collect();
        map.push(line)
    }

    for line in &map {
        println!("{}", line);
    }
    // Calculate number of copies necessary - width
    let nb_of_lines = map.len();
    println!("nb of lines: {}", nb_of_lines);
    let width = map[0].len();
    println!("width: {}", width);
    // Actually i dont need that, i can probably just move back to the left when necessary
    // let number_of_slopes = nb_of_lines / slope[1];
    // println!("number_of_slopes: {}", number_of_slopes);
    // let max_width = number_of_slopes * slope[0];
    // println!("max_width: {}", max_width);


    let mut trees = 0;
    let mut y_pos = 0;
    let mut x_pos = 0;
    while y_pos < nb_of_lines - 1 {
        x_pos = x_pos + x_slope;
        if x_pos > width - 1 {
            x_pos = x_pos - width;
        }
        y_pos = y_pos + y_slope;
        println!("{} {}",x_pos, y_pos);
        if map[y_pos].as_bytes()[x_pos] == b'#' {
            trees = trees + 1;
        }
    }
    println!("{}", trees);
    Ok(())
}
