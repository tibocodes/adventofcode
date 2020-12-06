// #![allow(unused)]
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

// Exo 1
// fn main() -> std::io::Result<()> {
   
//     let f = File::open("input")?;
//     let f = BufReader::new(f);
//     let mut letters = init_empty_dic();
//     let mut total = 0;
    
//     for line in f.lines() {
//         let line = line.unwrap();
//         if line.len() == 0 {
//             total += count_letters(letters);
//             letters = init_empty_dic();
//             continue;
//         }
//         // add_letters(&letters, line);
//         for c in line.chars() {
//             letters.insert(c as char, true);
//         }
//     }
//     println!("{}", total);

//     Ok(())
// }

// // Couldnt figure out how to make this one work
// // Kept getting
// // = note: expected type `&mut std::collections::HashMap<char, bool>`
// //         found type `&std::collections::HashMap<char, bool>`
// // fn add_letters(letters: &HashMap<char,bool>, line: String) {
// //     for c in line.chars() {
// //         letters.insert(c as char, true);
// //     }
// // }

// fn init_empty_dic() -> HashMap<char,bool> {
//     let mut map: HashMap<char,bool> = HashMap::new();
//     for c in b'a'..=b'z' {
//         // println!("{}", c as char);
//         map.insert(c as char, false); 
//     }
//     return map;
// }

// fn count_letters(map: HashMap<char,bool>) -> u32 {
//     let mut count = 0;
//     for (_key, val) in map.iter() {
//         if *val == true {
//             count += 1 
//         }
//     }
//     return count;
// }

// Exo 2
fn main() -> std::io::Result<()> {
    let f = File::open("input")?;
    let f = BufReader::new(f);
    let mut letters = init_empty_dic();
    let mut total = 0;
    let mut number_of_people_in_group = 0;
    
    for line in f.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            let group_nb = count_letters(letters, number_of_people_in_group);
            println!("count : {}", group_nb);
            total += group_nb;
            letters = init_empty_dic();
            number_of_people_in_group = 0;
            continue;
        }
        // add_letters(&letters, line);
        for c in line.chars() {
            let current_letter_count = letters.get(&(c as char));
            letters.insert(c as char, current_letter_count.unwrap() + 1);
        }
        number_of_people_in_group += 1;
    }
    println!("{}", total);

    Ok(())
}

fn init_empty_dic() -> HashMap<char,u32> {
    let mut map: HashMap<char,u32> = HashMap::new();
    for c in b'a'..=b'z' {
        map.insert(c as char, 0); 
    }
    return map;
}

// Count number of letters which count equals the number of ppl in the group
fn count_letters(map: HashMap<char,u32>, number_of_people_in_group: u32) -> u32 {
    let mut count = 0;
    for (_key, val) in map.iter() {
        if *val == number_of_people_in_group {
            count += 1 
        }
    }
    return count;
}