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