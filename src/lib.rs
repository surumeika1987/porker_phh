use std::fmt::{Display, Formatter};
use time::Time;

pub enum Error {
    ParseError(String),
}

#[derive(Debug, PartialEq)]
pub enum Variant {
    FT,
    NT,
    NS,
    PO,
    FO8,
    F7S,
    F7S8,
    FR,
    N2L1D,
    F2L3D,
    FB,
}

impl Display for Variant {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let variant_str = match self {
            Variant::FT => "FT",
            Variant::NT => "NT",
            Variant::NS => "NS",
            Variant::PO => "PO",
            Variant::FO8 => "FO/8",
            Variant::F7S => "F7S",
            Variant::F7S8 => "F7S/8",
            Variant::FR => "FR",
            Variant::N2L1D => "N2L1D",
            Variant::F2L3D => "F2L3D",
            Variant::FB => "FB",
        };
        write!(f, "{}", variant_str)
    }
}

#[derive(Debug, PartialEq)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
    Unknown,
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

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    Deuce,
    Trey,
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
    Unknown,
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

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}

#[derive(Debug, PartialEq)]
pub enum Action {
    // Dealing community cards
    DealingCC { cards: Vec<Card> },
    // Dealing down/up cards
    DealingDUC { player: u32, cards: Vec<Card> },
    // Biringing in
    BringingIn { player: u32 },
    // Completing/Betting/Raising
    CBR { player: u32, amount: f64 },
    // Checking/Calling
    CC { player: u32 },
    // Folding
    Folding { player: u32 },
    // Standing pat/Discarding,
    SD { player: u32, cards: Option<Vec<Card>> },
    // Showing/Mucking their hole cards
    SM { player: u32, cards: Option<Vec<Card>> },
}

pub struct PHH {
    pub variant: Variant,
    pub antes: Vec<f64>,
    pub blinds_or_straddles: Option<Vec<f64>>,
    pub bring_in: Option<f64>,
    pub small_bet: Option<f64>,
    pub big_bet: Option<f64>,
    pub min_bet: Option<f64>,
    pub starting_stacks: Vec<f64>,
    pub actions: Vec<Action>,
    pub auther: Option<String>,
    pub event: Option<String>,
    pub url: Option<String>,
    pub venue: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub time: Option<Time>,
    pub time_zone: Option<String>,
    pub time_zone_abbreviation: Option<String>,
    pub day: Option<u32>,
    pub month: Option<u32>,
    pub year: Option<u32>,
    pub hand: Option<String>,
    pub level: Option<u32>,
    pub seats: Option<Vec<u32>>,
    pub seat_count: Option<u32>,
    pub table: Option<String>,
    pub players: Option<Vec<String>>,
    pub finishing_stacks: Option<Vec<f64>>,
    pub winnings: Option<Vec<f64>>,
    pub currency: Option<String>,
    pub currency_symbol: Option<String>,
    pub ante_trimming_status: Option<bool>,
    pub time_limit: Option<f64>,
    pub time_banks: Option<Vec<f64>>,
}

