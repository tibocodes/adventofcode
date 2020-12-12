// #![allow(unused)]
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut plan = values();
    let rows = plan.len();
    let cols = plan[0].len();

    for i in 0..rows {
        for j in 0..cols {
            print!("{}", plan[i][j]);
            // print!("{}", adjacent_seats(&plan, i as isize, j as isize));
        }
        println!("");
    }

    println!();

    for i in 0..rows {
        for j in 0..cols {
            // print!("{}", seat);
            // print!("{}", adjacent_seats(&mut plan, i, j));
            print!("{}", visible_seats(&mut plan, i, j));
        }
        println!("");
    }

    let mut step = 0;
    loop {
        let (change, new_plan) = transform(&mut plan);
        step += 1;
        if change == false {
            break;
        }
        plan = new_plan;
        print_plan(&plan);
        println!();
    }
    println!("{}", step);
    println!("{}", occupied_seats(&plan));
}

fn print_plan(plan: &Vec<Vec<char>>) {
    let rows = plan.len();
    let cols = plan[0].len();
    for i in 0..rows {
        for j in 0..cols {
            print!("{}", plan[i][j]);
            // print!("{}", adjacent_seats(&plan, i as isize, j as isize));
        }
        println!("");
    }
}

// If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
// If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
// Otherwise, the seat's state does not change.
// Floor (.) never changes; seats don't move, and nobody sits on the floor.
fn transform(plan: &mut Vec<Vec<char>>) -> (bool, Vec<Vec<char>>) {
    let rows = plan.len();
    let cols = plan[0].len();
    let mut changes = false;
    let mut new_plan = Vec::new();
    for i in 0..rows {
        let mut new_row = Vec::new();
        for j in 0..cols {
        //    if plan[i][j] == 'L' && adjacent_seats(plan, i, j) == 0 {
           if plan[i][j] == 'L' && visible_seats(plan, i, j) == 0 {
               new_row.push('#');
               changes = true;
        //    } else if plan[i][j] == '#' && adjacent_seats(plan, i, j) >= 4 {
           } else if plan[i][j] == '#' && visible_seats(plan, i, j) >= 5 {
               changes = true;
               new_row.push('L');
           } else {
               new_row.push(plan[i][j]);
           }
        }
        new_plan.push(new_row);
    }
    return (changes, new_plan);
}

fn occupied_seats(plan: & Vec<Vec<char>>) -> u32 {
    let rows = plan.len();
    let cols = plan[0].len();
    let mut count = 0;
    for i in 0..rows {
        for j in 0..cols {
            if plan[i][j] == '#' {
                count += 1;
            }
        }
    }
    return count;
}

fn adjacent_seats(plan: &mut Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let mut occupied = 0;
    let row: isize = row as isize;
    let col: isize = col as isize;
    // note: second limit of ranges is _exclusive_
    for i in row - 1..row + 2 {
        for j  in col - 1..col + 2 {
            if i == row && j == col {
                continue;
            }
            if i >= 0 && j >= 0 && i < plan.len() as isize && j < plan[0].len() as isize {
                if plan[i as usize][j as usize] == '#' {
                    occupied += 1;
                }
            }
        }
    }
    return occupied;
}

fn visible_seats(plan: &mut Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let mut visible = 0;
    // let row: isize = row as isize;
    // let col: isize = col as isize;
    let rows = plan.len();
    let cols = plan[0].len();

    // vertical
    // left
    for j in (0..col).rev() {
        if plan[row][j] == 'L' {
            break;
        }
        if plan[row][j] == '#' {
            visible += 1;
            break;
        }

    }
    // right
    for j in col + 1..cols {
        if plan[row][j] == 'L' {
            break;
        }
        if plan[row][j] == '#' {
            visible += 1;
            break;
        }

    }
    // horizontal
    // up
    for i in (0..row).rev() {
        if plan[i][col] == 'L' {
            break;
        }
        if plan[i][col] == '#' {
            visible += 1;
            break;
        }

    }
    // down
    for i in row + 1..rows {
        if plan[i][col] == 'L' {
            break;
        }
        if plan[i][col] == '#' {
            visible += 1;
            break;
        }

    }
    // diagonal top left
    let mut i = row as isize;
    let mut j = col as isize;
    let mut first = true;
    while i >= 0 && i < plan.len() as isize && j >= 0 && j < plan[0].len() as isize {
        if first == true {
            first = false;
            i = i - 1;
            j = j - 1;
            continue;
        }
        if plan[i as usize][j as usize] == 'L' {
            break;
        }
        if plan[i as usize][j as usize] == '#' {
            visible += 1;
            break;
        }
        i = i - 1;
        j = j - 1;
    }
    // diagonal top right
    let mut i = row as isize;
    let mut j = col as isize;
    let mut first = true;
    while i >= 0 && i < plan.len() as isize && j >= 0 && j < plan[0].len() as isize {
        if first == true {
            first = false;
            i = i - 1;
            j = j + 1;
            continue;
        }
        if plan[i as usize][j as usize] == 'L' {
            break;
        }
        if plan[i as usize][j as usize] == '#' {
            visible += 1;
            break;
        }
        i = i - 1;
        j = j + 1;
    }
    // diagonal bot left
    let mut i = row as isize;
    let mut j = col as isize;
    let mut first = true;
    while i >= 0 && i < plan.len() as isize && j >= 0 && j < plan[0].len() as isize {
        if first == true {
            first = false;
            i = i + 1;
            j = j - 1;
            continue;
        }
        if plan[i as usize][j as usize] == 'L' {
            break;
        }
        if plan[i as usize][j as usize] == '#' {
            visible += 1;
            break;
        }
        i = i + 1;
        j = j - 1;
    }
    // diagonal bot right
    let mut i = row as isize;
    let mut j = col as isize;
    let mut first = true;
    while i >= 0 && i < plan.len() as isize && j >= 0 && j < plan[0].len() as isize {
        if first == true {
            first = false;
            i = i + 1;
            j = j + 1;
            continue;
        }
        if plan[i as usize][j as usize] == 'L' {
            break;
        }
        if plan[i as usize][j as usize] == '#' {
            visible += 1;
            break;
        }
        i = i + 1;
        j = j + 1;
    }


    // println!("visible: {}", visible);
    return visible;
}

fn values() -> Vec<Vec<char>> {
    let file = File::open("input").unwrap();
    let buf = BufReader::new(file);
    let mut input = Vec::new();
    for line in buf.lines() {
        let mut row = Vec::new();
        for char in line.unwrap().chars() {
            row.push(char);
        }
        input.push(row);
    }
    return input;
}