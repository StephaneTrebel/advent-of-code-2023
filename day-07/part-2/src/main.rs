use std::{cmp::Ordering, fmt::Display, fs};

use itertools::Itertools;

fn get_file_content(file_path: &String) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::Joker => write!(f, "J"),
            Card::Two => write!(f, "2"),
            Card::Three => write!(f, "3"),
            Card::Four => write!(f, "4"),
            Card::Five => write!(f, "5"),
            Card::Six => write!(f, "6"),
            Card::Seven => write!(f, "7"),
            Card::Eight => write!(f, "8"),
            Card::Nine => write!(f, "9"),
            Card::Ten => write!(f, "T"),
            Card::Queen => write!(f, "Q"),
            Card::King => write!(f, "K"),
            Card::Ace => write!(f, "A"),
        }
    }
}

impl TryFrom<char> for Card {
    fn try_from(c: char) -> Result<Self, Self::Error> {
        if c == 'A' {
            Ok(Card::Ace)
        } else if c == 'K' {
            Ok(Card::King)
        } else if c == 'Q' {
            Ok(Card::Queen)
        } else if c == 'T' {
            Ok(Card::Ten)
        } else if c == '9' {
            Ok(Card::Nine)
        } else if c == '8' {
            Ok(Card::Eight)
        } else if c == '7' {
            Ok(Card::Seven)
        } else if c == '6' {
            Ok(Card::Six)
        } else if c == '5' {
            Ok(Card::Five)
        } else if c == '4' {
            Ok(Card::Four)
        } else if c == '3' {
            Ok(Card::Three)
        } else if c == '2' {
            Ok(Card::Two)
        } else if c == 'J' {
            Ok(Card::Joker)
        } else {
            Err(format!("Invalid card: {}", c))
        }
    }

    type Error = String;
}

#[cfg(test)]
mod tests_card {
    use super::*;

    #[test]
    fn card_try_from_ok() {
        assert_eq!(Card::try_from('T'), Ok(Card::Ten));
    }

    #[test]
    fn card_try_from_ko() {
        assert_eq!(Card::try_from('X').is_err(), true,);
    }
}

#[derive(Ord, PartialEq, Eq, Debug, Clone)]
enum Kind {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::HighCard => {
                write!(f, "HighCard")
            }
            Kind::OnePair => write!(f, "OnePair"),
            Kind::TwoPairs => write!(f, "TwoPairs"),
            Kind::ThreeOfAKind => {
                write!(f, "ThreeOfAKind")
            }
            Kind::FullHouse => write!(f, "FullHouse"),
            Kind::FourOfAKind => write!(f, "FourOfAKind"),
            Kind::FiveOfAKind => write!(f, "FiveOfAKind"),
        }
    }
}

impl From<&str> for Kind {
    fn from(str_cards: &str) -> Self {
        let cards: Vec<Card> = str_cards
            .chars()
            .map(|c| Card::try_from(c).unwrap())
            .collect();

        let sorted = {
            let mut temp = cards.clone();
            temp.sort();
            temp.reverse();
            temp
        };
        let mut deduped: Vec<(usize, &Card)> =
            sorted.iter().dedup_by_with_count(|x, y| x.eq(y)).collect();
        deduped.sort_by(|a, b| match a.0.partial_cmp(&b.0).unwrap() {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => a.1.partial_cmp(b.1).unwrap(),
        });
        deduped.reverse();

        let joker_count = {
            if deduped.len() > 1 {
                match deduped.iter().find(|e| e.1 == &Card::Joker) {
                    Some((count, _)) => *count,
                    None => 0,
                }
            } else {
                // For the only tricky case: A FiveOfAKind of Jokers...
                0
            }
        };

        if joker_count != 0 {
            dbg!(&deduped);
            print!("{} ", &joker_count);
        }

        // Obviously the main card cannot be a Joker
        let most_copies = {
            if deduped[0].1 == &Card::Joker && deduped.len() > 1 {
                deduped[1].0
            } else {
                // For the only tricky case: A FiveOfAKind of Jokers...
                deduped[0].0
            }
        }
        // Using jokers to upgrade the highest kind (when applicable)
         + joker_count;

        if joker_count != 0 {
            dbg!(&most_copies);
        }

        let kind: Kind = {
            if most_copies == 5 {
                Kind::FiveOfAKind
            } else if most_copies == 4 {
                Kind::FourOfAKind
            } else if most_copies == 3 && deduped[1].0 == 2 {
                Kind::FullHouse
            } else if most_copies == 3 {
                Kind::ThreeOfAKind
            } else if most_copies == 2 && deduped[1].0 == 2 {
                Kind::TwoPairs
            } else if most_copies == 2 {
                Kind::OnePair
            } else if most_copies == 1 {
                Kind::HighCard
            } else {
                Err("Invalid kind").expect("Fatal Error")
            }
        };

        if joker_count != 0 {
            println!("{}", &kind);
        }
        return kind;
    }
}

