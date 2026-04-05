# PHH - Poker Hand History Library

[![Crates.io](https://img.shields.io/crates/v/poker_phh.svg)](https://crates.io/crates/poker_phh)
[![Docs.rs](https://docs.rs/poker_phh/badge.svg)](https://docs.rs/poker_phh/)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/surumeika1987/porker_phh#license)

`poker_phh` is a comprehensive Rust library for parsing and representing poker hand histories in the PHH (Poker Hand History) format. PHH is a standardized TOML-based format for storing and exchanging detailed information about poker hands.

For detailed PHH format specifications, see the [PHH Documentation](https://phh.readthedocs.io).

## Features

- **Multi-variant support**: Handles 11 different poker variants including Texas Hold'em, Omaha, Seven Card Stud, Razz, and more
- **Flexible data model**: Supports both required fields (game variant, stakes, actions) and optional metadata (location, timing, player information)
- **Custom fields**: Extensible support for custom fields via underscore-prefixed TOML keys
- **TOML parsing**: Native integration with TOML format for human-readable hand records
- **Complete action tracking**: Represents all poker actions including dealing, betting, checking, folding, and discarding
- **Type-safe**: Leverages Rust's type system for safe and ergonomic API

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
poker_phh = "0.1"
```

## Quick Start

```rust
use std::str::FromStr;
use poker_phh::PHH;

let phh_toml = r#"
variant = "NT"
antes = [0, 1, 2]
blinds_or_straddles = [0, 2, 1]
small_bet = 1
big_bet = 3
min_bet = 2
starting_stacks = [4, 3, 2]
actions = ["d dh p1 7s4c", "d dh p2 Jd8h", "d db JhAs9s", "p1 pb"]
"#;

let hand = PHH::from_str(phh_toml)?;
assert_eq!(hand.variant.to_string(), "NT");
# Ok::<(), poker_phh::Error>(())
```

## Supported Poker Variants

The library supports the following 11 poker variants:

| Code | Variant | Betting |
|------|---------|---------|
| **FT** | Fixed Limit Texas Hold'em | Fixed |
| **NT** | No Limit Texas Hold'em | No Limit |
| **NS** | No Limit Seven Card Stud | No Limit |
| **PO** | Pot Limit Omaha | Pot Limit |
| **FO8** | Fixed Limit Omaha Hi/Lo | Fixed |
| **F7S** | Fixed Limit Seven Card Stud | Fixed |
| **F7S8** | Fixed Limit Seven Card Stud Hi/Lo | Fixed |
| **FR** | Fixed Limit Razz | Fixed |
| **N2L1D** | No Limit 2-7 Lowball Single Draw | No Limit |
| **F2L3D** | Fixed Limit 2-7 Triple Draw | Fixed |
| **FB** | Fixed Limit Badugi | Fixed |

## Core Types

- [`PHH`]: The main structure representing a complete poker hand
- [`Variant`]: Enumeration of supported poker game variants
- [`Action`]: Represents individual actions taken during a hand
- [`Card`]: Represents a playing card with rank and suit
- [`Error`]: Error type for parsing and validation failures

## PHH Format

The PHH format uses TOML to represent poker hand histories. Here's a comprehensive example:

```toml
# Required fields
variant = "NT"
antes = [0, 1, 2]
blinds_or_straddles = [0, 2, 1]
small_bet = 1
big_bet = 3
min_bet = 2
starting_stacks = [100, 200, 150]
actions = ["d dh p1 7s4c", "d dh p2 JdQh", "d db KhAs9s", "p1 b 10"]

# Optional metadata
author = "Alice"
event = "Weekly Poker Game"
venue = "Local Poker Club"
address = "123 Main St"
city = "Anytown"
region = "State"
postal_code = "12345"
country = "Country"

# Timing information
time = 19:30:00
time_zone = "America/New_York"
time_zone_abbreviation = "EST"
day = 15
month = 6
year = 2024

# Game information
hand = "42"
level = 3
seats = [1, 2, 3]
seat_count = 6
table = "1"
players = ["Alice", "Bob", "Charlie"]

# Results
finishing_stacks = [150, 250, 50]
winnings = [50, 150, -100]
currency = "USD"
currency_symbol = "$"

# Advanced settings
bring_in = 2
ante_trimming_status = true
time_limit = 30
time_banks = [120, 120, 120]
```

### Action Format

Actions in PHH follow a specific string-based format. For complete details, see the [PHH Documentation](https://phh.readthedocs.io).

- `d dh pN <cards>` - Dealing hole cards to player N
- `d db <cards>` - Dealing board cards (community cards)
- `pN pb` - Player N posts blind
- `pN b <amount>` - Player N bets an amount
- `pN cb <amount>` - Player N check-bets an amount
- `pN cbr <amount>` - Player N check-bet raises an amount
- `pN cc` - Player N calls
- `pN r <amount>` - Player N raises
- `pN f` - Player N folds
- `pN c` - Player N checks
- `pN sd <cards>` - Player N stands pat (no discard)
- `pN sm` - Player N shows/mucks

## Use Cases

- **Hand analysis**: Store and replay poker hands for analysis
- **Game statistics**: Track player performance across multiple hands
- **Hand comparison**: Compare strategy decisions across different variants
- **Educational material**: Create standardized hand records for teaching
- **Tournament data**: Record and analyze tournament hands

## Documentation

For detailed API documentation, visit [docs.rs](https://docs.rs/poker_phh/).

For PHH format specification, visit [PHH Documentation](https://phh.readthedocs.io).

To generate local documentation:

```bash
cargo doc --open
```

## Testing

Run the test suite:

```bash
cargo test
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-Apache](LICENSE-Apache) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Repository

- [GitHub](https://github.com/surumeika1987/porker_phh)