impl PHH {
    pub fn parse_from_str(phh_str: &str) -> Result<Self, Error> {
        let mut variant = None;
        let mut antes = None;
        let mut blinds_or_straddles = None;
        let mut bring_in = None;
        let mut small_bet = None;
        let mut big_bet = None;
        let mut min_bet = None;
        let mut starting_stacks = None;
        let mut actions = None;
        let mut auther = None;
        let mut event = None;
        let mut url = None;
        let mut venue = None;
        let mut address = None;
        let mut city = None;
        let mut region = None;
        let mut postal_code = None;
        let mut country = None;
        let mut time = None;
        let mut time_zone = None;
        let mut time_zone_abbreviation = None;
        let mut day = None;
        let mut month = None;
        let mut year = None;
        let mut hand = None;
        let mut level = None;
        let mut seats = None;
        let mut seat_count = None;
        let mut table = None;
        let mut players = None;
        let mut finishing_stacks = None;
        let mut winnings = None;
        let mut currency = None;
        let mut currency_symbol = None;
        let mut ante_trimming_status = None;
        let mut time_limit = None;
        let mut time_banks = None;

        for line in phh_str.lines() {
            let mut split = line.split("=");
            let key = split.next().unwrap_or("");
            let key = key.trim();
            let value = split.collect::<Vec<&str>>().join("");
            let value = value.trim();

            if key.is_empty() || value.is_empty() {
                continue;
            }

            match &*key {
                "variant" => {
                    let value = Self::parse_as_str(&value);
                    variant = match value {
                        "FT" => Some(Variant::FT),
                        "NT" => Some(Variant::NT),
                        "NS" => Some(Variant::NS),
                        "PO" => Some(Variant::PO),
                        "FO/8" => Some(Variant::FO8),
                        "F7S" => Some(Variant::F7S),
                        "F7S/8" => Some(Variant::F7S8),
                        "FR" => Some(Variant::FR),
                        "N2L1D" => Some(Variant::N2L1D),
                        "F2L3D" => Some(Variant::F2L3D),
                        "FB" => Some(Variant::FB),
                        _ => None,
                    };
                    if let None = variant {
                        return Err(Error::ParseError("Invalid variant.".to_string()));
                    }
                }
                "antes" => antes = Some(Self::parse_as_array_of_f64(&value)?),
                "blinds_or_straddles" => blinds_or_straddles = Some(Self::parse_as_array_of_f64(&value)?),
                "bring_in" => bring_in = Some(Self::parse_as_f64(&value)?),
                "small_bet" => small_bet = Some(Self::parse_as_f64(&value)?),
                "big_bet" => big_bet = Some(Self::parse_as_f64(&value)?),
                "min_bet" => min_bet = Some(Self::parse_as_f64(&value)?),
                "starting_stacks" => starting_stacks = Some(Self::parse_as_array_of_f64(&value)?),
                "actions" => actions = Some(Self::parse_as_array_of_action(&value)?),
                "auther" => auther = Some(Self::parse_as_string(&value)?),
                "event" => event = Some(Self::parse_as_string(&value)?),
                "url" => url = Some(Self::parse_as_string(&value)?),
                "venue" => venue = Some(Self::parse_as_string(&value)?),
                "address" => address = Some(Self::parse_as_string(&value)?),
                "city" => city = Some(Self::parse_as_string(&value)?),
                "region" => region = Some(Self::parse_as_string(&value)?),
                "postal_code" => postal_code = Some(Self::parse_as_string(&value)?),
                "country" => country = Some(Self::parse_as_string(&value)?),
                "time" => time = Some(Self::parse_as_time(&value)?),
                "time_zone" => time_zone = Some(Self::parse_as_string(&value)?),
                "time_zone_abbreviation" => time_zone_abbreviation = Some(Self::parse_as_string(&value)?),
                "day" => day = Some(Self::parse_as_u32(&value)?),
                "month" => month = Some(Self::parse_as_u32(&value)?),
                "year" => year = Some(Self::parse_as_u32(&value)?),
                "hand" => hand = Some(Self::parse_as_string(&value)?),
                "level" => level = Some(Self::parse_as_u32(&value)?),
                "seats" => seats = Some(Self::parse_as_array_of_u32(&value)?),
                "seat_count" => seat_count = Some(Self::parse_as_u32(&value)?),
                "table" => table = Some(Self::parse_as_string(&value)?),
                "players" => players = Some(Self::parse_as_array_of_string(&value)?),
                "finishing_stacks" => finishing_stacks = Some(Self::parse_as_array_of_f64(&value)?),
                "winnings" => winnings = Some(Self::parse_as_array_of_f64(&value)?),
                "currency" => currency = Some(Self::parse_as_string(&value)?),
                "currency_symbol" => currency_symbol = Some(Self::parse_as_string(&value)?),
                "ante_trimming_status" => ante_trimming_status = Some(Self::parse_as_bool(&value)?),
                "time_limit" => time_limit = Some(Self::parse_as_f64(&value)?),
                "time_banks" => time_banks = Some(Self::parse_as_array_of_f64(&value)?),
                _ => {}
            }
        }

        if let None = variant {
            return Err(Error::ParseError("variant is required.".to_string()));
        }
        let variant = variant.unwrap();

        match variant {
            Variant::FT | Variant::FO8 | Variant::F2L3D | Variant::FB => {
                if let None = blinds_or_straddles {
                    return Err(Error::ParseError(format!("blinds_or_straddles is requried for {}.", variant)));
                }
                if let None = small_bet {
                    return Err(Error::ParseError(format!("samll_bet is required for {}.", variant)));
                }
                if let None = big_bet {
                    return Err(Error::ParseError(format!("samll_bet is required for {}.", variant)));
                }
            }
            Variant::NT | Variant::NS | Variant::PO | Variant::N2L1D => {
                if let None = blinds_or_straddles {
                    return Err(Error::ParseError(format!("blinds_or_straddles is requried for {}.", variant)));
                }
                if let None = min_bet {
                    return Err(Error::ParseError(format!("min_bet is required for {}.",variant)));
                }
            }
            Variant::F7S | Variant::F7S8 | Variant::FR => {
                if let None = bring_in {
                    return Err(Error::ParseError(format!("bring_in is required for {}.",variant)));
                }
                if let None = small_bet {
                    return Err(Error::ParseError(format!("samll_bet is required for {}.", variant)));
                }
                if let None = big_bet {
                    return Err(Error::ParseError(format!("samll_bet is required for {}.", variant)));
                }
            }
        }

        if let None = antes {
            return Err(Error::ParseError("antes is required.".to_string()));
        }
        let antes = antes.unwrap();

        if let None = starting_stacks {
            return Err(Error::ParseError("starting_stack is required.".to_string()));
        }
        let starting_stacks = starting_stacks.unwrap();

        if let None = actions {
            return Err(Error::ParseError("action is required.".to_string()));
        }
        let actions = actions.unwrap();

        Ok(PHH {
            variant,
            antes,
            blinds_or_straddles,
            bring_in,
            small_bet,
            big_bet,
            min_bet,
            starting_stacks,
            actions,
            auther,
            event,
            url,
            venue,
            address,
            city,
            region,
            postal_code,
            country,
            time,
            time_zone,
            time_zone_abbreviation,
            day,
            month,
            year,
            hand,
            level,
            seats,
            seat_count,
            table,
            players,
            finishing_stacks,
            winnings,
            currency,
            currency_symbol,
            ante_trimming_status,
            time_limit,
            time_banks,
        })
    }

