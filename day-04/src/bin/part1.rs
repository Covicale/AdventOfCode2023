struct Card {
    id: usize,
    winners: usize, // (number, number of time appeared)
}

fn part_one(input: &str) -> usize {
    let mut cards: Vec<Card> = Vec::new();
    for line in input.lines() {
        let mut card = Card { id: 0, winners: 0 };
        let game_split: Vec<&str> = line.split_whitespace().collect();
        card.id = game_split[1].trim_end_matches(':').parse::<usize>().unwrap();
        
        let split_index = game_split.iter().position(|&c| c =="|").unwrap();
        let winner_numbers = &game_split[2..split_index];
        for number in &game_split[split_index+1..game_split.len()] {
            if winner_numbers.contains(number) {
                card.winners += 1;
            }
        }
        cards.push(card);
    }

    let mut result = 0;
    for card in cards {
        if card.winners == 0 {
            continue;
        }
        let card_points = usize::pow(2, (card.winners - 1) as u32);
        result += card_points;
    }
    return result;
}

fn main() {
    let input_file: &str = include_str!("../input.txt");
    let result = part_one(&input_file);
    println!("Value: {}", result);
}