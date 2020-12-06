use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

fn main() {
    //advent_1();
    //advent_2();
    //advent_3();
    //advent_4();
    //advent_5();
    //advent_6();
    //advent_7();
    //advent_8();
    //advent_9();
    //advent_10();
    //advent_11();
    //advent_12();
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
            if (i + 1) == first {
                if c == letter {
                    found_once = true;
                }
            } else if (i + 1) == second {
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
        if vec[y].chars().nth(x % vec[0].len()).unwrap() == '#' {
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
    for (x_del, y_del) in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut count: u128 = 0;
        while y < vec.len() {
            if vec[y].chars().nth(x % vec[0].len()).unwrap() == '#' {
                count += 1;
            }
            x += x_del;
            y += y_del;
        }
        finalAns *= count;
    }
    println!("{}", finalAns);
}

fn advent_7() {
    let file_string: String = fs::read_to_string("advent_4").unwrap();
    let lines = file_string.split("\n\r\n");
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

        if necessary_terms.len() == 0 {
            count += 1;
        }
    }

    println!("{}", count);
}

fn advent_8() {
    let file_string: String = fs::read_to_string("advent_4").unwrap();
    let lines = file_string.split("\n\r\n");
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
                    if !valid_puzzle_8(&parts[0], &parts[1]) {
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

fn valid_puzzle_8(field_type: &&str, value: &&str) -> bool {
    match field_type {
        &"byr" => {
            let year = value.len();
            if year != 4 {
                return false;
            }
            let val = value.parse::<i32>();
            if val.is_ok() {
                let num = val.unwrap();
                num >= 1920 && num <= 2002
            } else {
                false
            }
        }
        &"iyr" => {
            let year = value.len();
            if year != 4 {
                return false;
            }
            let val = value.parse::<i32>();
            if val.is_ok() {
                let num = val.unwrap();
                num >= 2010 && num <= 2020
            } else {
                false
            }
        }
        &"eyr" => {
            let year = value.len();
            if year != 4 {
                return false;
            }
            let val = value.parse::<i32>();
            if val.is_ok() {
                let num = val.unwrap();
                num >= 2020 && num <= 2030
            } else {
                false
            }
        }
        &"hgt" => {
            let val = value
                .chars()
                .take(value.chars().count() - 2)
                .collect::<String>()
                .parse::<i32>();
            let unit_type = value
                .chars()
                .skip(value.chars().count() - 2)
                .take(2)
                .collect::<String>();
            if val.is_ok() {
                let num = val.unwrap();
                match &unit_type[..] {
                    "cm" => num >= 150 && num <= 193,
                    "in" => num >= 59 && num <= 76,
                    _ => false,
                }
            } else {
                false
            }
        }
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
        }
        &"ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(value),
        &"pid" => {
            if value.len() != 9 {
                return false;
            }
            let val = value.parse::<i32>();
            return val.is_ok();
        }
        &"cid" => true,
        _ => false,
    }
}

fn advent_9() {
    println!("Done in java for external reasons");
}

fn advent_10() {
    println!("Done in java for external reasons");
}

fn advent_11() {
    let file_string: String = fs::read_to_string("advent_6").unwrap();
    let lines = file_string.split("\n\r\n");
    let mut vec: Vec<String> = Vec::new();
    for line in lines {
        vec.push(line.to_string());
    }
    let mut b = 0;

    for group in vec {
        println!("{}", group);
        let mut a: HashSet<char> = HashSet::new();
        let mut vec2: Vec<String> = Vec::new();
        let lines2 = group.split("\n");
        for line in lines2 {
            println!("Line: {}", line);
            vec2.push(line.to_string());
        }

        for line in vec2 {
            for c in line.chars() {
                a.insert(c);
            }
        }
        a.remove(&'\n');
        a.remove(&'\r');

        println!("{}", a.len());
        b += a.len();
    }

    println!("{}", b);
}

fn intersection<T: Eq + Hash>(a: HashSet<T>, b: &HashSet<T>) -> HashSet<T> {
    a.into_iter().filter(|e| b.contains(e)).collect()
}

fn advent_12() {
    let file_string: String = fs::read_to_string("advent_6").unwrap();
    let lines = file_string.split("\n\r\n");
    let mut vec: Vec<String> = Vec::new();
    for line in lines {
        vec.push(line.to_string());
    }
    let mut b = 0;

    let mut letters: HashSet<char> = HashSet::new();
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    for letter in alphabet.chars() {
        letters.insert(letter);
    }

    for group in vec {
        println!("{}", group);
        let mut current_letters: HashSet<char> = letters.iter().copied().collect();
        let mut vec2: Vec<String> = Vec::new();
        let lines2 = group.split("\n");
        for line in lines2 {
            println!("Line: {}", line);
            vec2.push(line.to_string());
        }

        for line in vec2 {
            let mut a: HashSet<char> = HashSet::new();
            for c in line.chars() {
                a.insert(c);
            }
            current_letters = intersection(a, &current_letters);
        }
        println!("{}", current_letters.len());
        b += current_letters.len();
    }

    println!("{}", b);
}
