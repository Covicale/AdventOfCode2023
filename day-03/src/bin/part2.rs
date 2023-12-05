use std::collections::HashMap;

#[derive(Debug)]
struct Position(usize, usize);

fn create_range(start: usize, end: usize) -> std::ops::RangeInclusive<usize> {
    if start == 0 {
        return start..=end + 1;
    } else {
        return (start - 1)..=end + 1;
    }
}

fn is_adyacent(number_pos: &(Position, Position), gear_pos: &Position) -> bool {
    let (start_pos, end_pos) = number_pos;
    let x_range = create_range(start_pos.0, end_pos.0);
    let y_range = create_range(start_pos.1, end_pos.1);
    if x_range.contains(&gear_pos.0) && y_range.contains(&gear_pos.1) {
        return true;
    }
    return false;
}

fn part_one(input: &Vec<&str>) -> usize {
    let mut symbols_positions: Vec<Position> = Vec::new();
    let mut numbers_positions: HashMap<usize, Vec<(Position, Position)>> = HashMap::new();
    for (row, line) in input.into_iter().enumerate() {
        let mut col: usize = 0;
        let chars: Vec<char> = line.chars().collect();
        while col < line.len() {
            if chars[col] == '*' {
                symbols_positions.push(Position(row, col));
            } else if chars[col].is_numeric() {
                let end_col = chars[col..].iter().position(|&c| !c.is_numeric()).unwrap_or(line.len() - col);
                let end_col = col + end_col - 1;
                let number: usize = chars[col..end_col + 1].iter().collect::<String>().parse::<usize>().unwrap();
                numbers_positions.entry(number).or_insert(Vec::new()).push((Position(row, col), Position(row, end_col)));
                col = end_col;
            }
            col += 1;
        }
    }
    let mut result = 0;
    for gear_pos in symbols_positions {
        let mut gears_numbers: Vec<usize> = Vec::new();
        for (number, positions) in &numbers_positions {
            for position in positions {
                if is_adyacent(&position, &gear_pos) {
                    gears_numbers.push(*number);
                }
            } 
        }

        if gears_numbers.len() >= 2 {
            result += gears_numbers.iter().product::<usize>();
        }
    }
    return result;
}

fn main() {
    let input_file: &str = include_str!("../input.txt");
    let input: Vec<&str> = input_file.lines().collect();
    let result = part_one(&input);
    println!("Value: {}", result);
}