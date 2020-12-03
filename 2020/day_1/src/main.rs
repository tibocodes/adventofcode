use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut elements = [0; 1000];

    let mut index = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if let Ok(num) = ip.parse() {
                    elements[index] = num;
                }
                // println!("{}", ip);
                index = index + 1;
            }
        }
    }
    for elem in elements.iter() {
        println!("{}", elem);
    }

    // Exo 1
    // for i in elements.iter() {
    //     for j in elements.iter() {
    //         if i + j == 2020 {
    //             println!("{} : {}",i,j);
    //             println!("{}",i*j);
    //             return;
    //         }
    //     }
    // }

    // Exo 2
    for i in elements.iter() {
        for j in elements.iter() {
            for k in elements.iter() {
                if i + j + k == 2020 {
                    println!("{} : {} : {}",i,j,k);
                    println!("{}",i*j*k);
                    return;
                }
            }
        }
    }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
