struct Cubes(usize, usize, usize); // red, green, blue

struct Game {
    id: usize,
    number_cubes: Cubes,
}

fn part_two(input: &str) -> usize {
    let mut games: Vec<Game> = Vec::new();
    for line in input.lines() {
        let mut game = Game {id: 0, number_cubes: Cubes(0, 0, 0)};
        let line = line.replace(";", "").replace(":", "").replace(",","" );
        let line_split = line.split(" ").collect::<Vec<&str>>();
        game.id = line_split[1].parse::<usize>().unwrap();
        for pair in line_split[2..].chunks(2) {
            let number = pair[0].parse::<usize>().unwrap();
            let color = pair[1];
            match color {
                "red" => if game.number_cubes.0 < number { game.number_cubes.0 = number; } else { },
                "green" => if game.number_cubes.1 < number { game.number_cubes.1 = number; } else { },
                "blue" => if game.number_cubes.2 < number { game.number_cubes.2 = number; } else { },
                _ => println!("Error")
            }
        }
        games.push(game);
    }

    let mut result = 0;
    for game in games {
        let cubes_power = game.number_cubes.0 * game.number_cubes.1 * game.number_cubes.2;
        result += cubes_power;
    }
    return result;
}

fn main() {
    let input_file: &str = include_str!("../input.txt");
    let result = part_two(&input_file);

    println!("Result: {}", result);
}