[![crates.io](https://img.shields.io/crates/v/social_tournament.svg)](https://crates.io/crates/social_tournament)
[![Documentation](https://docs.rs/social_tournament/badge.svg)](https://docs.rs/social_tournament)
[![Workflow](https://github.com/flocksserver/social_tournament/workflows/Rust/badge.svg)](https://github.com/flocksserver/social_tournament/workflows/Rust/badge.svg)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)


# Social Tournament

This is a library for creating tournament schedules for the sport I love.

🏓  table tennis 🏓

The focus is on meeting as many opponents and teammates as possible during the tournament. One can draw a single or double tournament.

## Example

Get rounds for a single tournament (round robin)
```rust
use social_tournament::model::RoundSingles;
use social_tournament::single::draw_singles;

let tournament: Vec<RoundSingles> = draw_singles(10,9);
/*
Creates:
Round number: 0
SingleMatch { a: 0, b: 9 }
SingleMatch { a: 1, b: 8 }
SingleMatch { a: 2, b: 7 }
SingleMatch { a: 3, b: 6 }
SingleMatch { a: 4, b: 5 }
--------------
Round number: 1
SingleMatch { a: 1, b: 9 }
SingleMatch { a: 2, b: 0 }
SingleMatch { a: 3, b: 8 }
SingleMatch { a: 4, b: 7 }
SingleMatch { a: 5, b: 6 }
--------------
...
*/ 
```

If you want to get rounds for a double tournament you have to do the following:
```rust
use social_tournament::model::RoundDoubles;
use social_tournament::double::{draw_doubles, DrawOption};

let tournament: Vec<RoundDoubles> = draw_doubles(39, 12, Some(DrawOption::ForceDoubleOnly));
/*
Creates:
Round number: 0
DoubleMatch { double_a: (2, 37), double_b: (1, 38) }
DoubleMatch { double_a: (3, 36), double_b: (4, 35) }
DoubleMatch { double_a: (5, 34), double_b: (6, 33) }
DoubleMatch { double_a: (7, 32), double_b: (8, 31) }
DoubleMatch { double_a: (9, 30), double_b: (10, 29) }
DoubleMatch { double_a: (11, 28), double_b: (12, 27) }
DoubleMatch { double_a: (13, 26), double_b: (14, 25) }
DoubleMatch { double_a: (15, 24), double_b: (16, 23) }
DoubleMatch { double_a: (17, 22), double_b: (18, 21) }
--------------
Round number: 1
DoubleMatch { double_a: (2, 37), double_b: (1, 38) }
DoubleMatch { double_a: (3, 36), double_b: (4, 35) }
DoubleMatch { double_a: (5, 34), double_b: (6, 33) }
DoubleMatch { double_a: (7, 32), double_b: (8, 31) }
DoubleMatch { double_a: (9, 30), double_b: (10, 29) }
DoubleMatch { double_a: (11, 28), double_b: (12, 27) }
DoubleMatch { double_a: (13, 26), double_b: (14, 25) }
DoubleMatch { double_a: (15, 24), double_b: (16, 23) }
DoubleMatch { double_a: (17, 22), double_b: (18, 21) }
--------------
...
*/
```
### Double Options
For number of players that are not completely divisible by 4 you can choose between three `DrawOption`.
Depending on the selected option you can have doubles with only 3 players, single matches or player with byes. You have to make sure that the player ids >= `number_of_players` in the schedule post processed correctly. So that you can mark them as byes for example.


## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.