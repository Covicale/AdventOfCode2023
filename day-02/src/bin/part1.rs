struct Cubes(usize, usize, usize); // red, green, blue

struct Game {
    id: usize,
    game_valid: bool
}

fn part_one(input: &str, cubes: &Cubes) -> usize {
    let mut games: Vec<Game> = Vec::new();
    for line in input.lines() {
        let mut game = Game {id: 0, game_valid: true};
        let line = line.replace(";", "").replace(":", "").replace(",","" );
        let line_split = line.split(" ").collect::<Vec<&str>>();
        game.id = line_split[1].parse::<usize>().unwrap();
        for pair in line_split[2..].chunks(2) {
            let number = pair[0].parse::<usize>().unwrap();
            let color = pair[1];
            match color {
                "red" => if cubes.0 < number { game.game_valid = false; break; } else {},
                "green" => if cubes.1 < number { game.game_valid = false; break; } else {},
                "blue" => if cubes.2 < number { game.game_valid = false; break; } else {},
                _ => println!("Error")
            }
        }
        games.push(game);
    }

    let mut result = 0;
    for game in games {
        result += if game.game_valid { game.id } else { 0 };
    }
    return result;
}

fn main() {
    let input_file: &str = include_str!("../input.txt");

    let cubes = Cubes(12, 13, 14);
    let result = part_one(&input_file, &cubes);

    println!("Result: {}", result);
}