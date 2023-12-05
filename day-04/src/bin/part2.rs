struct Card {
    id: usize,
    winners: usize, // (number, number of time appeared)
}

fn part_two(input: &str) -> usize {
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

    let mut deck: Vec<usize> = Vec::new();
    for card in &cards { deck.push(card.id); }

    let mut deck_index = 0;
    while deck_index < deck.len() {
        let card_id = deck[deck_index];
        match cards.get(card_id - 1) {
            Some(card) => {
                let card_winners = &card.winners;
                for i in card_id+1..=card_id+card_winners {
                    deck.push(i);
                }
            },
            None => { }
        }
        deck_index += 1;
    }
    return deck.len();
}

fn main() {
    let input_file: &str = include_str!("../input.txt");
    let result = part_two(&input_file);
    println!("Value: {}", result);
}