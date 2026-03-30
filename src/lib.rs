use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use serde::de::Unexpected;
use time::Time;
use toml::Table;
use serde::{de::{self, Deserialize, Deserializer, Visitor}};

#[derive(Debug)]
pub enum Error {
    ParseError(String),
}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Self::ParseError(value.message().to_string())
    }
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

impl FromStr for Variant {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FT" => Ok(Variant::FT),
            "NT" => Ok(Variant::NT),
            "NS" => Ok(Variant::NS),
            "PO" => Ok(Variant::PO),
            "FO/8" => Ok(Variant::FO8),
            "F7S" => Ok(Variant::F7S),
            "F7S/8" => Ok(Variant::F7S8),
            "FR" => Ok(Variant::FR),
            "N2L1D" => Ok(Variant::N2L1D),
            "F2L3D" => Ok(Variant::F2L3D),
            "FB" => Ok(Variant::FB),
            _ => Err(Error::ParseError(format!("Invalid variant: {}", s))),
        }
    }
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

impl<'de> Deserialize<'de> for Variant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>
    {
        struct VariantVisitor;

        impl<'de> Visitor<'de> for VariantVisitor {
            type Value = Variant;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("PHH variant")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> 
                where
                    E: de::Error,
            {
                match Variant::from_str(v) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(de::Error::invalid_value(Unexpected::Str(v), &self)),
                }
            }
        }

        deserializer.deserialize_str(VariantVisitor)
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

