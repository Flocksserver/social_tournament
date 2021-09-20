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
//! let tables: Vec<Vec<Table>> = distribute_tables_doubles(&tournament, 4, None);
//! ```
//!

use crate::double::RoundDoubles;
use crate::single::RoundSingles;


/// Distribution options take effect only if the number of tables is not sufficient to play the
/// matches in one round.
/// If no option is provided [DistributionOption::FillUp] is the default one.
#[derive(PartialEq, Clone)]
pub enum DistributionOption {
    /// [DistributionOption::FillUp] distribute as many matches as possible in the first subround
    /// This means, that all available tables are used.
    /// This is the default option.
    FillUp,
    /// [DistributionOption::Evenly] ensures that the number of tables used in the
    /// sub-rounds are balanced. However, the number of sub-rounds to be played remains the same
    /// compared to [DistributionOption::FillUp].
    Evenly,
}


#[derive(Debug, Clone)]
pub struct Table {
    /// A table has a numerical value to identify it in the gym or room.
    pub table_number: usize,
    /// If there are more games than tables, a round must be played in several sub-rounds.
    /// Therefore, a table can be occupied in sub-rounds 0, 1, ...
    pub occupied_number: usize,
}

/// Public interface to distribute the tables for the double tournament.
///
/// Provides an interface to pass the drawn tournament [RoundDoubles] and the number of your
/// `available_tables`.
/// Sometimes you may have more tables, but not enough space. Specify how
/// many [Table] you can provide for the tournament in your room or gym. The algorithm ensures
/// that enough sub-rounds are formed. You can specify the forming method by providing the
/// [DistributionOption]. Depending on the option you choose, can have as many matches as possible
/// in a sub-round or mainly even matches in each sub-round.
///
/// # Example
/// ```
/// use social_tournament::double::{draw_doubles, RoundDoubles};
/// use social_tournament::table::{Table, distribute_tables_doubles};
///
/// let tournament: Vec<RoundDoubles> = draw_doubles(24, 2, None);
/// let tables: Vec<Vec<Table>> = distribute_tables_doubles(&tournament, 4, None);
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
pub fn distribute_tables_doubles(tournament: &Vec<RoundDoubles>, available_tables: usize, distribution_option: Option<DistributionOption>) -> Vec<Vec<Table>> {
    let number_of_matches = tournament.first().unwrap_or(&RoundDoubles { round_number: 0, matches: vec![] }).matches.len();
    let option = if number_of_matches <= available_tables {
        None
    } else {
        Some(distribution_option.unwrap_or(DistributionOption::FillUp))
    };

    let mut tables: Vec<Vec<Table>> = Vec::new();
    let number_of_sub_rounds = (number_of_matches / available_tables) + 1;
    tournament.iter().for_each(|r| {
        let mut tables_for_current_round: Vec<Table> = Vec::new();
        for i in r.matches.iter().enumerate() {
            tables_for_current_round.push(get_table_distribution(i.0, available_tables, number_of_sub_rounds, number_of_matches, option.clone()))
        }
        tables.push(tables_for_current_round);
    });
    tables
}


/// Public interface to distribute the tables for the single tournament.
///
/// Provides an interface to pass the drawn tournament [RoundDoubles] and the number of your
/// `available_tables`.
/// Sometimes you may have more tables, but not enough space. Specify how
/// many [Table] you can provide for the tournament in your room or gym. The algorithm ensures
/// that enough sub-rounds are formed. You can specify the forming method by providing the
/// [DistributionOption]. Depending on the option you choose, can have as many matches as possible
/// in a sub-round or mainly even matches in each sub-round.
///
/// # Example
/// ```
/// use social_tournament::single::{draw_singles, RoundSingles};
/// use social_tournament::table::{Table, distribute_tables_singles};
///
/// let tournament: Vec<RoundSingles> = draw_singles(12, 2);
/// let tables: Vec<Vec<Table>> = distribute_tables_singles(&tournament, 4, None);
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
pub fn distribute_tables_singles(tournament: &Vec<RoundSingles>, available_tables: usize, distribution_option: Option<DistributionOption>) -> Vec<Vec<Table>> {
    let number_of_matches = tournament.first().unwrap_or(&RoundSingles { round_number: 0, matches: vec![] }).matches.len();
    let option = if number_of_matches <= available_tables {
        None
    } else {
        Some(distribution_option.unwrap_or(DistributionOption::FillUp))
    };

    let mut tables: Vec<Vec<Table>> = Vec::new();
    let number_of_sub_rounds = (number_of_matches / available_tables) + 1;
    tournament.iter().for_each(|r| {
        let mut tables_for_current_round: Vec<Table> = Vec::new();
        for i in r.matches.iter().enumerate() {
            tables_for_current_round.push(get_table_distribution(i.0, available_tables, number_of_sub_rounds, number_of_matches, option.clone()))
        }
        tables.push(tables_for_current_round);
    });
    tables
}

