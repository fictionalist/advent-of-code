#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind
}

struct Hand {
    hand_type: HandType,
    cards: Vec<char>,
    bet: i32
}

/*fn get_test_data() -> String {
    "2345A 1
    Q2KJJ 13
    Q2Q2Q 19
    T3T3J 17
    T3Q33 11
    2345J 3
    J345A 2
    32T3K 5
    T55J5 29
    KK677 7
    KTJJT 34
    QQQJA 31
    JJJJJ 37
    JAAAA 43
    AAAAJ 59
    AAAAA 61
    2AAAA 23
    2JJJJ 53
    JJJJ2 41".to_string()
}*/

fn get_card_strength(card: &char) -> i8 {
    let cards = vec!('J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A');

    for (idx, a) in cards.iter().enumerate() {
        if a == card {
            return idx as i8;
        }
    }
    return -1;
}

fn compare_hand_and_strength(a: &Hand, b: &Hand) -> std::cmp::Ordering {
    if a.hand_type == b.hand_type {
        for (idx, card) in a.cards.iter().enumerate() {
            let order = get_card_strength(card).cmp(&get_card_strength(&b.cards[idx]));
            match order {
                std::cmp::Ordering::Equal => continue,
                _ => return order
            }
        }
        return std::cmp::Ordering::Equal;
    } else {
        return a.hand_type.partial_cmp(&b.hand_type).unwrap();
    }
}

fn get_hand(line: &str) -> Hand {
    let mut hand_a_count = std::collections::HashMap::<char, i8>::new();
    let mut cards: Vec<char> = vec!();

    let mut split = line.split_whitespace();
    let hand_a = split.next().unwrap();

    for card in hand_a.chars().into_iter() {
        let card_number = hand_a_count.get(&card).unwrap_or(&0);
        hand_a_count.insert(card, card_number + 1);
        cards.push(card);
    }

    let mut hand_type = HandType::HighCard;

    if hand_a_count.iter().any(|i| i.1 == &5) {
        hand_type = HandType::FiveKind;
    } else if hand_a_count.iter().any(|i| i.1 == &4) {
        hand_type = HandType::FourKind;
    } else if hand_a_count.iter().any(|i| i.1 == &3) && hand_a_count.iter().any(|i| i.1 == &2) {
        hand_type = HandType::FullHouse;
    } else if hand_a_count.iter().any(|i| i.1 == &3) {
        hand_type = HandType::ThreeKind;
    } else if hand_a_count.iter().any(|i| i.1 == &2) {
        hand_type = HandType::OnePair;
        if hand_a_count.iter().filter(|a| a.1 == &2).collect::<Vec<(&char, &i8)>>().len() == 2 {
            hand_type = HandType::TwoPair;
        }
    }

    let bet = split.next().unwrap().parse::<i32>().unwrap();

    Hand { hand_type, cards, bet }
}

fn part_one(lines: &Vec<&str>) -> i32 {
    let mut hands: Vec<Hand> = vec!();

    for line in lines {
        hands.push(get_hand(line));
    }

    hands.sort_by(|a, b| compare_hand_and_strength(a, b));
    
    let mut winnings = 0;
    for (idx, hand) in hands.iter().enumerate() {
        //println!("{:?} {:?} {:}\t({})\t=>({})", hand.hand_type, hand.cards, hand.bet, idx + 1, ((idx as i32) + 1) * hand.bet);
        winnings += ((idx as i32) + 1) * hand.bet;
    }

    winnings
}

fn get_hand_with_joker(line: &str) -> Hand {
    let mut hand_a_count = std::collections::HashMap::<char, i8>::new();
    let mut cards: Vec<char> = vec!();

    let mut split = line.split_whitespace();
    let hand_a = split.next().unwrap();

    for card in hand_a.chars().into_iter() {
        let card_number = hand_a_count.get(&card).unwrap_or(&0);
        hand_a_count.insert(card, card_number + 1);
        cards.push(card);
    }

    let mut hand_type = HandType::HighCard;

    if hand_a_count.iter().any(|i| i.1 == &5) {
        hand_type = HandType::FiveKind;
    } else if hand_a_count.iter().any(|i| i.1 == &4) {
        hand_type = HandType::FourKind;
        if hand_a_count.contains_key(&'J') {
            hand_type = HandType::FiveKind;
        }
    } else if hand_a_count.iter().any(|i| i.1 == &3) && hand_a_count.iter().any(|i| i.1 == &2) {
        hand_type = HandType::FullHouse;
        if hand_a_count.contains_key(&'J') {
            hand_type = HandType::FiveKind;
        }
    } else if hand_a_count.iter().any(|i| i.1 == &3) {
        hand_type = HandType::ThreeKind;
        if hand_a_count.contains_key(&'J') {
            hand_type = HandType::FourKind;
        }
    } else if hand_a_count.iter().any(|i| i.1 == &2) {
        hand_type = HandType::OnePair;

        if hand_a_count.iter().filter(|a| a.1 == &2).collect::<Vec<(&char, &i8)>>().len() == 2 {

            hand_type = HandType::TwoPair;
            if hand_a_count.contains_key(&'J') {
                let count = hand_a_count.get(&'J').unwrap();
                match *count {
                    1 => hand_type = HandType::FullHouse,
                    2 => hand_type = HandType::FourKind,
                    _ => {},
                }
            }
        }

        if hand_a_count.contains_key(&'J') {
            if hand_type == HandType::OnePair {
                let count = hand_a_count.get(&'J').unwrap();
                match *count {
                    1 => hand_type = HandType::ThreeKind,
                    2 => hand_type = HandType::ThreeKind,
                    _ => {},
                }
            }
        }
    } else {
        // 1234J -> 11234 one pair
        if hand_a_count.contains_key(&'J') {
            hand_type = HandType::OnePair;
        }
    }
    
    let bet = split.next().unwrap().parse::<i32>().unwrap();

    Hand { hand_type, cards, bet }
}

fn part_two(lines: &Vec<&str>) -> i32 {
    let mut hands: Vec<Hand> = vec!();

    for line in lines {
        let hand = get_hand_with_joker(line);
        //println!("{:?}  \t{:?} {}", hand.hand_type, hand.cards, hand.bet);
        hands.push(hand);
    }

    hands.sort_by(|a, b| compare_hand_and_strength(a, b));
    
    let mut winnings = 0;
    for (idx, hand) in hands.iter().enumerate() {
        //println!("{:?}  \t{:?} {}\t=>({})", hand.hand_type, hand.cards, hand.bet, idx);
        winnings += ((idx as i32) + 1) * hand.bet;
    }

    winnings
}

pub fn main() -> Result<(), std::io::Error> {
    let data = std::fs::read_to_string("input/input07.txt")?;

    //let data = get_test_data();

    let lines = data.trim().lines().collect();

    println!("Day 07:");

    let start = std::time::Instant::now();
    println!("\tPart 1 - Winnings: {} ({} ms)", part_one(&lines), start.elapsed().as_millis());

    let start = std::time::Instant::now();
    println!("\tPart 2 - Winnings: {} ({} ms)", part_two(&lines), start.elapsed().as_millis());

    Ok(())
}