//! Small crate to create a single tournament.
//!
//! Provides an interface for passing the number of players to participate in the tournament
//! and the number of rounds you would like to play. It returns a list of [RoundSingles].
//! # Example
//! ```
//! use social_tournament::model::RoundSingles;
//! use social_tournament::single::draw_singles;
//!
//! let tournament: Vec<RoundSingles> = draw_singles(10, 9);
//! ```
//!

use crate::model::*;
use round_robin_tournament::round_robin_tournament::draw;

/// Public interface to create the single tournament.
///
/// For a given `number_of_players` and `number_of_rounds` it returns a schedule of Rounds
/// with the corresponding matches. For `number_of_rounds` < `number_of_players` the round
/// robin algorithm ensures, that one does not face an opponents twice. For
/// `number_of_rounds` >= `number_of_players` the round robin is calculated one more round.
/// For an odd number of players, the algorithm calculates with `number_of_players` + 1.
/// So you have to make sure that the player who plays against the highest number has a bye.
/// # Example
/// ```
/// use social_tournament::model::RoundSingles;
/// use social_tournament::single::draw_singles;
///
/// let tournament: Vec<RoundSingles> = draw_singles(10, 9);
/// /*
/// Creates:
/// Round number: 0
/// SingleMatch { a: 0, b: 9 }
/// SingleMatch { a: 1, b: 8 }
/// SingleMatch { a: 2, b: 7 }
/// SingleMatch { a: 3, b: 6 }
/// SingleMatch { a: 4, b: 5 }
/// --------------
/// Round number: 1
/// SingleMatch { a: 1, b: 9 }
/// SingleMatch { a: 2, b: 0 }
/// SingleMatch { a: 3, b: 8 }
/// SingleMatch { a: 4, b: 7 }
/// SingleMatch { a: 5, b: 6 }
/// --------------
/// ...
/// */
/// ```
pub fn draw_singles(number_of_players: usize, number_of_rounds: usize) -> Vec<RoundSingles> {
    let round_robin_pairs = draw(number_of_players);
    let mut rounds: Vec<RoundSingles> = Vec::new();

    let mut iter_rounds = round_robin_pairs.iter().peekable();
    for i in 0..number_of_rounds {
        let mut iter_clone = iter_rounds.clone();
        let r = if iter_clone.peek().is_some() {
            iter_rounds.next().unwrap()
        }else {
            iter_rounds = round_robin_pairs.iter().peekable();
            iter_rounds.next().unwrap()
        };
        let mut matches = Vec::new();
        r.iter().for_each(|p|{
            matches.push(SingleMatch{ a: p.0, b: p.1 })
        });
        rounds.push(RoundSingles { round_number: i, matches });
    }
    rounds
}

#[cfg(test)]
mod tests {
    use crate::single::draw_singles;
    use crate::model::SingleMatch;

    #[test]
    fn draw_20_12() {
        let number_of_players = 20;
        let number_of_rounds = 12;

        let rounds = draw_singles(number_of_players, number_of_rounds);

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), number_of_players / 2);
            for i in 0..number_of_players {
                let fp = r.matches.iter().filter(|p| {
                    p.a == i || p.b == i
                }).collect::<Vec<&SingleMatch>>();
                assert_eq!(fp.len(), 1);
            }
        });
    }
    #[test]
    fn draw_19_12() {
        let number_of_players = 19;
        let number_of_rounds = 12;

        let rounds = draw_singles(number_of_players, number_of_rounds);

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), (number_of_players + 1) / 2);
            for i in 0..(number_of_players + 1) {
                let fp = r.matches.iter().filter(|p| {
                    p.a == i || p.b == i
                }).collect::<Vec<&SingleMatch>>();
                assert_eq!(fp.len(), 1);
            }
        });
    }
}