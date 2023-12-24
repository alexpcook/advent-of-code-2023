use std::{cmp::Ordering, collections::HashMap, fs};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Joker = 1,
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

impl From<char> for Card {
    fn from(value: char) -> Self {
        use Card::*;
        match value {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => Joker,
            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            _ => panic!("failed to conver char `{value}` to Card"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

impl<T> From<T> for Hand
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        let s = value.into();
        let mut chars = s.chars();

        Hand {
            cards: [
                Card::from(chars.next().unwrap()),
                Card::from(chars.next().unwrap()),
                Card::from(chars.next().unwrap()),
                Card::from(chars.next().unwrap()),
                Card::from(chars.next().unwrap()),
            ],
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        use HandType::*;

        let mut card_map: HashMap<Card, u32> = HashMap::with_capacity(5);

        for card in self.cards {
            card_map
                .entry(card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        match card_map.len() {
            1 => FiveOfAKind,
            2 => {
                // full house or four of a kind
                let count = *card_map.values().next().unwrap();

                if count == 1 || count == 4 {
                    FourOfAKind
                } else {
                    FullHouse
                }
            }
            3 => {
                // two pair or three of a kind
                let mut iter = card_map.values();

                let (count1, count2) = (*iter.next().unwrap(), *iter.next().unwrap());

                if count1 == 3 {
                    ThreeOfAKind
                } else if count1 == 2 {
                    TwoPair
                } else if count2 == 3 {
                    ThreeOfAKind
                } else if count2 == 2 {
                    TwoPair
                } else {
                    ThreeOfAKind
                }
            }
            4 => OnePair,
            5 => HighCard,
            n => panic!("got unexpected card count {n} in hand {self:?}"),
        }
    }

    fn hand_type_with_jokers(&self) -> HandType {
        use HandType::*;

        let hand_type = self.hand_type();

        let num_jokers = self.cards.iter().filter(|&&c| c == Card::Joker).count();

        if num_jokers == 0 {
            hand_type
        } else if num_jokers > 3 {
            FiveOfAKind
        } else if num_jokers == 3 {
            if hand_type == FullHouse {
                FiveOfAKind
            } else {
                // three of a kind
                FourOfAKind
            }
        } else if num_jokers == 2 {
            if hand_type == FullHouse {
                FiveOfAKind
            } else if hand_type == TwoPair {
                FourOfAKind
            } else {
                // one pair
                ThreeOfAKind
            }
        } else {
            if hand_type == FourOfAKind {
                FiveOfAKind
            } else if hand_type == ThreeOfAKind {
                FourOfAKind
            } else if hand_type == TwoPair {
                FullHouse
            } else if hand_type == OnePair {
                ThreeOfAKind
            } else {
                OnePair
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_hand_type = self.hand_type_with_jokers();
        let other_hand_type = other.hand_type_with_jokers();

        match self_hand_type.cmp(&other_hand_type) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            ord => ord,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input/day7.txt").unwrap();

    let mut hands: Vec<_> = input
        .lines()
        .map(|s| {
            let mut split = s.split_whitespace();
            (
                Hand::from(split.next().unwrap()),
                split.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect();

    hands.sort_unstable_by(|(hand1, _), (hand2, _)| hand1.cmp(hand2));

    let winnings = hands
        .into_iter()
        .enumerate()
        .fold(0, |result, (i, (_, bid))| {
            result + (u32::try_from(i).unwrap() + 1) * bid
        });

    println!("day 7, part 1: {winnings}");
}