fn get_table_distribution(iteration: usize, available_tables: usize, number_of_sub_rounds: usize, number_of_matches: usize, option: Option<DistributionOption>) -> Table {
    let tn_on = match option.clone() {
        None => { (iteration % available_tables, iteration / available_tables) }
        Some(o) => {
            match o {
                DistributionOption::FillUp => { (iteration % available_tables, iteration / available_tables) }
                DistributionOption::Evenly => {
                    let evenly_factor = (number_of_matches as f64 / number_of_sub_rounds as f64).ceil() as usize;
                    (iteration % evenly_factor, iteration / evenly_factor)
                }
            }
        }
    };
    Table { table_number: tn_on.0, occupied_number: tn_on.1 }
}

#[cfg(test)]
mod tests {
    use crate::table::{distribute_tables_doubles, DistributionOption, distribute_tables_singles};
    use crate::double::{RoundDoubles, DoubleMatch};
    use crate::single::{RoundSingles, SingleMatch};

    fn get_double_data() -> Vec<RoundDoubles> {
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

    fn get_single_data() -> Vec<RoundSingles> {
        vec![
            RoundSingles {
                round_number: 0,
                matches: vec![
                    SingleMatch { a: 2, b: 37 },
                    SingleMatch { a: 3, b: 36 },
                    SingleMatch { a: 5, b: 34 },
                    SingleMatch { a: 7, b: 32 },
                    SingleMatch { a: 9, b: 30 },
                    SingleMatch { a: 11, b: 28 },
                    SingleMatch { a: 13, b: 26 },
                    SingleMatch { a: 15, b: 24 },
                    SingleMatch { a: 17, b: 22 },
                ],
            },
            RoundSingles {
                round_number: 1,
                matches: vec![
                    SingleMatch { a: 20, b: 21 },
                    SingleMatch { a: 3, b: 38 },
                    SingleMatch { a: 4, b: 37 },
                    SingleMatch { a: 5, b: 36 },
                    SingleMatch { a: 8, b: 33 },
                    SingleMatch { a: 11, b: 30 },
                    SingleMatch { a: 12, b: 29 },
                    SingleMatch { a: 13, b: 28 },
                    SingleMatch { a: 16, b: 25 },
                ],
            },
        ]
    }

    #[test]
    fn distribute_double_enough_tables() {
        let data = get_double_data();
        let tables = distribute_tables_doubles(&data, 9, None);
        assert_eq!(tables.len(), data.len());
        assert_eq!(tables.first().unwrap().len(), data.first().unwrap().matches.len());
        tables.iter().for_each(|t_round| {
            t_round.iter().for_each(|t| assert_eq!(t.occupied_number, 0));
        });
    }

    #[test]
    fn distribute_double_not_enough_tables_none_option() {
        let data = get_double_data();
        let tables = distribute_tables_doubles(&data, 3, None);
        assert_eq!(tables.len(), data.len());
        assert_eq!(tables.first().unwrap().len(), data.first().unwrap().matches.len());
        tables.iter().for_each(|t_round| {
            for t in t_round.iter().enumerate() {
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
                    _ => {}
                }
            }
        });
    }

    #[test]
    fn distribute_double_not_enough_tables_fill_up_option() {
        let data = get_double_data();
        let tables = distribute_tables_doubles(&data, 4, Some(DistributionOption::FillUp));
        assert_eq!(tables.len(), data.len());
        assert_eq!(tables.first().unwrap().len(), data.first().unwrap().matches.len());
        tables.iter().for_each(|t_round| {
            for t in t_round.iter().enumerate() {
                match t.0 {
                    0 => assert_eq!(t.1.occupied_number, 0),
                    1 => assert_eq!(t.1.occupied_number, 0),
                    2 => assert_eq!(t.1.occupied_number, 0),
                    3 => assert_eq!(t.1.occupied_number, 0),
                    4 => assert_eq!(t.1.occupied_number, 1),
                    5 => assert_eq!(t.1.occupied_number, 1),
                    6 => assert_eq!(t.1.occupied_number, 1),
                    7 => assert_eq!(t.1.occupied_number, 1),
                    8 => assert_eq!(t.1.occupied_number, 2),
                    _ => {}
                }
            }
        });
    }

