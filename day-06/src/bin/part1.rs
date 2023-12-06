use roots::Roots;
use roots::find_roots_quadratic;

struct Race {
    duration: usize,
    record_distance: usize,
}

impl Race {
    fn ways_to_win(&self) -> usize {
        // x^2 - Tx + L = 0 -> x = hold_time | T = duration | L = record_distance
        let roots = find_roots_quadratic(1.0, -(self.duration as f64), self.record_distance as f64);
        return match roots {
            Roots::Two([a, b]) => return b as usize - a as usize,
            _ => 0,
        }
    }
}

fn part_one(input: &str) -> usize {
    let mut races: Vec<Race> = Vec::new();
    let input: Vec<Vec<String>> = input.lines().map(|x| x.split_whitespace().map(String::from).collect()).collect();
    for i in 1..input[0].len() {
        let race: Race = Race{
            duration: input[0][i].parse().unwrap(), 
            record_distance: input[1][i].parse().unwrap()
        };
        races.push(race);
    }
    return races.iter().map(|x| x.ways_to_win()).product();
}

fn main() {
    let input: &str = include_str!("../input.txt");
    let result = part_one(&input);
    println!("Value: {}", result);
}