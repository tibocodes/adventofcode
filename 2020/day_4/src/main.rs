// #![allow(unused)]
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;


// Exo 1

struct Card {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
    cid: bool,
}

// Exo 1
fn main() -> std::io::Result<()> {
    let f = File::open("example2")?;
    let f = BufReader::new(f);
    let mut valid = 0;

    let re = Regex::new(r"(\w{3}):(\S+)").unwrap();

    let mut card: HashMap<String, bool> = new_card();
    for line in f.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            if valid_card(card) {
                valid += 1;
            }
            card = new_card();
            continue
        }
        for cap in re.captures_iter(&line) {
            if check_field(cap[1].to_string(), cap[2].to_string()) {
                card.insert((&cap[1]).to_string(), true);
            }
        }
    }
    println!("{}", valid);
    Ok(())
}

fn valid_card(card: HashMap<String, bool>) -> bool {
    let fields = 0;
    for (k, v) in card.iter() {
        if k != "cid" && *v == false {
            return false;
        }
    }
    return true;
}

fn check_field(field_name: String, field_value: String) -> bool {
    // println!("check field");
    match field_name.as_ref() {
        "byr" => {
            // println!("check match byr");
            let re = Regex::new(r"(\d{4})").unwrap();
            if !re.is_match(&field_value) {
                println!("{}", field_value);
                println!("not match");
                return false;
            }
            println!("{}", field_value);
            let val: u32 = field_value.parse().unwrap();
            if val < 1920 || val > 2002 {
                println!("not match");
                return false;
            }
            println!("match");
        },
        "iyr" => {
            // println!("check match byr");
            let re = Regex::new(r"(\d{4})").unwrap();
            if !re.is_match(&field_value) {
                println!("{}", field_value);
                println!("not match");
                return false;
            }
            println!("{}", field_value);
            let val: u32 = field_value.parse().unwrap();
            if val < 2010 || val > 2020 {
                println!("not match");
                return false;
            }
            println!("match");
        },
        "eyr" => {
            // println!("check match byr");
            let re = Regex::new(r"(\d{4})").unwrap();
            if !re.is_match(&field_value) {
                println!("{}", field_value);
                println!("not match");
                return false;
            }
            println!("{}", field_value);
            let val: u32 = field_value.parse().unwrap();
            if val < 2020 || val > 2030 {
                println!("not match");
                return false;
            }
            println!("match");
        },
        "hgt" => {
            // println!("check match byr");
            let re = Regex::new(r"(\d{2,3})(cm|in)").unwrap();
            if !re.is_match(&field_value) {
                println!("{}", field_value);
                println!("not match");
                return false;
            }
            println!("{}", field_value);
            let caps = re.captures(&field_value).unwrap();
            let val: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
            if caps.get(2).unwrap().as_str() == "cm" {
                if val < 150 || val > 193 {
                    println!("not match");
                    return false;
                }
            } else {
                if val < 59 || val > 76 {
                    println!("not match");
                    return false;
                }
            }
            
            println!("match");
        },
        "hcl" => {
            // println!("check match byr");
            let re = Regex::new(r"#[\w\d]{6}").unwrap();
            if !re.is_match(&field_value) {
                println!("{}", field_value);
                println!("not match");
                return false;
            }
            println!("{}", field_value);
            println!("match");
        },
        _ => println!("field not implemented"),
    }
    // println!("match");
    return true;
}

fn new_card() -> HashMap<String, bool> {
    let mut card: HashMap<String, bool> = HashMap::new();
    card.insert("byr".to_string(), false);
    card.insert("iyr".to_string(), false);
    card.insert("eyr".to_string(), false);
    card.insert("hgt".to_string(), false);
    card.insert("hcl".to_string(), false);
    card.insert("ecl".to_string(), false);
    card.insert("pid".to_string(), false);
    card.insert("cid".to_string(), false);
    return card;
}
