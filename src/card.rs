use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::error::Error;

/// Represents the suit of a playing card.
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Suit {
    /// Clubs suit (♣)
    Club,
    /// Diamonds suit (♦)
    Diamond,
    /// Hearts suit (♥)
    Heart,
    /// Spades suit (♠)
    Spade,
    /// Unknown or unspecified suit
    Unknown,
}

impl FromStr for Suit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(Error::ParseError(format!(r#"Invalid suit "{}". expect PHH card suit."#, s)));
        }

        match s.chars().nth(0).unwrap() {
            'c' => Ok(Suit::Club),
            'd' => Ok(Suit::Diamond),
            'h' => Ok(Suit::Heart),
            's' => Ok(Suit::Spade),
            '?' => Ok(Suit::Unknown),
            _ => Err(Error::ParseError(format!(r#"Invalid suit "{}". expect PHH card suit."#, s))),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let suit_str = match self {
            Suit::Club => "c",
            Suit::Diamond => "d",
            Suit::Heart => "h",
            Suit::Spade => "s",
            Suit::Unknown => "?",
        };
        write!(f, "{}", suit_str)
    }
}

/// Represents the rank of a playing card.
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Rank {
    /// Ace (A) - high or low depending on game rules
    Ace,
    /// Deuce (2)
    Deuce,
    /// Trey (3)
    Trey,
    /// Four (4)
    Four,
    /// Five (5)
    Five,
    /// Six (6)
    Six,
    /// Seven (7)
    Seven,
    /// Eight (8)
    Eight,
    /// Nine (9)
    Nine,
    /// Ten (T)
    Ten,
    /// Jack (J)
    Jack,
    /// Queen (Q)
    Queen,
    /// King (K)
    King,
    /// Unknown or unspecified rank
    Unknown,
}

impl FromStr for Rank {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(Error::ParseError(format!(r#"Invalid rank "{}". expect PHH card rank."#, s)));
        }

        match s.chars().nth(0).unwrap() {
            'A' => Ok(Rank::Ace),
            '2' => Ok(Rank::Deuce),
            '3' => Ok(Rank::Trey),
            '4' => Ok(Rank::Four),
            '5' => Ok(Rank::Five),
            '6' => Ok(Rank::Six),
            '7' => Ok(Rank::Seven),
            '8' => Ok(Rank::Eight),
            '9' => Ok(Rank::Nine),
            'T' => Ok(Rank::Ten),
            'J' => Ok(Rank::Jack),
            'Q' => Ok(Rank::Queen),
            'K' => Ok(Rank::King),
            '?' => Ok(Rank::Unknown),
            _ => Err(Error::ParseError(format!(r#"Invalid rank "{}". expect PHH card rank."#, s))),
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let rank_str = match self {
            Rank::Ace => "A",
            Rank::Deuce => "2",
            Rank::Trey => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "T",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Unknown => "?",
        };
        write!(f, "{}", rank_str)
    }
}

/// Represents a playing card with a rank and suit.
///
/// A complete representation of a playing card, consisting of a rank (A-K)
/// and a suit (c/d/h/s).
#[derive(Debug, PartialEq, Clone, Copy, Hash)]
pub struct Card {
    /// The rank of the card
    pub rank: Rank,
    /// The suit of the card
    pub suit: Suit,
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(Error::ParseError(format!(r#"Invalid card "{}". expect PHH card."#, s)));
        }
        Ok(Card {
            rank: Rank::from_str(&s[0..1])?,
            suit: Suit::from_str(&s[1..2])?,
        })
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl Card {
    pub fn from_str_cards(s: &str) -> Result<Vec<Self>, Error> {
        if s.len() % 2 != 0 {
            return Err(Error::ParseError(format!("Invalid cards: {}", s)));
        }
        let mut array = Vec::with_capacity(s.len() / 2);

        for i in 0..(s.len()/2) {
            array.push(Card::from_str(&s[(i * 2)..(i * 2 + 2)])?);
        }

        Ok(array)
    }
}
