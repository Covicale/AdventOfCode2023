#[derive(Debug)]
struct Race {
    duration: usize,
    record_distance: usize,
}

impl Race {
    fn ways_to_win(&self) -> usize {
        let mut ways: usize = 0;
        for hold_time in 1..self.duration {
            let distance: usize = hold_time * (self.duration - hold_time);
            if distance > self.record_distance {
                ways += 1;
            } 
        }
        return ways;
    }
}

fn part_two(input: &str) -> usize {
    let mut race: Race = Race{duration: 0, record_distance: 0};
    let input: Vec<Vec<String>> = input.lines().map(|x| x.split_whitespace().map(String::from).collect()).collect();
    race.duration = input[0][1..].join("").parse().unwrap();
    race.record_distance = input[1][1..].join("").parse().unwrap();

    println!("{:?}", race);
    return race.ways_to_win();
}

fn main() {
    let input: &str = include_str!("../input.txt");
    let result = part_two(&input);
    println!("Value: {}", result);
}