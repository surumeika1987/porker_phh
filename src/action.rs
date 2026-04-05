use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{de, Deserialize, Deserializer};
use serde::de::{Visitor, Unexpected};
use crate::card::Card;
use crate::error::Error;

/// Represents an action or event that occurs in a poker hand.
///
/// Actions encompass all events that can occur during a hand of poker,
/// from dealing cards to player actions (bet, check, fold, etc.) to comments.
#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    /// Dealing community cards (used in flop/turn/river situations)
    DealingCC { cards: Vec<Card> },
    /// Dealing down (hole) or up (exposed) cards to a specific player
    DealingDUC { player: u32, cards: Vec<Card> },
    /// Bringing in a bet (typically in stud games)
    BringingIn { player: u32 },
    /// Completing, betting, or raising by a specific amount
    CBR { player: u32, amount: f64 },
    /// Checking or calling
    CC { player: u32 },
    /// Folding
    Folding { player: u32 },
    /// Standing pat or discarding cards (with optional cards shown)
    SD { player: u32, cards: Option<Vec<Card>> },
    /// Showing or mucking their hole cards
    SM { player: u32, cards: Option<Vec<Card>> },
    /// A comment or note about the hand
    Comment(String),
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> { 
        if s.starts_with("#") {
            return Ok(Action::Comment(s[1..].trim().to_string()));
        }
        let split = s.split(' ').collect::<Vec<&str>>();
        if split.len() < 2 {
            return Err(Error::ParseError(format!(r#"Invalid action "{}". expect PHH action."#, s)));
        }
        let key = split[0];
        let secound_key = split[1];

        if key == "d" {
            match &*secound_key {
                "db" => {
                    if split.len() < 3 {
                        return Err(Error::ParseError(format!(r#"Invalid action "{}". expect PHH action."#, s)));
                    }
                    let cards = Card::from_str_cards(split[2])?;
                    return Ok(Action::DealingCC { cards })
                }
                "dh" => {
                    if split.len() < 4 {
                        return Err(Error::ParseError(format!(r#"Invalid action "{}". expect PHH action."#, s)));
                    }
                    let player = Self::parse_as_player_number(split[2])?;
                    let cards = Card::from_str_cards(split[3])?;
                    return Ok(Action::DealingDUC { player, cards });
                }
                _ => return Err(Error::ParseError(format!(r#"Invalid action "{}". expect PHH action."#, s))),
            }
        }

        if key.starts_with("p") {
            let player = Self::parse_as_player_number(key)?;
            let key = secound_key;
            match &*key {
                "pb" => return Ok(Action::BringingIn { player }),
                "cbr" => {
                    if split.len() < 3 {
                        return Err(Error::ParseError(format!(r#"Invalid cbr action "{}". expect "p cbr [amount]."#, s)));
                    }
                    let amount = split[2].parse::<f64>();
                    if let Ok(amount) = amount {
                        return Ok(Action::CBR { player, amount });
                    } else {
                        return Err(Error::ParseError(format!(r#"Invalid cbr action "{}". expect "p cbr [amount]."#, s)));
                    }
                }
                "cc" => return Ok(Action::CC { player }),
                "f" => return Ok(Action::Folding { player }),
                "sd" => {
                    if split.len() < 3 {
                        return Ok(Action::SD { player, cards: None });
                    }
                    if split[2].chars().nth(0).unwrap() == '#' {
                        return Ok(Action::SD { player, cards: None });
                    }
                    let cards = Card::from_str_cards(split[2])?;
                    return Ok(Action::SD { player, cards: Some(cards) });
                }
                "sm" => {
                    if split.len() < 3 {
                        return Ok(Action::SM { player, cards: None });
                    }
                    if split[2].chars().nth(0).unwrap() == '#' {
                        return Ok(Action::SM { player, cards: None });
                    }
                    let cards = Card::from_str_cards(split[2])?;
                    return Ok(Action::SM { player, cards: Some(cards) });
                }
                _ => return Err(Error::ParseError(format!(r#"Invalid action "{}". expect PHH action."#, s))),
            }
        }

        Err(Error::ParseError(format!(r#"Invalid action "{}". expect PHH action."#, s)))
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let array_card_to_str = |cv: &Vec<Card>| cv.iter().map(|c| c.to_string()).collect::<Vec<String>>().join("");
        let action = match self {
            Action::DealingCC { cards } => format!("d db {}", array_card_to_str(cards)),
            Action::DealingDUC { player, cards } => format!("d dh p{} {}", player, array_card_to_str(cards)),
            Action::BringingIn { player } => format!("p{} pb", player),
            Action::CBR { player, amount } => format!("p{} cbr {}", player, amount),
            Action::CC { player } => format!("p{} cc", player),
            Action::Folding { player } => format!("p{} f", player),
            Action::SD { player, cards } => {
                match cards {
                    Some(value) => format!("p{} sd {}", player, array_card_to_str(value)),
                    None => format!("p{} sb", player),
                }
            }
            Action::SM { player, cards } => {
                match cards {
                    Some(value) => format!("p{} sm {}", player, array_card_to_str(value)),
                    None => format!("p{} sm", player),
                }
            },
            Action::Comment(message) => format!("# {}", message),
        };

        write!(f, "{}", action)
    }
}

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>
    {
        struct ActionVisitor;

        impl<'de> Visitor<'de> for ActionVisitor {
            type Value = Action;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PHH action")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: de::Error,
            {
                match Action::from_str(v) {
                    Ok(value) => return Ok(value),
                    Err(_) => return Err(de::Error::invalid_value(Unexpected::Str(v), &self)),
                }
            }
        }

        deserializer.deserialize_str(ActionVisitor)
    }
}

impl Action {
    fn parse_as_player_number(contents: &str) -> Result<u32, Error> {
        if contents.len() < 2 {
            return Err(Error::ParseError(format!("Invalid player number: {}", contents)));
        }
        if contents.chars().nth(0).unwrap() != 'p' {
            return Err(Error::ParseError(format!("Invalid player number: {}", contents)));
        }
        if let Ok(value) = contents[1..].parse() {
            return Ok(value);
        }
        Err(Error::ParseError(format!("Invliad player number: {}", contents)))
    }
}
