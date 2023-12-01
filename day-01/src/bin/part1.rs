fn get_first_and_last_digit(input: &str) -> [char; 2] {
    let only_numbers_input: Vec<char> = input
        .chars()
        .filter(|val|val.is_digit(10))
        .collect();

    let first_val: char = only_numbers_input[0];
    let last_val: char = only_numbers_input[only_numbers_input.len() - 1];
    return [first_val, last_val];
}

fn main() {
    let input_file: &str = include_str!("../input.txt");
    let input: Vec<&str> = input_file.lines().collect();
    let mut result: u32 = 0;
    for instruction in input {
        let [first, last] = get_first_and_last_digit(&instruction);
        let mut number = String::from(first);
        number.push(last);
        let value = number.parse::<u32>().unwrap(); 
        result += value;
    }
    println!("Value: {}", result);
}