    fn parse_as_array(contents: &str) -> Result<Vec<&str>, Error> {
        let mut array = Vec::new();
        if contents.len() < 2 {
            return Err(Error::ParseError("Invalid array.".to_string()));
        }
        let start_char = contents.chars().nth(0).unwrap();
        let end_char = contents.chars().nth(contents.len() - 1).unwrap();
        if start_char != '[' && end_char != ']' {
            return Err(Error::ParseError("Invalid array.".to_string()));
        }
        let values = contents[1..contents.len() - 1].split(',').collect::<Vec<&str>>();
        for value in values {
            array.push(value);
        }
        Ok(array)
    }

    fn parse_as_bool(contents: &str) -> Result<bool, Error> {
        let contents = &contents.to_lowercase()[..];
        match contents {
            "true" => return Ok(true),
            "false" => return Ok(false),
            _ => return Err(Error::ParseError("Invalid bool.".to_string())),
        }
    }

    fn parse_as_str(contents: &str) -> &str {
        let contents = contents.trim();
        if contents.len() < 2 {
            return contents;
        }
        let start_char = contents.chars().nth(0).unwrap();
        if start_char != '\'' && start_char != '"' {
            return contents;
        }
        if contents.chars().nth(contents.len() - 1).unwrap() != start_char {
            return contents;
        }
        &contents[1..contents.len() - 1]
    }

    fn parse_as_array_of_str(contents: &str) -> Result<Vec<&str>, Error> {
        let mut array = Vec::new();
        let values = PHH::parse_as_array(contents)?;
        for value in values {
            array.push(PHH::parse_as_str(value));
        }
        Ok(array)
    }

    fn parse_as_string(contents: &str) -> Result<String, Error> {
        Ok(PHH::parse_as_str(contents).to_string())
    }

    fn parse_as_array_of_string(contents: &str) -> Result<Vec<String>, Error> {
        let mut array = Vec::new();
        let values = PHH::parse_as_array(contents)?;
        for value in values {
            array.push(PHH::parse_as_string(value)?);
        }

        Ok(array)
    }