#[cfg(test)]
mod tests_kind_from {
    use super::*;

    #[test]
    fn detect_high_card() {
        assert_eq!(Kind::from("23456"), Kind::HighCard);
    }

    #[test]
    fn detect_one_pair() {
        assert_eq!(Kind::from("23256"), Kind::OnePair);
    }

    #[test]
    fn detect_two_pairs() {
        assert_eq!(Kind::from("23236"), Kind::TwoPairs);
    }

    #[test]
    fn detect_three_of_a_kind() {
        assert_eq!(Kind::from("23433"), Kind::ThreeOfAKind);
    }

    #[test]
    fn detect_full_house() {
        assert_eq!(Kind::from("23233"), Kind::FullHouse);
    }

    #[test]
    fn detect_four_of_a_kind() {
        assert_eq!(Kind::from("23333"), Kind::FourOfAKind);
    }

    #[test]
    fn detect_five_of_a_kind() {
        assert_eq!(Kind::from("33333"), Kind::FiveOfAKind);
    }
}

impl PartialOrd for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Kind::OnePair, Kind::HighCard)
            | (Kind::TwoPairs, Kind::HighCard)
            | (Kind::TwoPairs, Kind::OnePair)
            | (Kind::ThreeOfAKind, Kind::HighCard)
            | (Kind::ThreeOfAKind, Kind::OnePair)
            | (Kind::ThreeOfAKind, Kind::TwoPairs)
            | (Kind::FullHouse, Kind::HighCard)
            | (Kind::FullHouse, Kind::OnePair)
            | (Kind::FullHouse, Kind::TwoPairs)
            | (Kind::FullHouse, Kind::ThreeOfAKind)
            | (Kind::FourOfAKind, Kind::HighCard)
            | (Kind::FourOfAKind, Kind::OnePair)
            | (Kind::FourOfAKind, Kind::TwoPairs)
            | (Kind::FourOfAKind, Kind::ThreeOfAKind)
            | (Kind::FourOfAKind, Kind::FullHouse)
            | (Kind::FiveOfAKind, Kind::HighCard)
            | (Kind::FiveOfAKind, Kind::OnePair)
            | (Kind::FiveOfAKind, Kind::TwoPairs)
            | (Kind::FiveOfAKind, Kind::ThreeOfAKind)
            | (Kind::FiveOfAKind, Kind::FullHouse)
            | (Kind::FiveOfAKind, Kind::FourOfAKind) => Some(Ordering::Greater),
            (Kind::HighCard, Kind::OnePair)
            | (Kind::HighCard, Kind::TwoPairs)
            | (Kind::OnePair, Kind::TwoPairs)
            | (Kind::HighCard, Kind::ThreeOfAKind)
            | (Kind::OnePair, Kind::ThreeOfAKind)
            | (Kind::TwoPairs, Kind::ThreeOfAKind)
            | (Kind::HighCard, Kind::FullHouse)
            | (Kind::OnePair, Kind::FullHouse)
            | (Kind::TwoPairs, Kind::FullHouse)
            | (Kind::ThreeOfAKind, Kind::FullHouse)
            | (Kind::HighCard, Kind::FourOfAKind)
            | (Kind::OnePair, Kind::FourOfAKind)
            | (Kind::TwoPairs, Kind::FourOfAKind)
            | (Kind::ThreeOfAKind, Kind::FourOfAKind)
            | (Kind::FullHouse, Kind::FourOfAKind)
            | (Kind::HighCard, Kind::FiveOfAKind)
            | (Kind::OnePair, Kind::FiveOfAKind)
            | (Kind::TwoPairs, Kind::FiveOfAKind)
            | (Kind::ThreeOfAKind, Kind::FiveOfAKind)
            | (Kind::FullHouse, Kind::FiveOfAKind)
            | (Kind::FourOfAKind, Kind::FiveOfAKind) => Some(Ordering::Less),
            // Kind only deals on the…well "kind" level.
            // If two hands are of equal kind, comparisons will be done on
            // the Hand level
            _ => Some(Ordering::Equal),
        }
    }
}

