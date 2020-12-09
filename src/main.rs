use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

use petgraph::*;
use petgraph::visit::Dfs;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;

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
    //advent_13();
    //advent_14();
    //advent_15();
    //advent_16();
    //advent_17();
    advent_18();
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
    let mut final_ans = 1;
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
        final_ans *= count;
    }
    println!("{}", final_ans);
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

fn advent_13() {
    let file_string: String = fs::read_to_string("advent_7").unwrap();
    let lines = file_string.split("\n").collect::<Vec<&str>>();
    let mut g = Graph::<String, u32>::new();
    let mut bags: HashSet<String> = HashSet::<>::new();
    for line in &lines {
        let line_str = line.to_string();
        let words: Vec<&str> = line_str.split("contain").collect::<Vec<&str>>();
        let bigger: String = words[0].to_string();
        let smaller: String = words[1].to_string();
        let first_split = bigger.split(" ").collect::<Vec<&str>>();
        let bigger_keyword_1 = first_split[0].to_string();
        let bigger_keyword_2 = first_split[1].to_string();
        let bigger_together = format!("{} {}", bigger_keyword_1, bigger_keyword_2);
        bags.insert(bigger_together);
        let each_smaller = smaller.split(",").collect::<Vec<&str>>();
        for contain in each_smaller {
            let destination = contain.split(" ").collect::<Vec<&str>>();
            if destination[0] != "no" {
                let keyword_1 = destination[1];
                let keyword_2 = destination[2];
                let together = format!("{} {}", keyword_1, keyword_2);
                bags.insert(together);
            }
        }
    }
    let mut map = HashMap::new();
    for bag in bags {
        let index = g.add_node(bag.clone());
        map.insert(bag, index);
    }
    for line in &lines {
        let line_str = line.to_string();
        let words: Vec<&str> = line_str.split("contain").collect::<Vec<&str>>();
        let bigger: String = words[0].to_string();
        let smaller: String = words[1].to_string();
        let first_split = bigger.split(" ").collect::<Vec<&str>>();
        let bigger_keyword_1 = first_split[0].to_string();
        let bigger_keyword_2 = first_split[1].to_string();
        let bigger_together = format!("{} {}", bigger_keyword_1, bigger_keyword_2);
        let each_smaller: Vec<String> = smaller.trim().split(",").collect::<Vec<&str>>()
            .iter().map(|x| x.to_string().trim().to_string()).collect::<Vec<String>>();
        for contain in each_smaller {
            let destination = contain.split(" ").collect::<Vec<&str>>();
            if destination[0] != "no" {
                let number: u32 = destination[0].parse::<u32>().unwrap();
                let keyword_1 = destination[1];
                let keyword_2 = destination[2];
                let together = format!("{} {}", keyword_1, keyword_2);
                g.update_edge(*map.get(&bigger_together).unwrap(), *map.get(&together).unwrap(), number);
            }
        }
    }
    let shiny_gold = map.get("shiny gold").unwrap();
    let mut final_ans = 0;
    for start in g.node_indices() {
        let mut dfs = Dfs::new(&g, start);
        let mut can_contain: HashSet<usize> = HashSet::new();
        while let Some(visited) = dfs.next(&g) {
            can_contain.insert(visited.index());
        }
        if can_contain.contains(&shiny_gold.index()) && start.index() != shiny_gold.index() {
            final_ans += 1;
        }
    }
    println!("{}", final_ans);
}

