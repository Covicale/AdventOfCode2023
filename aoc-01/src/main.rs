fn get_vec_from_input(input: &String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for line in input.lines() {
        result.push(line.to_string());
    }
    return result;
}

fn elves_calories(input_vec: &Vec<String>) -> Vec<u32> {
    return input_vec.split(|val: &String| val.is_empty())
        .map(|x: &[String]| x.iter().map(|s: &String| s.parse::<u32>().unwrap_or(0)).sum::<u32>())
        .collect();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input: String = std::fs::read_to_string(&args[1])
        .expect("Ha habido un error leyendo el fichero.");

    let input: Vec<String> = get_vec_from_input(&input);
    
    let mut elves_cal: Vec<u32> = elves_calories(&input);

    elves_cal.sort_by(|a,b| b.cmp(a));

    let max_cal: u32 = elves_cal[0];
    let top_three: u32 = elves_cal
        .iter()
        .take(3)
        .sum();  

    println!("Top 1: {:?}", max_cal);
    println!("Top 3: {:?}", top_three);

    // let slice = [10, 40, 33, 20];
    // let iter: Vec<&[i32]> = slice
    //     .split(|num| num % 3 == 0)
    //     .collect();
    // println!("{:?}", iter)
}