    #[test]
    fn distribute_double_not_enough_tables_evenly_option() {
        let data = get_double_data();
        let tables = distribute_tables_doubles(&data, 4, Some(DistributionOption::Evenly));
        assert_eq!(tables.len(), data.len());
        assert_eq!(tables.first().unwrap().len(), data.first().unwrap().matches.len());
        tables.iter().for_each(|t_round| {
            for t in t_round.iter().enumerate() {
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
                    _ => {}
                }
            }
        });
    }

    #[test]
    fn distribute_double_not_enough_tables_evenly_option_uneven() {
        let data = get_double_data();
        let tables = distribute_tables_doubles(&data, 6, Some(DistributionOption::Evenly));
        assert_eq!(tables.len(), data.len());
        assert_eq!(tables.first().unwrap().len(), data.first().unwrap().matches.len());
        tables.iter().for_each(|t_round| {
            for t in t_round.iter().enumerate() {
                match t.0 {
                    0 => assert_eq!(t.1.occupied_number, 0),
                    1 => assert_eq!(t.1.occupied_number, 0),
                    2 => assert_eq!(t.1.occupied_number, 0),
                    3 => assert_eq!(t.1.occupied_number, 0),
                    4 => assert_eq!(t.1.occupied_number, 0),
                    5 => assert_eq!(t.1.occupied_number, 1),
                    6 => assert_eq!(t.1.occupied_number, 1),
                    7 => assert_eq!(t.1.occupied_number, 1),
                    8 => assert_eq!(t.1.occupied_number, 1),
                    _ => {}
                }
            }
        });
    }


    #[test]
    fn distribute_single_enough_tables() {
        let data = get_single_data();
        let tables = distribute_tables_singles(&data, 9, None);
        assert_eq!(tables.len(), data.len());
        assert_eq!(tables.first().unwrap().len(), data.first().unwrap().matches.len());
        tables.iter().for_each(|t_round| {
            t_round.iter().for_each(|t| assert_eq!(t.occupied_number, 0));
        });
    }

    #[test]
    fn distribute_single_not_enough_tables_none_option() {
        let data = get_single_data();
        let tables = distribute_tables_singles(&data, 3, None);
        assert_eq!(tables.len(), data.len());
        assert_eq!(tables.first().unwrap().len(), data.first().unwrap().matches.len());
        tables.iter().for_each(|t_round| {
            for t in t_round.iter().enumerate() {
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
                    _ => {}
                }
            }
        });
    }

    #[test]
    fn distribute_single_not_enough_tables_fill_up_option() {
        let data = get_single_data();
        let tables = distribute_tables_singles(&data, 4, Some(DistributionOption::FillUp));
        assert_eq!(tables.len(), data.len());
        assert_eq!(tables.first().unwrap().len(), data.first().unwrap().matches.len());
        tables.iter().for_each(|t_round| {
            for t in t_round.iter().enumerate() {
                match t.0 {
                    0 => assert_eq!(t.1.occupied_number, 0),
                    1 => assert_eq!(t.1.occupied_number, 0),
                    2 => assert_eq!(t.1.occupied_number, 0),
                    3 => assert_eq!(t.1.occupied_number, 0),
                    4 => assert_eq!(t.1.occupied_number, 1),
                    5 => assert_eq!(t.1.occupied_number, 1),
                    6 => assert_eq!(t.1.occupied_number, 1),
                    7 => assert_eq!(t.1.occupied_number, 1),
                    8 => assert_eq!(t.1.occupied_number, 2),
                    _ => {}
                }
            }
        });
    }

    #[test]
    fn distribute_single_not_enough_tables_evenly_option() {
        let data = get_single_data();
        let tables = distribute_tables_singles(&data, 4, Some(DistributionOption::Evenly));
        assert_eq!(tables.len(), data.len());
        assert_eq!(tables.first().unwrap().len(), data.first().unwrap().matches.len());
        tables.iter().for_each(|t_round| {
            for t in t_round.iter().enumerate() {
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
                    _ => {}
                }
            }
        });
    }

    #[test]
    fn distribute_single_not_enough_tables_evenly_option_uneven() {
        let data = get_single_data();
        let tables = distribute_tables_singles(&data, 6, Some(DistributionOption::Evenly));
        assert_eq!(tables.len(), data.len());
        assert_eq!(tables.first().unwrap().len(), data.first().unwrap().matches.len());
        tables.iter().for_each(|t_round| {
            for t in t_round.iter().enumerate() {
                match t.0 {
                    0 => assert_eq!(t.1.occupied_number, 0),
                    1 => assert_eq!(t.1.occupied_number, 0),
                    2 => assert_eq!(t.1.occupied_number, 0),
                    3 => assert_eq!(t.1.occupied_number, 0),
                    4 => assert_eq!(t.1.occupied_number, 0),
                    5 => assert_eq!(t.1.occupied_number, 1),
                    6 => assert_eq!(t.1.occupied_number, 1),
                    7 => assert_eq!(t.1.occupied_number, 1),
                    8 => assert_eq!(t.1.occupied_number, 1),
                    _ => {}
                }
            }
        });
    }
}