    fn parse_as_u32(contents: &str) -> Result<u32, Error> {
        if let Ok(value) = contents.trim().parse::<u32>() {
            return Ok(value);
        } else {
            return Err(Error::ParseError("Integer parse error.".to_string()));
        }
    }

    fn parse_as_array_of_u32(contents: &str) -> Result<Vec<u32>, Error> {
        let mut array = Vec::new();
        let values = PHH::parse_as_array(contents)?;
        for value in values {
            array.push(PHH::parse_as_u32(value)?);
        }

        Ok(array)
    }

    fn parse_as_f64(contents: &str) -> Result<f64, Error> {
        if let Ok(value) = contents.trim().parse::<f64>() {
            return Ok(value);
        } else {
            return Err(Error::ParseError("Float parse error.".to_string()));
        }
    }

    fn parse_as_array_of_f64(contents: &str) -> Result<Vec<f64>, Error> {
        let mut array = Vec::new();
        let values = PHH::parse_as_array(contents)?;
        for value in values {
            array.push(PHH::parse_as_f64(value)?);
        }

        Ok(array)
    }

    fn parse_as_time(contents: &str) -> Result<Time, Error> {
        let split = contents.split(':').collect::<Vec<&str>>();
        if split.len() != 3 {
            return Err(Error::ParseError(format!("Invalid time: {}", contents)));
        }

        let hh = PHH::parse_as_u32(split[0])?;
        if 24 <= hh {
            return Err(Error::ParseError(format!("Invalid time: {}", contents)));
        }
        let hh = hh as u8;
        
        let mm = PHH::parse_as_u32(split[1])?;
        if 60 <= mm as u8{
            return Err(Error::ParseError(format!("Invalid time: {}", contents)));
        }
        let mm = mm as u8;
        
        let ss = PHH::parse_as_u32(split[2])?;
        if 60 <= ss {
            return Err(Error::ParseError(format!("Invalid time: {}", contents)));
        }
        let ss = ss as u8;

        return Ok(Time::from_hms(hh, mm, ss).unwrap());
    }

    fn parse_as_cards(contents: &str) -> Result<Vec<Card>, Error> {
        if contents.len() % 2 != 0 {
            return Err(Error::ParseError(format!("Invalid card: {}",contents)));
        }
        let mut array = Vec::with_capacity(contents.len() / 2);
        let mut contents = contents.chars();

        while let Some(rank) = contents.next() {
            let suit = contents.next().unwrap();
            let rank = match rank {
                'A' => Rank::Ace,
                '2' => Rank::Deuce,
                '3' => Rank::Trey,
                '4' => Rank::Four,
                '5' => Rank::Five,
                '6' => Rank::Six,
                '7' => Rank::Seven,
                '8' => Rank::Eight,
                '9' => Rank::Nine,
                'T' => Rank::Ten,
                'J' => Rank::Jack,
                'Q' => Rank::Queen,
                'K' => Rank::King,
                '?' => Rank::Unknown,
                _ => return Err(Error::ParseError(format!("Invalid card: {}{}", suit, rank))),
            };

            let suit = match suit {
                'c' => Suit::Club,
                'd' => Suit::Diamond,
                'h' => Suit::Heart,
                's' => Suit::Spade,
                '?' => Suit::Unknown,
                _ => return Err(Error::ParseError(format!("Invalid card: {}{}", suit, rank))),
            };

            array.push(Card { suit, rank });
        }

        Ok(array)
    }

    fn parse_as_player_number(contents: &str) -> Result<u32, Error> {
        if contents.len() < 2 {
            return Err(Error::ParseError(format!("Invalid player number: {}", contents)));
        }
        if contents.chars().nth(0).unwrap() != 'p' {
            return Err(Error::ParseError(format!("Invalid player number: {}", contents)));
        }
        Ok(PHH::parse_as_u32(&contents[1..])?)
    }


