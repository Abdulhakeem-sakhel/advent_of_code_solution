use std::{fs, cmp::Ordering};
use std::collections::HashMap;
fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("cant't find the file");
    let mut hand_set = get_hand_bid(contents.as_str());
    hand_set.sort_by(|left, right| order_hand(left, right) );
    let total_wining: usize = hand_set.iter().enumerate()
            .map(|(i, set)| (i+1) * set.1).sum();
        println!("total wining = {}", total_wining)
}

#[derive(PartialEq, Debug)]
struct Hand (String, usize);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1
}
fn strength_of_card(card: &char) -> u16 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        '9' => 9, 
        '8' => 8, 
        '7' => 7, 
        '6' => 6, 
        '5' => 5, 
        '4' => 4, 
        '3' => 3, 
        '2' => 2, 
        _ => 0
    }
}
fn get_hand_type(hand: &Hand) -> HandType {
    let mut card_map: HashMap<char, u16> = HashMap::new();
    hand.0.chars()
        .for_each(|ch| {
            let get_count = card_map.get(&ch);
            match get_count {
                None => card_map.insert(ch, 1),
                Some(num) => card_map.insert(ch, num + 1),
            };
        });

    if card_map.contains_key(&'J') {
        let j_count = card_map.remove(&'J').unwrap().clone();

        if j_count == 5 {
            card_map.insert('J', 5);
        } else {
            let mut max_card = 
            card_map.iter()
                .max_by(|a, b| u16::cmp(a.1, b.1))
                .unwrap();
    
            card_map.insert(*max_card.0, max_card.1 + j_count);
        }
    }
    
    if card_map.keys().len() == 1 {
        return  HandType::FiveOfKind;
    } else if card_map.keys().len() == 2 {
        if card_map.values().copied().collect::<Vec<u16>>().contains(&1) {
            return HandType::FourOfKind;
        } else {
            return HandType::FullHouse;
        }
    } else if card_map.keys().len() == 3 {
        if card_map.values().copied().collect::<Vec<u16>>().contains(&3) {
            return HandType::ThreeOfKind;
        } else {
            return HandType::TwoPair;
        }
    } else if card_map.keys().len() == 4 {
        return  HandType::OnePair;
    } else {
        return HandType::HighCard;
    }
}

fn order_hand(left: &Hand, right: &Hand ) -> Ordering {

    if get_hand_type(left) > get_hand_type(right) {
        return Ordering::Greater;
    } else if get_hand_type(left) < get_hand_type(right) {
        return Ordering::Less;
    } else {
        for i in 0..left.0.len() {
            
            let left_card = strength_of_card(&left.0.chars().nth(i).unwrap());
            let right_card = strength_of_card(&right.0.chars().nth(i).unwrap());
            if u16::cmp(&left_card, &right_card) != Ordering::Equal {
                return u16::cmp(&left_card, &right_card);
            }
        }
    }

    return Ordering::Equal;
}

fn get_hand_bid(set_card: &str) -> Vec<Hand> {
    set_card
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line|  {
            let hand = line.split(" ")
                .nth(0)
                .unwrap()
                .to_string();
            let bid: usize = line.split(" ")
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            return Hand(hand, bid);
        })
        .collect()
} 



#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_func() {
    //     let hand: Hand = Hand(String::from("KK677"), 1);
    //     assert_eq!(HandType::TwoPair, get_hand_type(&hand));
    //     let right_hand: Hand = Hand(String::from("KTJJT"), 1);
    //     assert_eq!(HandType::TwoPair, get_hand_type(&hand));
    //     assert_eq!(Ordering::Greater, order_hand(&hand, &right_hand));
        }

    #[test]
    fn test() {
        let set_card = 
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let mut hand_set = get_hand_bid(set_card);
        hand_set.sort_by(|left, right| order_hand(left, right) );
        println!("{:?}", hand_set);
        let total_wining: usize = hand_set.iter().enumerate()
            .map(|(i, set)| (i+1) * set.1).sum();
        assert_eq!(5905, total_wining); 


    }
}