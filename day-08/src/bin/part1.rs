use std::collections::HashMap;

fn resolve(input: &str) -> usize{
    let lines: Vec<&str> = input.lines().collect();
    let dirs: Vec<char> = lines[0].chars().collect::<Vec<char>>();
    let map: HashMap<&str, Vec<&str>> = lines[2..].iter()
        .map(|line| {
            let parts = line.split(" = ").collect::<Vec<&str>>();
            let key = parts[0];
            let values = parts[1][1..parts[1].len()-1].split(", ")
                .collect::<Vec<&str>>();
            return (key, values);
        })
        .collect::<HashMap<&str, Vec<&str>>>();

    let mut steps = 0;
    let mut current = "AAA";
    let end = "ZZZ";

    while current != end {
        let dir: char = dirs[steps % dirs.len()];
        match dir {
            'L' => current = map.get(current).unwrap()[0],
            'R' => current = map.get(current).unwrap()[1],
            _ => panic!("Unknown direction: {}", dir)
        }
        steps += 1;
    }
    return steps;
}

fn main() {
    let input_file: &str = include_str!("../input.txt");
    let result = resolve(&input_file);
    println!("Value: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = 
"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(resolve(input), 6);
    }
}