    fn parse_as_action(contents: &str) -> Result<Option<Action>, Error> {
        let contents = contents.trim();
        if contents.is_empty() {
            return Err(Error::ParseError("Invalid action.".to_string()));
        }

        if contents.chars().nth(0).unwrap() == '#' {
            return Ok(None);
        }

        let split = contents.split(' ').collect::<Vec<&str>>();
        if split.len() < 2 {
            return Err(Error::ParseError(format!("Invalid action: {}", contents)));
        }
        let key = split[0];
        let second_key = split[1];
        
        if key == "d" {
            match &*second_key {
                "db" => {
                    if split.len() < 3 {
                        return Err(Error::ParseError(format!("Invalud action: {}", contents)));
                    }
                    let cards = PHH::parse_as_cards(split[2])?;
                    return Ok(Some(Action::DealingCC { cards  }))
                }
                "dh" => {
                    if split.len() < 4 {
                        return Err(Error::ParseError(format!("Invalud action: {}", contents)));
                    }
                    let player = PHH::parse_as_player_number(split[2])?;
                    let cards = PHH::parse_as_cards(split[3])?;
                    return Ok(Some(Action::DealingDUC { player, cards }))
                }
                _ => return Err(Error::ParseError(format!("Invalid action: {}", contents))),
            }
        }

        if key.chars().nth(0).unwrap() == 'p' {
            let player = PHH::parse_as_player_number(key)?;
            let key = second_key;
            match &*key {
                "pb" => return Ok(Some(Action::BringingIn { player })),
                "cbr" => {
                    if split.len() < 3 {
                        return Err(Error::ParseError(format!("Invalid action: {}", contents)));
                    }
                    let amount = PHH::parse_as_f64(split[2])?;
                    return Ok(Some(Action::CBR { player, amount }));
                }
                "cc" => return Ok(Some(Action::CC { player })),
                "f" => return Ok(Some(Action::Folding { player })),
                "sd" => {
                    if split.len() < 3 {
                        return Ok(Some(Action::SD { player, cards: None }));
                    }
                    if split[2].chars().nth(0).unwrap() == '#' {
                        return Ok(Some(Action::SD { player, cards: None }));
                    }
                    let cards = PHH::parse_as_cards(split[2])?;
                    return Ok(Some(Action::SD { player, cards: Some(cards) }));
                }
                "sm" => {
                    if split.len() < 3 {
                        return Ok(Some(Action::SM { player, cards: None }));
                    }
                    if split[2].chars().nth(0).unwrap() == '#' {
                        return Ok(Some(Action::SM { player, cards: None }));
                    }
                    let cards = PHH::parse_as_cards(split[2])?;
                    return Ok(Some(Action::SM { player, cards: Some(cards) }));
                }
                _ => return Err(Error::ParseError(format!("Invalid action: {}", contents))),
            }
        }
        return Err(Error::ParseError(format!("Invalid action: {}", contents)));
    }