fn advent_14() {
    let file_string: String = fs::read_to_string("advent_7").unwrap();
    let lines = file_string.split("\n").collect::<Vec<&str>>();
    let mut g = Graph::<String, u32>::new();
    let mut bags: HashSet<String> = HashSet::<>::new();
    for line in &lines {
        let line_str = line.to_string();
        let words: Vec<&str> = line_str.split("contain").collect::<Vec<&str>>();
        let bigger: String = words[0].to_string();
        let smaller: String = words[1].to_string();
        let first_split = bigger.split(" ").collect::<Vec<&str>>();
        let bigger_keyword_1 = first_split[0].to_string();
        let bigger_keyword_2 = first_split[1].to_string();
        let bigger_together = format!("{} {}", bigger_keyword_1, bigger_keyword_2);
        bags.insert(bigger_together);
        let each_smaller = smaller.split(",").collect::<Vec<&str>>();
        for contain in each_smaller {
            let destination = contain.split(" ").collect::<Vec<&str>>();
            if destination[0] != "no" {
                let keyword_1 = destination[1];
                let keyword_2 = destination[2];
                let together = format!("{} {}", keyword_1, keyword_2);
                bags.insert(together);
            }
        }
    }
    let mut map = HashMap::new();
    for bag in bags {
        let index = g.add_node(bag.clone());
        map.insert(bag, index);
    }
    for line in &lines {
        let line_str = line.to_string();
        let words: Vec<&str> = line_str.split("contain").collect::<Vec<&str>>();
        let bigger: String = words[0].to_string();
        let smaller: String = words[1].to_string();
        let first_split = bigger.split(" ").collect::<Vec<&str>>();
        let bigger_keyword_1 = first_split[0].to_string();
        let bigger_keyword_2 = first_split[1].to_string();
        let bigger_together = format!("{} {}", bigger_keyword_1, bigger_keyword_2);
        let each_smaller: Vec<String> = smaller.trim().split(",").collect::<Vec<&str>>()
            .iter().map(|x| x.to_string().trim().to_string()).collect::<Vec<String>>();
        for contain in each_smaller {
            let destination = contain.split(" ").collect::<Vec<&str>>();
            if destination[0] != "no" {
                let number: u32 = destination[0].parse::<u32>().unwrap();
                let keyword_1 = destination[1];
                let keyword_2 = destination[2];
                let together = format!("{} {}", keyword_1, keyword_2);
                g.update_edge(*map.get(&bigger_together).unwrap(), *map.get(&together).unwrap(), number);
            }
        }
    }
    let shiny_gold = map.get("shiny gold").unwrap();
    let mut total_num_bags = -1;
    let mut bags_to_unpack: Vec<NodeIndex> = Vec::new();
    bags_to_unpack.push(*shiny_gold);
    while !bags_to_unpack.is_empty() {
        total_num_bags += 1;
        for unpacked_bag in bags_to_unpack.clone() {
            print!("{} ", unpacked_bag.index());
        }
        println!("");
        let current = bags_to_unpack.pop().unwrap();
        let edges = g.edges(current);
        for edge in edges {
            println!("A {} contains {} {}", edge.source().index(), edge.weight(), edge.target().index());
            for num in 0..*edge.weight() {
                bags_to_unpack.push(edge.target());
            }
        }
    }
    let mut dfs = Dfs::new(&g, *shiny_gold);
    let mut can_contain: HashSet<usize> = HashSet::new();
    while let Some(visited) = dfs.next(&g) {
        can_contain.insert(visited.index());
    }
    println!("{}", total_num_bags);
}

fn advent_15() {
    let file_string: String = fs::read_to_string("advent_8").unwrap();
    let lines = file_string.split("\n").collect::<Vec<&str>>();
    let mut current_index: i32 = 0;
    let mut acc = 0;
    let mut map: HashSet<i32> = HashSet::new();
    loop {
        println!("Executing instruction: {}", current_index);
        if map.contains(&current_index.clone()) {
            break;
        }
        map.insert(current_index);
        let instruction = lines[current_index as usize].split(" ").collect::<Vec<&str>>();
        match instruction[0] {
            "acc" => {
                let string: String = instruction[1].to_string();
                acc += match string.chars().next().unwrap() {
                    '+' => instruction[1][1..].to_string().trim().parse::<i32>().unwrap(),
                    '-' => instruction[1].to_string().trim().parse::<i32>().unwrap(),
                    _ => 0
                };
                current_index += 1;
            },
            "nop" => {
                current_index += 1;
            },
            "jmp" => {
                let string: String = instruction[1].to_string();
                current_index += match string.chars().next().unwrap() {
                    '+' => instruction[1][1..].to_string().trim().parse::<i32>().unwrap(),
                    '-' => instruction[1].to_string().trim().parse::<i32>().unwrap(),
                    _ => 0
                };
            },
            _ => {
                println!("What the fuck");
            }
        };
    }
    println!("Final Ans: {}", acc);
}

