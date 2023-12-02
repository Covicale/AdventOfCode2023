#[derive(Debug)]
struct NumberCubes(usize, usize, usize);
#[derive(Debug)]
struct Game {
    id: usize,
    cubes_revealed: NumberCubes
}

fn get_cubes_revealed(bags: &[String]) -> NumberCubes {
    let mut num_cubes: NumberCubes = NumberCubes(0, 0, 0);
    for bag in bags {

    }
    return num_cubes;
}

fn part_one(games: &Vec<&str>, max_cubes: &NumberCubes) -> usize {
    let all_games: Vec<_> = games
        .into_iter()
        .map(|game| {
            let mut game_object = Game { id: 0, cubes_revealed: NumberCubes(0, 0, 0)};
            let game = game.replace(":", ";")
                .split(";")
                .map(|x| x.trim().to_string())
                .collect::<Vec<String>>();

            return game_object;
        })
        .collect();

    println!("{:?}", all_games);
    return 1;
}

fn main() {
    let input_file: &str = include_str!("../input.txt");
    let input: Vec<&str> = input_file.lines().collect();

    //12 red cubes, 13 green cubes, and 14 blue cubes
    let max_cubes = NumberCubes(12, 13, 14);
    println!("{}", part_one(&input, &max_cubes));

}