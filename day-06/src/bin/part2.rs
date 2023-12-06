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

fn part_two(input: &str) -> usize {
    let mut race: Race = Race{duration: 0, record_distance: 0};
    let input: Vec<Vec<String>> = input.lines().map(|x| x.split_whitespace().map(String::from).collect()).collect();
    race.duration = input[0][1..].join("").parse().unwrap();
    race.record_distance = input[1][1..].join("").parse().unwrap();
    return race.ways_to_win();
}

fn main() {
    let input: &str = include_str!("../input.txt");
    let result = part_two(&input);
    println!("Value: {}", result);
}