fn advent_16() {
    let file_string: String = fs::read_to_string("advent_8").unwrap();
    let lines = file_string.split("\n").collect::<Vec<&str>>();
    for i in 0..lines.len() {
        let mut current_index: i32 = 0;
        let mut acc = 0;
        let mut map: HashSet<i32> = HashSet::new();
        println!("{}", i);
        loop {
            if map.contains(&current_index.clone()) {
                acc = -1;
                break;
            }
            map.insert(current_index);
            if current_index == lines.len() as i32 {
                println!("Final Answer: {}", acc);
            }
            let instruction = lines[current_index as usize].split(" ").collect::<Vec<&str>>();
            if current_index != i as i32 {
                match instruction[0] {
                    "acc" => {
                        let string: String = instruction[1].to_string();
                        acc += match string.chars().next().unwrap() {
                            '+' => instruction[1][1..].to_string().trim().parse::<i32>().unwrap(),
                            '-' => instruction[1].to_string().trim().parse::<i32>().unwrap(),
                            _ => 0
                        };
                        current_index += 1;
                    },
                    "nop" => {
                        current_index += 1;
                    },
                    "jmp" => {
                        let string: String = instruction[1].to_string();
                        current_index += match string.chars().next().unwrap() {
                            '+' => instruction[1][1..].to_string().trim().parse::<i32>().unwrap(),
                            '-' => instruction[1].to_string().trim().parse::<i32>().unwrap(),
                            _ => 0
                        };
                    },
                    _ => {
                        println!("What the fuck");
                    }
                };
            } else {
                println!("Swap on instruction {}", i);
                match instruction[0] {
                    "acc" => {
                        println!("Acc found on instruction {}", i);
                        acc = -1;
                        break;
                    },
                    "jmp" => {
                        current_index += 1;
                    },
                    "nop" => {
                        let string: String = instruction[1].to_string();
                        current_index += match string.chars().next().unwrap() {
                            '+' => instruction[1][1..].to_string().trim().parse::<i32>().unwrap(),
                            '-' => instruction[1].to_string().trim().parse::<i32>().unwrap(),
                            _ => 0
                        };
                    },
                    _ => {
                        println!("What the fuck");
                    }
                };
            }
        }
        if acc != -1 {
            println!("Final Ans: {}", acc);
        }
    }
}

fn advent_17() {
    let file_string: String = fs::read_to_string("advent_9").unwrap();
    let lines: Vec<u64> = file_string.split("\n").collect::<Vec<&str>>().iter().map(|x| x.to_string().trim().parse::<u64>().unwrap()).collect::<Vec<u64>>();
    for i in 25..lines.len() {
        let preamble = lines[i-25..i].to_vec();
        let num = lines[i];
        let mut can_be_added = false;
        for j in 0..25 {
            for k in j+1..25 {
                if num == (preamble[j] + preamble[k]) {
                    can_be_added = true;
                }
            }
        };
        if !can_be_added {
            println!("{}", i);
            println!("{}", lines[i]);
            break;
        }
    }
}

fn advent_18() {
    let target: u64 = 20874512;
    let file_string: String = fs::read_to_string("advent_9").unwrap();
    let lines: Vec<u64> = file_string.split("\n").collect::<Vec<&str>>().iter().map(|x| x.to_string().trim().parse::<u64>().unwrap()).collect::<Vec<u64>>();
    for i in 0..lines.len() {
        for j in i+1..lines.len() {
            if lines[i..j].to_vec().iter().sum::<u64>() == target {
                println!("{}, {}, {}, {}", i, j-1, lines[i] + lines[j-1], " answer");
            }
        }
    }
}