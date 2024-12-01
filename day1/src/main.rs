use core::panic;
use std::{collections::HashMap, fs::File, io::{self, BufRead, BufReader, Lines}};


fn read_lines() -> io::Result<Lines<BufReader<File>>> {
    let file = File::open("input.txt")?;
    Ok(BufReader::new(file).lines())
}

fn parse_file(lines: Lines<BufReader<File>>) -> (Vec<usize>, Vec<usize>) {
    let mut one = vec![0; 1000];
    let mut two = vec![0; 1000];
    for line in lines.flatten() {
        let mut parts = line.split_whitespace();
        let p1 = parts.next().unwrap();
        let p2 = parts.next().unwrap();
        let num1 = p1.parse::<usize>().unwrap();
        let num2 = p2.parse::<usize>().unwrap();
        one.push(num1);
        two.push(num2);
    }
    (one, two)
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

fn main() {
    if let Ok(lines) = read_lines() {
        let (mut one, mut two) = parse_file(lines);
        one.sort();
        two.sort();
        let result = calculate_difference(&one, &two);
        println!("The difference is {}", result);
        let similarity = calculate_similarity_score(&one, &two);
        println!("The similarity score is {}", similarity);
    }
}
