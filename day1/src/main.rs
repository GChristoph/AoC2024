use core::panic;
use std::{collections::HashMap, fs::File, io::{self, BufRead, BufReader, Lines}};

struct Report {
    levels: Vec<u8>
}

fn read_lines() -> io::Result<Lines<BufReader<File>>> {
    let file = File::open("input.txt")?;
    Ok(BufReader::new(file).lines())
}

fn parse_file(lines: Lines<BufReader<File>>) -> Vec<Report> {
    let reports = Vec::<Report>::new();
    for line in lines.flatten() {
        let parts = line.split_whitespace();
        let mut levels = Vec::<u8>::new();
        for level in parts {
            let num = level.parse::<u8>().unwrap();
            levels.push(num);
        }
    }
    reports
}

fn calculate_difference(one: &Vec<usize>, two: &Vec<usize>) -> usize {
    if one.len() != two.len() {
        panic!("Different sized vecs");
    }

    let mut diff = 0;
    for i in 0..one.len() {
        diff += one[i].abs_diff(two[i]);
    }

    diff
}

fn calculate_similarity_score(one: &Vec<usize>, two: &Vec<usize>) -> usize {
    let mut score = 0;
    let mut frequency_map = HashMap::<usize, usize>::new();

    for i in 0..two.len() {
        if let Some(entry) = frequency_map.get_mut(&two[i]) {
            *entry += two[i];
            // println!("found something");
        } else {
            frequency_map.insert(two[i], two[i]);
            // println!("insert {}", two[i]);
        }
    }
    
    for i in 0..one.len() {
        if let Some(value) = frequency_map.get(&one[i]) {
            score += value;
            //println!("found {}", value);
        }
    }

    score
}

fn count_safe_reports(reports: &Vec<Report>) -> usize {
    0
}

fn count_safe_with_dampener(reports: &Vec<Report>) -> usize {
    0
}

fn main() {
    if let Ok(lines) = read_lines() {
        let mut reports = parse_file(lines);
        let num_safe_reports = count_safe_reports(&reports);
        println!("Number of safe reports: {}", num_safe_reports);
        let num_safe_with_dampener = count_safe_with_dampener(&reports);
        println!("Number of safe reports with problem dampener: {}", num_safe_with_dampener);
    }
}
