//! Small crate to distribute rounds and matches to tables in a room or gym.
//!
//! Provides an interface to pass the drawn tournament and your available tables.
//!
//! # Example
//! ```
//! use social_tournament::double::{draw_doubles, RoundDoubles};
//! use social_tournament::table::{Table, distribute_tables_doubles};
//!
//! let tournament: Vec<RoundDoubles> = draw_doubles(24, 2, None);
//! let tables: Vec<Vec<Table>> = distribute_tables_doubles(&tournament, 4);
//! ```
//!

use crate::double::RoundDoubles;
use crate::single::RoundSingles;

#[derive(Debug, Clone)]
pub struct Table {
    /// A table has a numerical value to identify it in the gym or room.
    table_number: usize,
    /// If there are more games than tables, a round must be played in several sub-rounds.
    /// Therefore, a table can be occupied in sub-rounds 0, 1, ...
    occupied_number: usize,
}

/// Public interface to distribute the tables for the double tournament.
///
/// Provides an interface to pass the drawn tournament [RoundDoubles] and the number of your
/// `available_tables`. Sometimes you may have more tables, but not enough space. Specify how
/// many [Table] you can provide for the tournament in your room or gym. The algorithm ensures
/// that enough sub-rounds are formed.
///
/// # Example
/// ```
/// use social_tournament::double::{draw_doubles, RoundDoubles};
/// use social_tournament::table::{Table, distribute_tables_doubles};
///
/// let tournament: Vec<RoundDoubles> = draw_doubles(24, 2, None);
/// let tables: Vec<Vec<Table>> = distribute_tables_doubles(&tournament, 4);
/// /*
/// Creates:
/// Table { table_number: 0, occupied_number: 0 }
/// Table { table_number: 1, occupied_number: 0 }
/// Table { table_number: 2, occupied_number: 0 }
/// Table { table_number: 3, occupied_number: 0 }
/// Table { table_number: 0, occupied_number: 1 }
/// Table { table_number: 1, occupied_number: 1 }
/// --------------
/// Table { table_number: 0, occupied_number: 0 }
/// Table { table_number: 1, occupied_number: 0 }
/// Table { table_number: 2, occupied_number: 0 }
/// Table { table_number: 3, occupied_number: 0 }
/// Table { table_number: 0, occupied_number: 1 }
/// Table { table_number: 1, occupied_number: 1 }
/// --------------
/// */
/// ```
///
pub fn distribute_tables_doubles(tournament: &Vec<RoundDoubles>, available_tables: usize) -> Vec<Vec<Table>> {
    let mut tables: Vec<Vec<Table>> = Vec::new();
    tournament.iter().for_each(|r| {
        let mut tables_for_current_round: Vec<Table> = Vec::new();
        for i in r.matches.iter().enumerate() {
            tables_for_current_round.push(Table { table_number: i.0 % available_tables, occupied_number: i.0 / available_tables })
        }
        tables.push(tables_for_current_round);
    });
    tables
}


/// Public interface to distribute the tables for the single tournament.
///
/// Provides an interface to pass the drawn tournament [RoundDoubles] and the number of your
/// `available_tables`. Sometimes you may have more tables, but not enough space. Specify how
/// many [Table] you can provide for the tournament in your room or gym. The algorithm ensures
/// that enough sub-rounds are formed.
///
/// # Example
/// ```
/// use social_tournament::single::{draw_singles, RoundSingles};
/// use social_tournament::table::{Table, distribute_tables_singles};
///
/// let tournament: Vec<RoundSingles> = draw_singles(12, 2);
/// let tables: Vec<Vec<Table>> = distribute_tables_singles(&tournament, 4);
/// /*
/// Creates:
/// Table { table_number: 0, occupied_number: 0 }
/// Table { table_number: 1, occupied_number: 0 }
/// Table { table_number: 2, occupied_number: 0 }
/// Table { table_number: 3, occupied_number: 0 }
/// Table { table_number: 0, occupied_number: 1 }
/// Table { table_number: 1, occupied_number: 1 }
/// --------------
/// Table { table_number: 0, occupied_number: 0 }
/// Table { table_number: 1, occupied_number: 0 }
/// Table { table_number: 2, occupied_number: 0 }
/// Table { table_number: 3, occupied_number: 0 }
/// Table { table_number: 0, occupied_number: 1 }
/// Table { table_number: 1, occupied_number: 1 }
/// --------------
/// */
/// ```
///
pub fn distribute_tables_singles(tournament: &Vec<RoundSingles>, available_tables: usize) -> Vec<Vec<Table>> {
    let mut tables: Vec<Vec<Table>> = Vec::new();
    tournament.iter().for_each(|r| {
        let mut tables_for_current_round: Vec<Table> = Vec::new();
        for i in r.matches.iter().enumerate() {
            tables_for_current_round.push(Table { table_number: i.0 % available_tables, occupied_number: i.0 / available_tables })
        }
        tables.push(tables_for_current_round);
    });
    tables
}


