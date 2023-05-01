use std::cmp::{max, min};
use std::collections::HashMap;
/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14, // Ace can also count as 1, but it only matters in the special case of a straight
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRankError;

impl FromStr for Rank {
    type Err = ParseRankError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rank = match s.to_uppercase().as_str() {
            "2" => Rank::Two,
            "3" => Rank::Three,
            "4" => Rank::Four,
            "5" => Rank::Five,
            "6" => Rank::Six,
            "7" => Rank::Seven,
            "8" => Rank::Eight,
            "9" => Rank::Nine,
            "10" => Rank::Ten,
            "J" => Rank::Jack,
            "Q" => Rank::Queen,
            "K" => Rank::King,
            "A" => Rank::Ace,
            _ => return Err(ParseRankError),
        };
        Ok(rank)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSuitError;

impl FromStr for Suit {
    type Err = ParseSuitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "S" => Ok(Suit::Spades),
            "H" => Ok(Suit::Hearts),
            "D" => Ok(Suit::Diamonds),
            "C" => Ok(Suit::Clubs),
            _ => Err(ParseSuitError),
        }
    }
}

#[derive(PartialEq, Eq)]
struct Card {
    rank: Rank,
    suit: Suit,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCardError;

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rank = match Rank::from_str(&s[0..s.len() - 1]) {
            Ok(rank) => rank,
            Err(..) => return Err(ParseCardError),
        };
        let suit = match Suit::from_str(&s[s.len() - 1..s.len()]) {
            Ok(suit) => suit,
            Err(..) => return Err(ParseCardError),
        };
        // Ok(Card(rank: rank, suit: suit))
        Ok(Card {
            rank: rank,
            suit: suit,
        })
    }
}

struct Hand {
    // "3S 4S 5D 6H JH"
    cards: Vec<Card>,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Copy, Clone, Debug)]
enum HandRank {
    // Ordering is derived. Higher ordering is a better hand.
    HighCard(Rank, Rank, Rank, Rank, Rank),
    OnePair(Rank, Rank, Rank, Rank),
    TwoPair(Rank, Rank, Rank),
    ThreeOfAKind(Rank, Rank, Rank),
    Straight(Rank),
    Flush(Rank, Rank, Rank, Rank, Rank),
    FullHouse(Rank, Rank),
    FourOfAKind(Rank, Rank),
    StraightFlush(Rank),
}

impl Hand {
    fn new(mut cards: Vec<Card>) -> Self {
        // Sort cards lowest to highest rank
        cards.sort_by(|card1, card2| card1.rank.cmp(&card2.rank));
        Self { cards }
    }

    fn rank(self) -> HandRank {
        // Count the number of occurrances for each rank
        let mut rank_counts: HashMap<Rank, u8> = HashMap::new();
        for card in &self.cards {
            rank_counts
                .entry(card.rank)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        // Sort rank counts in descending order
        let mut rank_counts_sorted: Vec<(&Rank, &u8)> = rank_counts.iter().collect();
        rank_counts_sorted.sort_by(|a, b| b.1.cmp(a.1));
        if *rank_counts_sorted[0].1 == 4 {
            return HandRank::FourOfAKind(*rank_counts_sorted[0].0, *rank_counts_sorted[1].0);
        } else if *rank_counts_sorted[0].1 == 3 {
            if *rank_counts_sorted[1].1 == 2 {
                return HandRank::FullHouse(*rank_counts_sorted[0].0, *rank_counts_sorted[1].0);
            } else {
                return HandRank::ThreeOfAKind(
                    *rank_counts_sorted[0].0,
                    *rank_counts_sorted[1].0,
                    *rank_counts_sorted[2].0,
                );
            }
        } else if *rank_counts_sorted[0].1 == 2 {
            if *rank_counts_sorted[1].1 == 2 {
                let pair1 = *rank_counts_sorted[0].0;
                let pair2 = *rank_counts_sorted[1].0;
                let kicker = *rank_counts_sorted[2].0;
                return HandRank::TwoPair(max(pair1, pair2), min(pair1, pair2), kicker);
            } else {
                return HandRank::OnePair(
                    *rank_counts_sorted[0].0,
                    *rank_counts_sorted[1].0,
                    *rank_counts_sorted[2].0,
                    *rank_counts_sorted[3].0,
                );
            }
        }

        let flush = self.cards.iter().all(|c| c.suit == self.cards[0].suit);
        match self.cards[..] {
            // Straight flush with `a` as highest ranking card
            [Card { rank: a, .. }, Card { rank: b, .. }, Card { rank: c, .. }, Card { rank: d, .. }, Card { rank: e, .. }]
                if (a as u32 == b as u32 - 1)
                    && (b as u32 == c as u32 - 1)
                    && (c as u32 == d as u32 - 1)
                    && (d as u32 == e as u32 - 1) =>
            {
                if flush {
                    return HandRank::StraightFlush(e);
                } else {
                    return HandRank::Straight(e);
                }
            }
            [Card {
                rank: Rank::Two, ..
            }, Card {
                rank: Rank::Three, ..
            }, Card {
                rank: Rank::Four, ..
            }, Card {
                rank: Rank::Five, ..
            }, Card {
                rank: Rank::Ace, ..
            }] => {
                if flush {
                    // Special case: Five-high (ace as 1) straight flush
                    return HandRank::StraightFlush(Rank::Five);
                } else {
                    // Special case: Five-high (ace as 1) straight
                    return HandRank::Straight(Rank::Five);
                }
            }
            _ => {}
        }
        if flush {
            return HandRank::Flush(
                self.cards[0].rank,
                self.cards[1].rank,
                self.cards[2].rank,
                self.cards[3].rank,
                self.cards[4].rank,
            );
        }
        return HandRank::HighCard(
            self.cards[0].rank,
            self.cards[1].rank,
            self.cards[2].rank,
            self.cards[3].rank,
            self.cards[4].rank,
        );
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(" ").map(Card::from_str).collect() {
            Ok(cards) => Ok(Hand::new(cards)),
            Err(..) => Err(ParseHandError),
        }
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mb_parsed_hands: Result<Vec<Hand>, _> =
        hands.iter().map(|hand| Hand::from_str(&hand)).collect();
    let parsed_hands: Vec<Hand> = mb_parsed_hands.unwrap();
    let mut ranked_hands: Vec<(HandRank, &&str)> = parsed_hands
        .into_iter()
        .map(|hand| hand.rank())
        .zip(hands.into_iter())
        .collect();
    ranked_hands.sort_by(|(rank1, _hand1), (rank2, _hand2)| rank2.cmp(&rank1));
    println!("ranked hands: {:?}", ranked_hands);
    let best_rank = ranked_hands[0].0.clone();
    let best_hands: Vec<&str> = ranked_hands
        .into_iter()
        .take_while(|(rank, _hand)| rank == &best_rank)
        .map(|(_rank, hand)| *hand)
        .collect();
    println!("hands: {:?} of which best: {:?}", hands, best_hands);
    best_hands
}
