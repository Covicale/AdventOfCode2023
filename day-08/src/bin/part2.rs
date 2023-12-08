use num::integer::lcm;
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

    let mut cycle_reached: HashMap<&str, usize> = HashMap::new();
    let mut steps = 0;
    let mut current: Vec<&str> = map.keys()
        .filter(|key| key.ends_with("A"))
        .map(|val| *val).collect();

    while cycle_reached.len() != current.len() {
        for i in 0..current.len() {
            let dir: char = dirs[steps % dirs.len()];
            match dir {
                'L' => current[i] = &map.get(current[i]).unwrap()[0],
                'R' => current[i] = &map.get(current[i]).unwrap()[1],
                _ => panic!("Unknown direction: {}", dir)
            }
            if current[i].ends_with("Z") && !cycle_reached.contains_key(current[i]) {
                cycle_reached.insert(current[i], steps+1);
            }
        }
        steps += 1;
    }
    return cycle_reached.into_iter()
        .fold(1, |acc, (_, val)| lcm(acc, val));
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
"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(resolve(input), 6);
    }
}