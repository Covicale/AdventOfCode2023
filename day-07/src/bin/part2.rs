use std::collections::HashMap;
use std::cmp::Ordering;

const GAME_CARDS: [&str; 13] = ["A", "K", "Q", "T", "9", "8", "7", "6", "5", 
                                "4", "3", "2", "J"];

#[derive(PartialOrd, PartialEq, Eq, Ord)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard
}
struct Hand {
    cards: Vec<String>,
    bid: usize,
    hand_type: HandType
}

fn sort_by_rank(a: &Hand, b: &Hand) -> Ordering {
    if a.hand_type == b.hand_type {
        for i in 0..5 {
            if a.cards[i] == b.cards[i] { continue; }
            else {
                let a_pos = GAME_CARDS.into_iter()
                    .position(|card| card == a.cards[i]).unwrap();
                let b_pos = GAME_CARDS.into_iter()
                    .position(|card| card == b.cards[i]).unwrap();
                return a_pos.cmp(&b_pos);
            }
        }
        return Ordering::Equal;
    } else {
        return a.hand_type.cmp(&b.hand_type);
    }
}

fn create_map_of_hands(cards: &Vec<String>) -> HashMap<&String, i32> {
    return cards.into_iter()
        .fold(HashMap::new(), |mut map, card| {
            *map.entry(card).or_insert(0) += 1;
            return map
        });
}

fn get_hand_type(cards: &Vec<String>) -> HandType {
    let map = create_map_of_hands(&cards);
    let mut amounts = Vec::new();
    let mut jokers_count = 0;
    for (card, amount) in map {
        if card == "J" { jokers_count += amount; }
        else { amounts.push(amount); }
    }
    amounts.sort_by(|a, b| b.cmp(a));
    println!("{:?} -> Jokers: {}", amounts, jokers_count);

    if jokers_count >= 5 || amounts[0] + jokers_count >= 5 { 
        return HandType::FiveOfKind;
    } 
    if jokers_count >= 4 || amounts[0] + jokers_count >= 4 { 
        return HandType::FourOfKind;
    } 
    
    // Full house or three of kind
    if jokers_count + amounts[0] >= 3 {
        let remaining_jokers = jokers_count + amounts[0] - 3;
        if amounts.len() >= 2 && amounts[0 + 1] + remaining_jokers >= 2 || remaining_jokers >= 3 {
            return HandType::FullHouse;
        } else {
            return HandType::ThreeOfKind;
        }
    }  

    // Two pairs or one pair
    if jokers_count + amounts[0] >= 2 {
        let remaining_jokers = jokers_count + amounts[0] - 2;
        if amounts.len() >= 2 && amounts[0 + 1] + remaining_jokers >= 2 || remaining_jokers >= 2 {
            return HandType::TwoPair;
        } else {
            return HandType::OnePair;
        }
    }  
    return HandType::HighCard;
}

fn resolve(input: &str) -> usize{
    let mut hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let line_split: Vec<&str> = line.split_whitespace().collect();
        let cards: Vec<String> = line_split[0].split("").map(String::from).collect();
        let hand = Hand {
            cards: cards[1..cards.len() - 1].to_vec(),
            bid: line_split[1].parse::<usize>().unwrap(),
            hand_type: get_hand_type(&cards[1..cards.len() - 1].to_vec())
        };
        hands.push(hand);
    }

    hands.sort_by(|a, b| sort_by_rank(b, a));
    return hands.into_iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1))
        .sum();
}

fn main() {
    let input_file: &str = include_str!("../input.txt");
    let result = resolve(&input_file);
    println!("Value: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = 
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(resolve(input), 5905);
    }
}