impl FromStr for Suit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(Error::ParseError(format!("Invalid suit: {}", s)));
        }

        match s.chars().nth(0).unwrap() {
            'c' => Ok(Suit::Club),
            'd' => Ok(Suit::Diamond),
            'h' => Ok(Suit::Heart),
            's' => Ok(Suit::Spade),
            '?' => Ok(Suit::Unknown),
            _ => Err(Error::ParseError(format!("Invalid suit: {}", s))),
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

impl FromStr for Rank {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(Error::ParseError(format!("Invalid Rank: {}", s)));
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
            _ => Err(Error::ParseError(format!("Invalid Rank: {}", s))),
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

#[derive(Debug, PartialEq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(Error::ParseError(format!("Invalid card: {}", s)));
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
    fn from_str_cards(s: &str) -> Result<Vec<Self>, Error> {
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

#[derive(Debug, PartialEq)]
pub enum Action {
    /// Dealing community cards
    DealingCC { cards: Vec<Card> },
    /// Dealing down/up cards
    DealingDUC { player: u32, cards: Vec<Card> },
    /// Biringing in
    BringingIn { player: u32 },
    /// Completing/Betting/Raising
    CBR { player: u32, amount: f64 },
    /// Checking/Calling
    CC { player: u32 },
    /// Folding
    Folding { player: u32 },
    /// Standing pat/Discarding,
    SD { player: u32, cards: Option<Vec<Card>> },
    /// Showing/Mucking their hole cards
    SM { player: u32, cards: Option<Vec<Card>> },
    /// Comment
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
            return Err(Error::ParseError(format!("Invalid action: {}", s)));
        }
        let key = split[0];
        let secound_key = split[1];

        if key == "d" {
            match &*secound_key {
                "db" => {
                    if split.len() < 3 {
                        return Err(Error::ParseError(format!("Invalid action: {}", s)));
                    }
                    let cards = Card::from_str_cards(split[2])?;
                    return Ok(Action::DealingCC { cards })
                }
                "dh" => {
                    if split.len() < 4 {
                        return Err(Error::ParseError(format!("Invalid action: {}", s)));
                    }
                    let player = Self::parse_as_player_number(split[2])?;
                    let cards = Card::from_str_cards(split[3])?;
                    return Ok(Action::DealingDUC { player, cards });
                }
                _ => return Err(Error::ParseError(format!("Invalid action: {}", s))),
            }
        }

        if key.starts_with("p") {
            let player = Self::parse_as_player_number(key)?;
            let key = secound_key;
            match &*key {
                "pb" => return Ok(Action::BringingIn { player }),
                "cbr" => {
                    if split.len() < 3 {
                        return Err(Error::ParseError(format!("Invalid action: {}", s)));
                    }
                    let amount = split[2].parse::<f64>();
                    if let Ok(amount) = amount {
                        return Ok(Action::CBR { player, amount });
                    } else {
                        return Err(Error::ParseError(format!("Invalid amount: {}", s)));
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
                _ => return Err(Error::ParseError(format!("Invalid action: {}", s))),
            }
        }

        return Err(Error::ParseError(format!("Invalid action: {}", s)));
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

#[derive(Debug, PartialEq)]
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

impl FromStr for PHH {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

        let data = s.parse::<Table>();
        if let Err(err) = data {
            return Err(Error::ParseError(err.message().to_string()));
        }
        let data = data.unwrap();
        
        for key in data.keys() {
            let value = data[key].clone();
            match &**key {
                "variant" => variant = Some(value.try_into()?),
                "antes" => antes = Some(value.try_into()?),
                "blinds_or_straddles" => blinds_or_straddles = Some(value.try_into()?),
                "bring_in" => bring_in = Some(value.try_into()?),
                "small_bet" => small_bet = Some(value.try_into()?),
                "big_bet" => big_bet = Some(value.try_into()?),
                "min_bet" => min_bet = Some(value.try_into()?),
                "starting_stacks" => starting_stacks = Some(value.try_into()?),
                "actions" => actions = Some(value.try_into()?),
                "auther" => auther = Some(value.try_into()?),
                "event" => event = Some(value.try_into()?),
                "url" => url = Some(value.try_into()?),
                "venue" => venue = Some(value.try_into()?),
                "address" => address = Some(value.try_into()?),
                "city" => city = Some(value.try_into()?),
                "region" => region = Some(value.try_into()?),
                "postal_code" => postal_code = Some(value.try_into()?),
                "country" => country = Some(value.try_into()?),
                "time" => time = Some(PHH::parse_as_time(&value)?),
                "time_zone" => time_zone = Some(value.try_into()?),
                "time_zone_abbreviation" => time_zone_abbreviation = Some(value.try_into()?),
                "day" => day = Some(value.try_into()?),
                "month" => month = Some(value.try_into()?),
                "year" => year = Some(value.try_into()?),
                "hand" => hand = Some(value.try_into()?),
                "level" => level = Some(value.try_into()?),
                "seats" => seats = Some(value.try_into()?),
                "seat_count" => seat_count = Some(value.try_into()?),
                "table" => table = Some(value.try_into()?),
                "players" => players = Some(value.try_into()?),
                "finishing_stacks" => finishing_stacks = Some(value.try_into()?),
                "winnings" => winnings = Some(value.try_into()?),
                "currency" => currency = Some(value.try_into()?),
                "currency_symbol" => currency_symbol = Some(value.try_into()?),
                "ante_trimming_status" => ante_trimming_status = Some(value.try_into()?),
                "time_limit" => time_limit = Some(value.try_into()?),
                "time_banks" => time_banks = Some(value.try_into()?),
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
}

impl Display for PHH {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut array = Vec::new();
        array.push(format!("variant = \"{}\"", self.variant));
        array.push(format!("antes = {}", PHH::array_to_string(&self.antes)));

        if let Some(blinds_or_straddles) = &self.blinds_or_straddles {
            array.push(format!("blinds_or_straddles = {}", PHH::array_to_string(blinds_or_straddles)));
        }
        if let Some(bring_in) = self.bring_in {
            array.push(format!("bring_in = {}", bring_in));
        }
        if let Some(small_bet) = self.small_bet {
            array.push(format!("small_bet = {}", small_bet));
        }
        if let Some(big_bet) = self.big_bet {
            array.push(format!("big_bet = {}", big_bet));
        }
        if let Some(min_bet) = self.min_bet {
            array.push(format!("min_bet = {}", min_bet));
        }
        array.push(format!("starting_stacks = {}", PHH::array_to_string(&self.starting_stacks)));
        let actions_strs: Vec<String> = self.actions.iter().map(|a| a.to_string()).collect();
        array.push(format!("actions = {}", PHH::array_to_string_with_quotes(&actions_strs)));

        if let Some(auther) = &self.auther {
            array.push(format!("auther = \"{}\"", auther));
        }
        if let Some(event) = &self.event {
            array.push(format!("event = \"{}\"", event));
        }
        if let Some(url) = &self.url {
            array.push(format!("url = \"{}\"", url));
        }
        if let Some(venue) = &self.venue {
            array.push(format!("venue = \"{}\"", venue));
        }
        if let Some(address) = &self.address {
            array.push(format!("address = \"{}\"", address));
        }
        if let Some(city) = &self.city {
            array.push(format!("city = \"{}\"", city));
        }
        if let Some(region) = &self.region {
            array.push(format!("region = \"{}\"", region));
        }
        if let Some(postal_code) = &self.postal_code {
            array.push(format!("postal_code = \"{}\"", postal_code));
        }
        if let Some(country) = &self.country {
            array.push(format!("country = \"{}\"", country));
        }
        if let Some(time) = self.time {
            array.push(format!("time = {}", PHH::time_to_string(time)));
        }
        if let Some(time_zone) = &self.time_zone {
            array.push(format!("time_zone = \"{}\"", time_zone));
        }
        if let Some(time_zone_abbreviation) = &self.time_zone_abbreviation {
            array.push(format!("time_zone_abbreviation = \"{}\"", time_zone_abbreviation));
        }
        if let Some(day) = self.day {
            array.push(format!("day = {}", day));
        }
        if let Some(month) = self.month {
            array.push(format!("month = {}", month));
        }
        if let Some(year) = self.year {
            array.push(format!("year = {}", year));
        }
        if let Some(hand) = &self.hand {
            array.push(format!("hand = \"{}\"", hand));
        }
        if let Some(level) = self.level {
            array.push(format!("level = {}", level));
        }
        if let Some(seats) = &self.seats {
            array.push(format!("seats = {}", PHH::array_to_string(seats)));
        }
        if let Some(seat_count) = self.seat_count {
            array.push(format!("seat_count = {}", seat_count));
        }
        if let Some(table) = &self.table {
            array.push(format!("table = \"{}\"", table));
        }
        if let Some(players) = &self.players {
            array.push(format!("players = {}", PHH::array_to_string_with_quotes(players)));
        }
        if let Some(finishing_stacks) = &self.finishing_stacks {
            array.push(format!("finishing_stacks = {}", PHH::array_to_string(finishing_stacks)));
        }
        if let Some(winnings) = &self.winnings {
            array.push(format!("winnings = {}", PHH::array_to_string(winnings)));
        }
        if let Some(currency) = &self.currency {
            array.push(format!("currency = \"{}\"", currency));
        }
        if let Some(currency_symbol) = &self.currency_symbol {
            array.push(format!("currency_symbol = \"{}\"", currency_symbol));
        }
        if let Some(ante_trimming_status) = self.ante_trimming_status {
            array.push(format!("ante_trimming_status = {}", ante_trimming_status));
        }
        if let Some(time_limit) = self.time_limit {
            array.push(format!("time_limit = {}", time_limit));
        }
        if let Some(time_banks) = &self.time_banks {
            array.push(format!("time_banks = {}", PHH::array_to_string(time_banks)));
        }
        
        write!(f, "{}", array.join("\n"))

    }
}

impl PHH {
    fn parse_as_time(value: &toml::Value) -> Result<Time, Error> {
        if let toml::Value::Datetime(t) = value {
            if let Some(t) = t.time {
                return Ok(Time::from_hms(t.hour, t.minute, t.second.unwrap_or_default()).unwrap());
            } else {
                return Err(Error::ParseError(format!("Invalid time: {}", value.to_string())));
            }
        }
        return Err(Error::ParseError(format!("Invalid time: {}", value.to_string())));
    }

    fn time_to_string(t: Time) -> String{
        format!("{}:{}:{}", t.hour(), t.minute(), t.second())
    }

    fn array_to_string<T: ToString>(array: &Vec<T>) -> String {
        let array: Vec<String> = array.iter().map(|f| f.to_string()).collect();
        format!("[{}]", array.join(","))
    }

    fn array_to_string_with_quotes<T: ToString>(array: &Vec<T>) -> String {
        let array: Vec<String> = array.iter().map(|f| format!("\"{}\"", f.to_string())).collect();
        format!("[{}]", array.join(","))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_toml = r#"
variant = "NT"
antes = [0,1,2]
blinds_or_straddles = [0,2,1]
bring_in = 2
small_bet = 1
big_bet = 3
min_bet = 2
starting_stacks = [4,3,2]
actions = ["d dh p1 7s4c","d dh p2 Jd8h","d db JhAs9s","p1 pb","p2 cbr 100","p1 cc","p2 f","p1 sd 7s4s","p2 sm"]
auther = "Tom"
event = "test' event"
url = "https://foobar"
venue = "venue value"
address = "1-1"
city = "Chiyoda-ku"
region = "Tokyo"
postal_code = "000-0001"
country = "Japan"
time = 12:34:56
time_zone = "Asia/Tokyo"
time_zone_abbreviation = "JST"
day = 3
month = 3
year = 2026
hand = "33"
level = 2
seats = [1,2]
seat_count = 8
table = "1"
players = ["foo","bar"]
finishing_stacks = [20,30]
winnings = [10,15]
currency = "currency value"
currency_symbol = "currency_symbol value"
ante_trimming_status = true
time_limit = 20
time_banks = [20,11.5]
"#.trim();

        let phh = PHH::from_str(test_toml);

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

            let ex_phh = PHH::from_str(&phh.to_string()[..]).unwrap();
            assert_eq!(phh, ex_phh);
        }
    }
}
