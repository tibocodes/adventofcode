// #![allow(unused)]
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

// Exo 1
// fn main() {
//     for line in file_to_vec() {
//         println!("{} : {} : {}", line.0, line.1, line.2);
//     }
//     let mut vec = file_to_vec();
//     let mut index: i32 = 0;
//     let mut acc: i32 = 0;
//     loop {
//         if vec[index as usize].2 == true {
//             println!("infinite loop detected !");
//             break;
//         }
//         match vec[index as usize].0.as_ref() {
//             "nop" => {
//                 vec[index as usize].2 = true;
//                 index += 1;
//             },
//             "acc" => {
//                 acc += vec[index as usize].1;
//                 vec[index as usize].2 = true;
//                 index += 1;
//             },
//             "jmp" => {
//                 vec[index as usize].2 = true;
//                 index += vec[index as usize].1;
//             },
//             &_ => {
//                 panic!("error");
//             }
//         }
//     }
//     println!("acc: {}", acc);
// }

// Exo 2
fn main() {
    for line in file_to_vec() {
        println!("{} : {} : {}", line.0, line.1, line.2);
    }
    println!("");
    // let mut vec = file_to_vec();
    // for line in &vec {
    //     println!("{} : {} : {}", line.0, line.1, line.2);
    // }
    let mut index = 0;
    loop {
        index += 1;
        let mut vec = change_nth_instr(index);
        println!("index: {}", index);
        try_game(&mut vec);
    }
}


fn try_game(vec: &mut Vec<(String, i32, bool)>) {
    let mut index: i32 = 0;
    let mut acc: i32 = 0;
    loop {
        if vec[index as usize].2 == true {
            println!("infinite loop detected !");
            println!("acc: {}", acc);
            return;
        }
        match vec[index as usize].0.as_ref() {
            "nop" => {
                vec[index as usize].2 = true;
                index += 1;
            },
            "acc" => {
                acc += vec[index as usize].1;
                vec[index as usize].2 = true;
                index += 1;
            },
            "jmp" => {
                vec[index as usize].2 = true;
                index += vec[index as usize].1;
            },
            &_ => {
                panic!("error");
            }
        }
        println!("acc: {}", acc);
    }
    panic!("found an exit");
}

fn file_to_vec() -> Vec<(String, i32, bool)> {
    let file = File::open("input").unwrap();
    let buf = BufReader::new(file);
    let mut input = Vec::new();
    for line in buf.lines() {
        let line = line.unwrap();
        input.push((line[0..3].to_string(), line[4..].to_string().parse().unwrap(), false));
    }
    return input;
}

fn change_nth_instr(n: usize) -> Vec<(String, i32, bool)> {
    let mut vec = file_to_vec();
    let mut index = 0;
    for (i, line) in vec.iter().enumerate() {
        if line.0 == "nop" || line.0 == "jmp" {
            index += 1;
            if index == n {
                if line.0 == "nop" {
                    vec[i].0 = "jmp".to_string();
                } else {
                    vec[i].0 = "nop".to_string();
                }
                break;
            }
        }
    }
    for line in &vec {
        println!("{} : {} : {}", line.0, line.1, line.2);
    }
    return vec;
}