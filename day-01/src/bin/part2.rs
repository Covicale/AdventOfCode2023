use std::collections::HashMap;
use regex::Regex;

fn get_words_pos(numbers_map: &HashMap<&str, usize>, input: &str) -> HashMap<usize, [usize; 2]> {
    let mut words_pos: HashMap<usize, [usize; 2]> = HashMap::new();

    for (key, val) in numbers_map {
        let re = Regex::new(&format!(r"{}|{}", key, val)).unwrap();
        let mut matches = re.find_iter(input);
        if let Some(first_match) = matches.next() {
            let first = first_match.start();
            let mut last = first_match.end();
            for mat in matches {
                last = mat.end();
            }
            words_pos.insert(*val, [first as usize, last as usize]);
        }
    }
    return words_pos;
}

fn get_first_and_last_digit(words_pos: &HashMap<usize, [usize; 2]>) -> [char; 2] {
    let first_val: usize = words_pos
        .into_iter()
        .min_by(|a, b| a.1[0].cmp(&b.1[0]))
        .unwrap()
        .0.clone();

    let last_val: usize = words_pos
        .into_iter()
        .max_by(|a, b| a.1[1].cmp(&b.1[1]))
        .unwrap()
        .0.clone();

    return [
        char::from_digit(first_val as u32, 10).unwrap(),
        char::from_digit(last_val as u32, 10).unwrap()
    ];
}

fn main() {
    let numbers_map: HashMap<&str, usize> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let input_file: &str = include_str!("../input.txt");
    let input: Vec<&str> = input_file.lines().collect();
    let mut result: usize = 0;
    for instruction in input {
        let words_pos: HashMap<usize, [usize; 2]> = get_words_pos(&numbers_map, &instruction);
        let [first, last] = get_first_and_last_digit(&words_pos);
        let mut number = String::from(first);
        number.push(last);
        let value = number.parse::<usize>().expect(&format!("err: {}", &number)); 
        result += value;
    }
    println!("Value: {}", result);
}