    fn parse_as_array_of_action(contents: &str) -> Result<Vec<Action>, Error> {
        let mut array = Vec::new();
        let values = PHH::parse_as_array_of_str(contents)?;
        for value in values {
            let action = PHH::parse_as_action(value)?;
            if let Some(action) = action {
                array.push(action);
            }
        }
        Ok(array)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time;

    #[test]
    fn parse_test() {
        let test_toml = "
variant = \"NT\"
antes = [0, 1, 2]
blinds_or_straddles = [0.0, 2.0, 1.0]
bring_in = 2
small_bet = 1.0
big_bet = 3
min_bet = 2.0
starting_stacks = [4, 3, 2]
actions = [\"d dh p1 7s4c\", \"d dh p2 Jd8h\", \"# PreFlop\", \"d db JhAs9s\", \"p1 pb\" ,\"p2 cbr 100.0\", \"p1 cc\", \"p2 f\", \"p1 sd 7s4s\", \"p2 sm\"]
auther = \"Tom\"
event = \"test' event\"
url = \"https://foobar\"
venue = \"venue value\"
address = \"1-1\"
city = \"Chiyoda-ku\"
region = \"Tokyo\"
postal_code = \"000-0001\"
country = \"Japan\"
time = 12:34:56
time_zone = \"Asia/Tokyo\"
time_zone_abbreviation = \"JST\"
day = 3
month = 3
year = 2026
hand = 33
level = 2
seats = [1, 2]
seat_count = 8
table = 1
players = [\"foo\", \"bar\"]
finishing_stacks = [20.0, 30.0]
winnings = [10.0, 15.0]
currency = \"currency value\"
currency_symbol = \"currency_symbol value\"
ante_trimming_status = true
time_limit = 20.0
time_banks = [20.0, 11.5]
".trim();

        let phh = PHH::parse_from_str(test_toml);

        if let Err(err) = phh {
            match err {
                Error::ParseError(message) => panic!("{}", message),
            }
        }
        if let Ok(phh) = phh {
            assert_eq!(phh.variant, Variant::NT);
            assert_eq!(phh.antes, vec![0.0, 1.0, 2.0]);
            assert_eq!(phh.blinds_or_straddles, Some(vec![0.0, 2.0, 1.0]));
            assert_eq!(phh.bring_in, Some(2.0));
            assert_eq!(phh.small_bet, Some(1.0));
            assert_eq!(phh.big_bet, Some(3.0));
            assert_eq!(phh.min_bet, Some(2.0));
            assert_eq!(phh.starting_stacks, vec![4.0, 3.0, 2.0]);
            let actions = vec![
                Action::DealingDUC {
                    player: 1,
                    cards: vec![
                        Card { rank: Rank::Seven, suit: Suit::Spade },
                        Card { rank: Rank::Four, suit: Suit::Club },
                    ],
                },
                Action::DealingDUC {
                    player: 2,
                    cards: vec![
                        Card { rank: Rank::Jack, suit: Suit::Diamond },
                        Card { rank: Rank::Eight, suit: Suit::Heart },
                    ],
                },
                Action::DealingCC { 
                    cards: vec![
                        Card { rank: Rank::Jack, suit: Suit::Heart },
                        Card { rank: Rank::Ace, suit: Suit::Spade },
                        Card { rank: Rank::Nine, suit: Suit::Spade },
                    ],
                },
                Action::BringingIn { player: 1 },
                Action::CBR { player: 2, amount: 100.0 },
                Action::CC { player: 1 },
                Action::Folding { player: 2 },
                Action::SD {
                    player: 1,
                    cards: Some(vec![
                        Card { rank: Rank::Seven, suit: Suit::Spade },
                        Card { rank: Rank::Four, suit: Suit::Spade },
                    ]),
                },
                Action::SM { player: 2, cards: None },
            ];
            assert_eq!(phh.actions, actions);
            assert_eq!(phh.auther, Some("Tom".to_string()));
            assert_eq!(phh.event, Some("test' event".to_string()));
            assert_eq!(phh.url, Some("https://foobar".to_string()));
            assert_eq!(phh.venue, Some("venue value".to_string()));
            assert_eq!(phh.address, Some("1-1".to_string()));
            assert_eq!(phh.city, Some("Chiyoda-ku".to_string()));
            assert_eq!(phh.region, Some("Tokyo".to_string()));
            assert_eq!(phh.postal_code, Some("000-0001".to_string()));
            assert_eq!(phh.country, Some("Japan".to_string()));
            assert_eq!(phh.time, Some(Time::from_hms(12, 34, 56).unwrap()));
            assert_eq!(phh.time_zone, Some("Asia/Tokyo".to_string()));
            assert_eq!(phh.time_zone_abbreviation, Some("JST".to_string()));
            assert_eq!(phh.day, Some(3));
            assert_eq!(phh.month, Some(3));
            assert_eq!(phh.year, Some(2026));
            assert_eq!(phh.hand, Some("33".to_string()));
            assert_eq!(phh.level, Some(2));
            assert_eq!(phh.seats, Some(vec![1, 2]));
            assert_eq!(phh.seat_count, Some(8));
            assert_eq!(phh.table, Some("1".to_string()));
            assert_eq!(phh.players, Some(vec!["foo".to_string(), "bar".to_string()]));
            assert_eq!(phh.finishing_stacks, Some(vec![20.0, 30.0]));
            assert_eq!(phh.winnings, Some(vec![10.0, 15.0]));
            assert_eq!(phh.currency, Some("currency value".to_string()));
            assert_eq!(phh.currency_symbol, Some("currency_symbol value".to_string()));
            assert_eq!(phh.ante_trimming_status, Some(true));
            assert_eq!(phh.time_limit, Some(20.0));
            assert_eq!(phh.time_banks, Some(vec![20.0, 11.5]));
        }
    }
}
