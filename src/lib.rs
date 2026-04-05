pub mod error;
pub mod time;
pub mod value;
pub mod variant;
pub mod card;
pub mod action;
pub mod phh;

pub use error::Error;
pub use time::PHHTime;
pub use value::PHHCustomValue;
pub use variant::Variant;
pub use card::{Card, Rank, Suit};
pub use action::Action;
pub use phh::PHH;

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use ::time::Time;

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
            assert_eq!(phh.time, Some(PHHTime(Time::from_hms(12, 34, 56).unwrap())));
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
