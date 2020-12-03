use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    //advent_1();
    //advent_2();
    //advent_3();
    //advent_4();
    advent_5();
    advent_6();
}

fn advent_5() {
    let file = File::open("advent_3").unwrap();
    let mut vec: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        vec.push(line.unwrap());
    }
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    while y < vec.len() {
        if vec[y].chars().nth(x%vec[0].len()).unwrap() == '#' {
            count += 1;
        }
        x += 3;
        y += 1;
    }
    println!("{}", count);
}

fn advent_6() {
    let file = File::open("advent_3").unwrap();
    let mut vec: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        vec.push(line.unwrap());
    }
    let mut finalAns = 1;
    for (xDel,yDel) in [(1,1), (3,1), (5,1), (7,1), (1, 2)].iter() {
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut count: u128 = 0;
        while y < vec.len() {
            if vec[y].chars().nth(x%vec[0].len()).unwrap() == '#' {
                count += 1;
            }
            x += xDel;
            y += yDel;
        }
        finalAns *= count;
    }
    println!("{}", finalAns);
}

fn advent_1() {
    let file = File::open("advent_1").unwrap();
    let mut vec: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        vec.push(line.unwrap());
    }
    let v: Vec<u32> = vec.iter().map(|x| x.parse::<u32>().unwrap()).collect();

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
    let v: Vec<u32> = vec.iter().map(|x| x.parse::<u32>().unwrap()).collect();

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

fn advent_3() {
    let file = File::open("advent_2").unwrap();
    let mut vec: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        vec.push(line.unwrap());
    }
    let mut correct_passwords = 0;
    for password_str in vec {
        let mut iter = password_str.split_whitespace();
        let numbers: String = iter.next().unwrap().to_string();
        let letter: char = iter.next().unwrap().chars().next().unwrap();
        let password: String = iter.next().unwrap().to_string();
        let v: Vec<&str> = numbers.split('-').collect();
        let first: u32 = v[0].parse::<u32>().unwrap();
        let second: u32 = v[1].parse::<u32>().unwrap();


        let mut count = 0;
        for c in password.chars() {
            if c == letter {
                count = count + 1;
            }
        }
        if count >= first && count <= second {
            correct_passwords += 1;
        }
    }
    println!("{}", correct_passwords);
}

fn advent_4() {
    let file = File::open("advent_2").unwrap();
    let mut vec: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        vec.push(line.unwrap());
    }
    let mut correct_passwords = 0;
    for password_str in vec {
        let mut iter = password_str.split_whitespace();
        let numbers: String = iter.next().unwrap().to_string();
        let letter: char = iter.next().unwrap().chars().next().unwrap();
        let password: String = iter.next().unwrap().to_string();
        let v: Vec<&str> = numbers.split('-').collect();
        let first: usize = v[0].parse::<usize>().unwrap();
        let second: usize = v[1].parse::<usize>().unwrap();
        let mut found_once = false;
        let mut found_twice = false;
        for (i, c) in password.chars().enumerate() {
            if (i+1) == first {
                if c == letter {
                    found_once = true;
                }
            }
            else if (i+1) == second {
                if found_once {
                    if c == letter {
                        found_twice = true;
                    }
                } else {
                    if c == letter {
                        found_once = true;
                    }
                }
            }
        }
        if found_once && !found_twice {
            correct_passwords += 1;
        }
    }
    println!("{}", correct_passwords);
}
