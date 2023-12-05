#[derive(Debug)]
struct Map {
    destination_range: (usize, usize),
    source_range: (usize, usize)
}

fn create_map(lines: &Vec<&str>, start: usize) -> (Vec<Map>, usize) { // (map, end)
    let range = start + lines[start..].iter().position(|line| line.is_empty()).unwrap_or(lines.len() - start);
    return (parse_input_to_map(&lines[start..range].to_vec()), range);
}

fn parse_input_to_map(input: &Vec<&str>) -> Vec<Map> {
    let mut map_ranges: Vec<Map> = Vec::new();
    for line in input {
        let line_split = line.split_whitespace()
            .map(|val| val.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let range = line_split[2] - 1;
        
        let map: Map = Map {
            destination_range: (line_split[0], line_split[0] + range),
            source_range: (line_split[1], line_split[1] + range)
        };
        map_ranges.push(map);
    }
    return map_ranges;
}

fn parse_seed(line: &str) -> Vec<usize> {
    let seeds = line.split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>()[1..]
        .iter()
        .map(|seed| seed.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    return seeds;
}

fn make_conversion(value: &usize, maps: &Vec<Map>) -> usize {
    for map in maps {
        let range_value = map.source_range.0..=map.source_range.1;
        if range_value.contains(value) {
            let diffrenece = value - map.source_range.0;
            return map.destination_range.0 + diffrenece;
        }
    }
    return *value;
}

fn part_two(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    let seeds: Vec<usize> = parse_seed(&lines[0]);

    // Seed to soil map
    let (seed_to_soil, end): (Vec<Map>, usize) = create_map(&lines, 3);
    let (soil_to_fertilizer, end): (Vec<Map>, usize) = create_map(&lines, end + 2);
    let (fertilizer_to_water, end): (Vec<Map>, usize) = create_map(&lines, end + 2);
    let (water_to_light, end): (Vec<Map>, usize) = create_map(&lines, end + 2);
    let (light_to_temperature, end): (Vec<Map>, usize) = create_map(&lines, end + 2);
    let (temperature_to_humidity, end): (Vec<Map>, usize) = create_map(&lines, end + 2);
    let (humidity_to_location, _): (Vec<Map>, usize) = create_map(&lines, end + 2);

    let mut result = usize::MAX;
    for seed in seeds.chunks(2) {
        for i in seed[0]..seed[0]+seed[1] {
            let soil = make_conversion(&i, &seed_to_soil);
            let fertilizer = make_conversion(&soil, &soil_to_fertilizer);
            let water = make_conversion(&fertilizer, &fertilizer_to_water);
            let light = make_conversion(&water, &water_to_light);
            let temperature = make_conversion(&light, &light_to_temperature);
            let humidity = make_conversion(&temperature, &temperature_to_humidity);
            let location = make_conversion(&humidity, &humidity_to_location);
            if result > location { result = location; }
        }
    }
    return result;
}

fn main() {
    let input_file: &str = include_str!("../input.txt");
    let result = part_two(&input_file);
    println!("Value: {}", result);
}