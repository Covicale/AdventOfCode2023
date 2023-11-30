struct Submarine {
    x: u32,
    y: u32,
}

enum Movement {
    UP(u32),
    FORWARD(u32),
    DOWN(u32),
    UNKNOWN
}

fn type_step(step: &Vec<String>) -> Movement {
    println!("{:?}", step);
    let movement: &str = &step[0];
    let units: u32 =  step[1].parse::<u32>().unwrap();

    match movement {
        "up" => Movement::UP(units),
        "down" => Movement::DOWN(units),
        "forward" => Movement::FORWARD(units),
        _ => Movement::UNKNOWN
    }
}

fn get_steps(input_txt: &str) -> Vec<Movement> {
    let mut result: Vec<Movement> = Vec::new();
    for line in input_txt.lines() {
        result.push(type_step(&line.split(' ').map(String::from).collect()))
    }
    return result;
}

fn execute_steps(steps: &Vec<Movement>) -> Submarine{
    let mut submarine: Submarine = Submarine { x: 0, y: 0 };
    for step in steps {
        match step {
            Movement::FORWARD(units) => submarine.x += units,
            Movement::UP(units) => submarine.y -= units,
            Movement::DOWN(units) => submarine.y += units,
            Movement::UNKNOWN => ()
        }
    }
    return submarine;
}

fn main() {
    let input_file: &str = include_str!("../input.txt");
    
    let steps: Vec<Movement> = get_steps(&input_file);
    let submarine: Submarine = execute_steps(&steps);
    println!("Value: {}", submarine.x * submarine.y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let input: &str = 
"forward 5
down 5
forward 8
up 3
down 8
forward 2";

        let result: u32 = 150;
        let steps: Vec<Movement> = get_steps(&input);
        let submarine: Submarine = execute_steps(&steps);
        println!("{}", submarine.x);
        assert_eq!(result, submarine.x * submarine.y);
    }
}