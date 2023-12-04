use std::fs;
fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("cant't find the file");
    println!("the answer = {}",sum_cards(contents.as_str()));
}


fn get_card_point(card: &str) -> usize {
    let winning_numbers: Vec<usize> =
    card.split(":").nth(1).unwrap().trim()
        .split('|').nth(0).unwrap().trim()
        .split(" ")
        .filter(|a| !a.is_empty())
        .map(|number| number.parse::<usize>().unwrap())
        .collect();

    let holding_number: Vec<usize> =
        card.split(":").nth(1).unwrap()
        .trim()
        .split('|').nth(1).unwrap().trim()
        .split(" ")
        .filter(|a| !a.is_empty())
        .map(|number| number.parse::<usize>().unwrap())
        .collect();
    
    let mut matched_numbers: usize = 0;
    for wining_number in winning_numbers {
        if holding_number.contains(&wining_number) {
            matched_numbers += 1;
        }
}
        return matched_numbers;

}

fn sum_cards(cards: &str) -> usize {
    //              (matching_number, number_of_copy)
    let mut cards_numbers: Vec<(usize, usize)> = cards.lines()
        .map(|card| (get_card_point(card.trim()), 1))
        .collect();
    

    for i in 0..cards_numbers.len() {
        for _ in 0.. cards_numbers[i].1 {
            if cards_numbers[i].0 > 0 {
                let mut j = i+1;
                while j < cards_numbers.len() && j < i + cards_numbers[i].0 + 1 {
                    //println!("increasing card {} by 1 from card {}", j+1, i+1);
                    cards_numbers[j].1 += 1;
                    j +=1 ;
                }
            }
        }
    }
    cards_numbers.iter()
        .map(|a| a.1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cards_test()  {
        let cards = 
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        // assert_eq!(8, get_card_point("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"));
        // assert_eq!(2, get_card_point("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"));
        // assert_eq!(2, get_card_point("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"));
        // assert_eq!(1, get_card_point("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"));
        // assert_eq!(0, get_card_point("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"));
        // assert_eq!(0, get_card_point("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"));
        assert_eq!(30, sum_cards(cards));
    }

}