#[cfg(test)]
mod tests {
    use crate::table::{distribute_tables_doubles};
    use crate::double::{RoundDoubles, DoubleMatch};

    fn get_data() -> Vec<RoundDoubles> {
        vec![
            RoundDoubles {
                round_number: 0,
                matches: vec![
                    DoubleMatch { double_a: (2, 37), double_b: (1, 38) },
                    DoubleMatch { double_a: (3, 36), double_b: (4, 35) },
                    DoubleMatch { double_a: (5, 34), double_b: (6, 33) },
                    DoubleMatch { double_a: (7, 32), double_b: (8, 31) },
                    DoubleMatch { double_a: (9, 30), double_b: (10, 29) },
                    DoubleMatch { double_a: (11, 28), double_b: (12, 27) },
                    DoubleMatch { double_a: (13, 26), double_b: (14, 25) },
                    DoubleMatch { double_a: (15, 24), double_b: (16, 23) },
                    DoubleMatch { double_a: (17, 22), double_b: (18, 21) },
                ],
            },
            RoundDoubles {
                round_number: 1,
                matches: vec![
                    DoubleMatch { double_a: (20, 21), double_b: (2, 0) },
                    DoubleMatch { double_a: (3, 38), double_b: (7, 34) },
                    DoubleMatch { double_a: (4, 37), double_b: (6, 35) },
                    DoubleMatch { double_a: (5, 36), double_b: (9, 32) },
                    DoubleMatch { double_a: (8, 33), double_b: (10, 31) },
                    DoubleMatch { double_a: (11, 30), double_b: (15, 26) },
                    DoubleMatch { double_a: (12, 29), double_b: (14, 27) },
                    DoubleMatch { double_a: (13, 28), double_b: (17, 24) },
                    DoubleMatch { double_a: (16, 25), double_b: (18, 23) },
                ],
            },
        ]
    }

    #[test]
    fn distribute_enough_tables() {
        let data = get_data();
        let tables = distribute_tables_doubles(&data, 9);
        assert_eq!(tables.len(), data.len());
        tables.iter().for_each(|t_round|{
            t_round.iter().for_each(|t|assert_eq!(t.occupied_number, 0));
        });
    }

    #[test]
    fn distribute_not_enough_tables() {
        let data = get_data();
        let tables = distribute_tables_doubles(&data, 3);
        assert_eq!(tables.len(), data.len());
        tables.iter().for_each(|t_round|{
            for t in t_round.iter().enumerate(){
                match t.0 {
                    0 => assert_eq!(t.1.occupied_number, 0),
                    1 => assert_eq!(t.1.occupied_number, 0),
                    2 => assert_eq!(t.1.occupied_number, 0),
                    3 => assert_eq!(t.1.occupied_number, 1),
                    4 => assert_eq!(t.1.occupied_number, 1),
                    5 => assert_eq!(t.1.occupied_number, 1),
                    6 => assert_eq!(t.1.occupied_number, 2),
                    7 => assert_eq!(t.1.occupied_number, 2),
                    8 => assert_eq!(t.1.occupied_number, 2),
                    9 => assert_eq!(t.1.occupied_number, 3),
                    _ => {}
                }
            }
        });
    }
}