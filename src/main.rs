use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    advent_1();
    advent_2();
}

fn advent_1() {
    let file = File::open("advent_1").unwrap();
    let mut vec: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        vec.push(line.unwrap());
    }

    let v : Vec<u32> = vec.iter().map(|x| x.parse::<u32>().unwrap()).collect();

    for i in &v {
        for j in &v {
            if i + j == 2020 {
                println!("{}, {}", i, j);
                println!("{}", i * j);
            }
        }
    }
}

fn advent_2() {
    let file = File::open("advent_1").unwrap();
    let mut vec: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        vec.push(line.unwrap());
    }

    let v : Vec<u32> = vec.iter().map(|x| x.parse::<u32>().unwrap()).collect();

    for i in &v {
        for j in &v {
            for k in &v {
                if i + j + k == 2020 {
                    println!("{}, {}, {}", i, j, k);
                    println!("{}", i * j * k);
                }
            }
        }
    }
}
