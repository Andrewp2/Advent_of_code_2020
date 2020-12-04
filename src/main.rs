use std::fs::File;
use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    //advent_1();
    //advent_2();
    //advent_3();
    //advent_4();
    //advent_5();
    //advent_6();
    //advent_7();
    advent_8();
}

fn advent_7() {
    let fileString: String = fs::read_to_string("advent_4").unwrap();
    let lines = fileString.split("\n\r\n");
    let mut vec: Vec<String> = Vec::new();
    for line in lines {
        vec.push(line.to_string());
    }
    let mut count = 0;
    for passport in vec {
        let terms: Vec<&str> = passport.split_whitespace().collect();
        let mut necessary_terms = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        for term in terms {
            let string_term: String = term.to_string();
            if string_term.len() > 1 {
                let parts: Vec<&str> = string_term.split(":").collect();
                if necessary_terms.contains(&parts[0]) {
                    let index = necessary_terms.iter().position(|s| s == &parts[0]);
                    necessary_terms.remove(index.unwrap());
                }
            }
        }

        if necessary_terms.len() == 0{
            count += 1;
        }
    }

    println!("{}", count);
}

fn valid(field_type: &&str, value: &&str) -> bool {
    match field_type {
        &"byr" =>  {
            let year = value.len();
            if year != 4  {
                return false;
            }
            let val = value.parse::<i32>();
            if val.is_ok() {
                let num = val.unwrap();
                num >= 1920 && num <= 2002
            } else {
                false
            }
        },
        &"iyr" => {
            let year = value.len();
            if year != 4  {
                return false;
            }
            let val = value.parse::<i32>();
            if val.is_ok() {
                let num = val.unwrap();
                num >= 2010 && num <= 2020
            } else {
                false
            }
        },
        &"eyr" => {
            let year = value.len();
            if year != 4  {
                return false;
            }
            let val = value.parse::<i32>();
            if val.is_ok() {
                let num = val.unwrap();
                num >= 2020 && num <= 2030
            } else {
                false
            }
        },
        &"hgt" => {
            let val = value.chars().take(value.chars().count()-2).collect::<String>().parse::<i32>();
            let unit_type = value.chars().skip(value.chars().count()-2).take(2).collect::<String>();
            if val.is_ok() {
                let num = val.unwrap();
                match &unit_type[..] {
                    "cm" => {
                        num >= 150 && num <= 193
                    },
                    "in" => {
                        num >= 59 && num <= 76
                    },
                    _ => {
                        false
                    }
                }
            } else {
                false
            }
        },
        &"hcl" => {
            if value.len() != 7 {
                return false;
            }
            let mut chars = value.chars();
            let first = chars.next().unwrap();
            if first != '#' {
                return false;
            }
            for _x in 0..6 {
                let char_x = chars.next().unwrap();
                if char_x < '0' || (char_x > '9' && char_x < 'a') || char_x > 'f' {
                    return false;
                }
            }
            return true;
        },
        &"ecl" => {
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(value)
        },
        &"pid" => {
            if value.len() != 9 {
                return false;
            }
            let val = value.parse::<i32>();
            return val.is_ok();
        },
        &"cid" => {
            true
        }
        _ => false
    }
}

fn advent_8() {
    let fileString: String = fs::read_to_string("advent_4").unwrap();
    let lines = fileString.split("\n\r\n");
    let mut vec: Vec<String> = Vec::new();
    for line in lines {
        vec.push(line.to_string());
    }
    let mut count = 0;
    for passport in vec {
        println!("#######################");
        println!("Trying passport:\n{}\n", passport);
        let terms: Vec<&str> = passport.split_whitespace().collect();
        let mut necessary_terms = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let mut valid_data = true;
        for term in terms {
            let string_term: String = term.to_string();
            if string_term.len() > 1 {
                let parts: Vec<&str> = string_term.split(":").collect();
                if necessary_terms.contains(&parts[0]) {
                    let index = necessary_terms.iter().position(|s| s == &parts[0]);
                    necessary_terms.remove(index.unwrap());
                    if !valid(&parts[0], &parts[1]) {
                        println!("Field Marked as invalid: {}", string_term);
                        valid_data = false;
                        continue;
                    }
                }
            } 
        }

        if necessary_terms.len() == 0 && valid_data {
            count += 1;
            println!("\nValid passport:\n{}\n{}{}\n", passport, "Count: ", count);
            println!("#######################");
        }
    }

    println!("{}", count);
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
