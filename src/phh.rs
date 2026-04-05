use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use std::collections::HashSet;
use toml::{Table, map::Map};
use crate::action::Action;
use crate::error::Error;
use crate::time::PHHTime;
use crate::value::PHHCustomValue;
use crate::variant::Variant;

/// Represents a complete poker hand in PHH (Poker Hand History) format.
///
/// This is the main data structure that contains all information about a poker hand,
/// including game variant, stakes, player stacks, and all actions taken during the hand.
/// The PHH format is designed to store and exchange poker hand history data in a
/// standardized TOML-based format.
///
/// # Required Fields
/// - `variant`: The poker game variant
/// - `antes`: Ante amounts for each player
/// - `starting_stacks`: Starting chip counts for each player
/// - `actions`: All actions taken in the hand
///
/// # Optional Fields
/// Various optional fields store additional context about the game, such as:
/// - Game stakes (`blinds_or_straddles`, `bring_in`, `small_bet`, `big_bet`, `min_bet`)
/// - Game location and timing information
/// - Player information and results
#[derive(Debug, PartialEq, Clone)]
pub struct PHH {
    /// The poker game variant played
    pub variant: Variant,
    /// Ante amounts for each player (required)
    pub antes: Vec<f64>,
    /// Blind or straddle amounts for each position
    pub blinds_or_straddles: Option<Vec<f64>>,
    /// Bring-in amount (for stud games)
    pub bring_in: Option<f64>,
    /// Small bet amount (for fixed limit games)
    pub small_bet: Option<f64>,
    /// Big bet amount (for fixed limit games)
    pub big_bet: Option<f64>,
    /// Minimum bet amount
    pub min_bet: Option<f64>,
    /// Starting chip stacks for each player (required)
    pub starting_stacks: Vec<f64>,
    /// Sequence of all actions in the hand (required)
    pub actions: Vec<Action>,
    /// Name of the person who recorded the hand
    pub auther: Option<String>,
    /// Name or identifier of the event/tournament
    pub event: Option<String>,
    /// URL link to the original hand history or source
    pub url: Option<String>,
    /// Venue name where the game took place
    pub venue: Option<String>,
    /// Street address of the venue
    pub address: Option<String>,
    /// City where the game took place
    pub city: Option<String>,
    /// Region/state where the game took place
    pub region: Option<String>,
    /// Postal code of the venue
    pub postal_code: Option<String>,
    /// Country where the game took place
    pub country: Option<String>,
    /// Time the hand started
    pub time: Option<PHHTime>,
    /// Time zone identifier (e.g., "America/New_York")
    pub time_zone: Option<String>,
    /// Time zone abbreviation (e.g., "EST")
    pub time_zone_abbreviation: Option<String>,
    /// Day of the month (1-31)
    pub day: Option<u32>,
    /// Month (1-12)
    pub month: Option<u32>,
    /// Year
    pub year: Option<u32>,
    /// Hand number or identifier
    pub hand: Option<String>,
    /// Tournament level or blind level
    pub level: Option<u32>,
    /// Seat numbers of the players
    pub seats: Option<Vec<u32>>,
    /// Total number of players at the table
    pub seat_count: Option<u32>,
    /// Table number or identifier
    pub table: Option<String>,
    /// Names or identifiers of the players
    pub players: Option<Vec<String>>,
    /// Final chip stacks for each player
    pub finishing_stacks: Option<Vec<f64>>,
    /// Winnings or losses for each player
    pub winnings: Option<Vec<f64>>,
    /// Currency used (e.g., "USD", "EUR")
    pub currency: Option<String>,
    /// Currency symbol (e.g., "$", "€")
    pub currency_symbol: Option<String>,
    /// Whether ante trimming was applied
    pub ante_trimming_status: Option<bool>,
    /// Time limit per action (in seconds)
    pub time_limit: Option<f64>,
    /// Time bank amounts for each player (in seconds)
    pub time_banks: Option<Vec<f64>>,
    /// Custom fields prefixed with underscore in the source file
    pub custom_field: Map<String, PHHCustomValue>,
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
        let mut custom_field = Map::new();

        let data = s.parse::<Table>();
        if let Err(err) = data {
            return Err(Error::ParseError(err.message().to_string()));
        }
        let data = data.unwrap();

        let mut seen_keys = HashSet::new();
        
        for key in data.keys() {
            if !seen_keys.insert(key) {
                return Err(Error::ParseError(format!("Duplicate key: {}", key)));
            }
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
                "time" => time = Some(PHHTime::try_from(value)?),
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
                _ => {
                    if key.starts_with('_') {
                        custom_field.insert(key[1..].to_string(), PHHCustomValue::try_from(value)?);
                    }
                }
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
            custom_field,
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
        if let Some(time) = &self.time {
            array.push(format!("time = {}", time));
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
    fn array_to_string<T: ToString>(array: &Vec<T>) -> String {
        let array: Vec<String> = array.iter().map(|f| f.to_string()).collect();
        format!("[{}]", array.join(","))
    }

    fn array_to_string_with_quotes<T: ToString>(array: &Vec<T>) -> String {
        let array: Vec<String> = array.iter().map(|f| format!("\"{}\"", f.to_string())).collect();
        format!("[{}]", array.join(","))
    }
}
