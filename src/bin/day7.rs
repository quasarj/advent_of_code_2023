use std::collections::HashMap;
use core::cmp::Ordering;
use std::fs::read_to_string;

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Hash)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u64,
}

impl Ord for Hand {
    // A custom ordering that looks at HandType first, and then the
    // actual card vector after
    fn cmp(&self, other: &Self) -> Ordering {
        (self.get_type(), &self.cards).cmp(&(other.get_type(), &other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn from_str(full_hand_string: &str) -> Self {
        let parts: Vec<_> = full_hand_string.split(" ").collect();
        if parts.len() != 2 {
            println!("Didn't split right: {full_hand_string}");
            panic!("ahhhh");
        }

        let hand_str = parts[0];
        let bid_str = parts[1];

        let bid: u64 = bid_str.parse().expect("bid must be a number!");

        Self::from_str_with_bid(hand_str, bid)
    }
    fn from_str_with_bid(hand_string: &str, bid: u64) -> Self {
        let mut ret = Hand { cards: Vec::new(), bid };
        for c in hand_string.chars() {
            // println!("{}", c);
            ret.cards.push(match c {
                'A' => Card::Ace,
                'K' => Card::King,
                'Q' => Card::Queen,
                'J' => Card::Jack,
                'T' => Card::Ten,
                '9' => Card::Nine,
                '8' => Card::Eight,
                '7' => Card::Seven,
                '6' => Card::Six,
                '5' => Card::Five,
                '4' => Card::Four,
                '3' => Card::Three,
                '2' => Card::Two,
                _ => todo!(),
            });
        }

        ret
    }

    fn get_type(&self) -> HandType {

        // get frequencies of all cards
        let mut m: HashMap<Card, usize> = HashMap::new();
        for x in self.cards.clone() {
            *m.entry(x).or_default() += 1;
        }


        // let s: String = m.values().into_iter().map(|x| x.to_string()).collect();
        // println!("{}", s);
        let mut vals: Vec<usize> = m.values().cloned().collect();
        vals.sort();
        let s: String = vals.iter().map(|x| x.to_string()).collect();

        println!("{:?}", vals);
        println!("{}", s);
        println!("{:?}", m);

        // get a sorted, unique list
        let mut ucards = self.cards.clone();
        ucards.sort();
        ucards.dedup();

        if ucards.len() == 1 {
            return HandType::FiveOfAKind;
        }

        if ucards.len() == 2 {
            // must be  : 4 of a kind
            //          : full house
            //          :
            // AAAAT
            // AAATT
            // AATTT
            if s == "23" {
                return HandType::FullHouse;
            }
            return HandType::FourOfAKind;
        }

        if ucards.len() == 3 {
            // must be  : 3 of a kind
            //          : two pair
            //
            // 23444
            if s == "122" {
                return HandType::TwoPair;
            }
            return HandType::ThreeOfAKind;
        }

        if ucards.len() == 4 {
            // must be  : pair
            // 
            // 44A55
            return HandType::OnePair;
        }

        HandType::HighCard
    }
}

fn main() {
    println!("Day 7");

    // let hand_list = vec![
    //     "32T3K",
    //     "T55J5",
    //     "KK677",
    //     "KTJJT",
    //     "QQQJA",
    // ];

    // let hand = Hand::from_str("32T3K 765");
    // println!("{:?}", hand);

    // let filename = "input/day7-test.txt";
    let filename = "input/day7.txt";
    let input_file = read_to_string(filename).unwrap();
    let lines = input_file.lines();

    let mut hand_list: Vec<Hand> = lines
        .map(|x| Hand::from_str(x))
        .collect();

    hand_list.sort();

    // to score
    let mut total_score = 0;
    for (i, hand) in hand_list.iter().enumerate() {
        let rank = i + 1;
        let score = (rank as u64) * hand.bid;
        println!("{rank}:{:?}", hand);
        println!("{rank}:{:?}", score);
        total_score += score;
    }

    println!("Final score: {}", total_score);

}

#[test]
fn hand_type_test() {
    assert_eq!(
        Hand::from_str("AAAAA").get_type(),
        HandType::FiveOfAKind,
        "Five of a kind"
    );
    assert_eq!(
        Hand::from_str("AAAAT").get_type(),
        HandType::FourOfAKind,
        "Four of a Kind"
    );
    assert_eq!(
        Hand::from_str("AAATT").get_type(),
        HandType::FullHouse,
        "Full House"
    );
    assert_eq!(
        Hand::from_str("AAA23").get_type(),
        HandType::ThreeOfAKind,
        "Three of a Kind"
    );
    assert_eq!(
        Hand::from_str("AAKJJ").get_type(),
        HandType::TwoPair,
        "Two Pair"
    );
    assert_eq!(
        Hand::from_str("AAKJT").get_type(),
        HandType::OnePair,
        "One Pair"
    );
    assert_eq!(
        Hand::from_str("AKQ23").get_type(),
        HandType::HighCard,
        "High Card"
    );
}

#[test]
fn card_test() {
    assert!(Card::Ace > Card::King);
}

#[test]
fn hand_test() {
    let hand1 = Hand {
        cards: vec![Card::Two, Card::Three, Card::Four],
    };

    let hand2 = Hand {
        cards: vec![Card::Two, Card::Three, Card::Five],
    };

    assert!(hand2 > hand1);
    assert!(hand2 > hand1);
}