#[cfg(test)]
mod tests_kind_partialord {
    use super::*;

    #[test]
    fn detect_high_card_equality() {
        assert_eq!(
            Kind::from("23456").partial_cmp(&Kind::from("23456")),
            Some(Ordering::Equal)
        );
    }

    #[test]
    fn detect_one_pair_vs_high_card() {
        assert_eq!(
            Kind::from("63456").partial_cmp(&Kind::from("23456")),
            Some(Ordering::Greater)
        );
    }

    #[test]
    fn detect_full_house_vs_four_of_a_kind() {
        assert_eq!(
            Kind::from("63636").partial_cmp(&Kind::from("22322")),
            Some(Ordering::Less)
        );
    }
}

#[derive(Debug, Eq, PartialEq, Ord)]
struct Hand {
    cards: String,
    kind: Kind,
    bid: u32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut ordering = self.kind.partial_cmp(&other.kind);
        match ordering {
            Some(Ordering::Equal) => {
                let mut self_chars = self.cards.chars().map(|c| Card::try_from(c).unwrap());
                let mut other_chars = other.cards.chars().map(|c| Card::try_from(c).unwrap());
                for _ in 0..5 {
                    ordering = self_chars
                        .next()
                        .unwrap()
                        .partial_cmp(&other_chars.next().unwrap());
                    if ordering != Some(Ordering::Equal) {
                        break;
                    }
                }
                return ordering;
            }
            _ => ordering,
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.cards, self.kind, self.bid)
    }
}

#[cfg(test)]
mod tests_hand {
    use std::cmp::Ordering;

    use super::*;

    #[test]
    // This is the trick test of the problem description.
    // It's not the same as Poker !
    fn hand_partial_cmp() {
        assert_eq!(
            Hand {
                cards: String::from("33332"),
                kind: Kind::FourOfAKind,
                bid: 0
            }
            .partial_cmp(&Hand {
                cards: String::from("2AAAA"),
                kind: Kind::FourOfAKind,
                bid: 0
            }),
            Some(Ordering::Greater)
        );
    }
}

fn main() {
    let content = get_file_content(&String::from("assets/input"));

    let mut hands: Vec<Hand> = vec![];
    for str_hands in content.lines() {
        let mut split = str_hands.split_whitespace();
        let cards = split.next().unwrap();
        hands.push(Hand {
            cards: cards.to_string(),
            kind: Kind::from(cards),
            bid: split.next().unwrap().parse().unwrap_or(0),
        });
    }

    hands.sort();
    println!(
        "\nTotal Winnings: {}",
        hands.iter().enumerate().fold(0, |acc, (index, element)| acc
            + element.bid * (index as u32 + 1))
    );
}
