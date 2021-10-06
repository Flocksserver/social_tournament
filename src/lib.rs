//!
//! This is a library for creating tournament schedules for the sport I love.
//!
//! 🏓  table tennis 🏓
//!
//! The focus is on meeting as many opponents and teammates as possible during the tournament. One can draw a single or double tournament.
//! After the games are drawn, you can decide whether you also want to use the automatic table distribution algorithm.
use crate::double::{DrawOption, draw_doubles};
use crate::table::{DistributionOption, Table, distribute_tables};
use crate::single::draw_singles;
use crate::Error::DefaultError;
use thiserror::Error;
use crate::pdf::language::Language;
use crate::pdf::route_card::create_route_cards_for_double;

#[macro_use]
extern crate lazy_static;

pub mod single;
pub mod double;
pub mod table;
pub mod pdf;

type Result<T> = std::result::Result<T, Error>;

/// Default [enum@Error] for this crate
#[derive(Error, Debug)]
pub enum Error {
    #[error("default social tournament error")]
    DefaultError(),
}

/// This enum describes all relevant parameters for the tournament. Choose between a single
/// and a double tournament. How many players and rounds do you have? If you are playing doubles,
/// what is your preferred [DrawOption] if there are not enough players for "full" double pairs.
/// How many tables do you have? Specify a [TableConfig] to determine how many tables you can set up
/// in your room or gym and choose a [DistributionOption] if you have more matches in a round than
/// tables available.
#[derive(PartialEq, Clone, Debug)]
pub enum TournamentConfig {
    Single {
        number_of_players: usize,
        number_of_rounds: usize,
        table_config: TableConfig,
    },
    Double {
        number_of_players: usize,
        number_of_rounds: usize,
        draw_option: Option<DrawOption>,
        table_config: TableConfig,
    },
}

/// Specify a [TableConfig] to determine how many tables you can set up
/// in your room or gym and choose a [DistributionOption] if you have more matches in a round than
/// tables available.
#[derive(PartialEq, Clone, Debug)]
pub struct TableConfig {
    pub available_tables: usize,
    pub distribution_option: Option<DistributionOption>,
}

/// Enum that represent one match.
#[derive(PartialEq, Clone, Debug)]
pub enum Match {
    /// Struct that represent a single match. It holds the opponents `a` and `b`.
    /// The unique numbers is the id of the corresponding player.
    SingleMatch { a: usize, b: usize },
    /// Struct that represent a double match. It holds the pairs `double_a` and `double_b`.
    /// The tuple contains two numbers, the unique player ids.
    DoubleMatch { double_a: (usize, usize), double_b: (usize, usize) },
}

/// Struct for a tournament that represent one round. It holds the `round_number`
/// and the `matches` that take place in this round. Matches are a list of [Match].
#[derive(Debug, Clone)]
pub struct Round {
    pub round_number: usize,
    pub matches: Vec<Match>,
}

/// This struct provides the interface to `draw` the tournament, `distribute` the tables and
/// `create_route_cards_pdf`. All results are stored in this instance and can be read out
/// by accessing the corresponding field.
/// `rounds` contains the drawn rounds, `tables` contains the distributed tables and `pdf` contains
/// a pdf as a byte array.
#[derive(Debug, Clone)]
pub struct SocialTournament {
    config: TournamentConfig,
    pub rounds: Option<Vec<Round>>,
    pub tables: Option<Vec<Vec<Table>>>,
    pub pdf: Option<Vec<u8>>,
}

impl SocialTournament {

    /// This method is used to get a new instance of [SocialTournament]. You have to provide a
    /// [TournamentConfig].
    pub fn new(config: TournamentConfig) -> SocialTournament {
        SocialTournament { config, rounds: None, tables: None, pdf: None }
    }

    /// Interface to "create" a tournament.
    ///
    /// For a given `number_of_players` and `number_of_rounds` in [TournamentConfig] it returns a
    /// schedule of Rounds with the corresponding matches.
    ///
    /// # Example Single Tournament
    ///
    /// For `number_of_rounds` < `number_of_players` the round
    /// robin algorithm ensures, that one does not face an opponents twice. For
    /// `number_of_rounds` >= `number_of_players` the round robin is calculated one more round.
    /// For an odd number of players, the algorithm calculates with `number_of_players` + 1.
    /// So you have to make sure that the player who plays against the highest number has a bye.
    /// ```
    /// use social_tournament::{Round, SocialTournament, TournamentConfig, TableConfig};
    ///
    /// let mut tournament = SocialTournament::new(TournamentConfig::Single {
    ///     number_of_players: 12,
    ///     number_of_rounds: 2,
    ///     table_config: TableConfig { available_tables: 10, distribution_option: None }
    /// });
    ///
    /// tournament.draw().unwrap();
    /// let rounds: Vec<Round> = tournament.rounds.unwrap();
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
    /// # Example Double Tournament
    ///
    /// For number of players that are not completely divisible by 4 you can choose between three
    /// [DrawOption].
    /// Depending on the selected option you can have doubles with only 3 players, single matches or
    /// player with byes. You have to make sure that the player ids >= `number_of_players` in the
    /// schedule post processed correctly. So that you can mark them as byes for example.
    ///
    /// ```
    /// use social_tournament::{Round, SocialTournament, TournamentConfig, TableConfig};
    ///
    /// let mut tournament = SocialTournament::new(TournamentConfig::Double {
    ///     number_of_players: 24,
    ///     number_of_rounds: 2,
    ///     draw_option: None,
    ///     table_config: TableConfig { available_tables: 10, distribution_option: None }
    /// });
    ///
    /// tournament.draw().unwrap();
    /// let rounds: Vec<Round> = tournament.rounds.unwrap();
    /// /*
    /// Creates:
    /// Round number: 0
    /// DoubleMatch { double_a: (2, 37), double_b: (1, 38) }
    /// DoubleMatch { double_a: (3, 36), double_b: (4, 35) }
    /// DoubleMatch { double_a: (5, 34), double_b: (6, 33) }
    /// DoubleMatch { double_a: (7, 32), double_b: (8, 31) }
    /// DoubleMatch { double_a: (9, 30), double_b: (10, 29) }
    /// DoubleMatch { double_a: (11, 28), double_b: (12, 27) }
    /// DoubleMatch { double_a: (13, 26), double_b: (14, 25) }
    /// DoubleMatch { double_a: (15, 24), double_b: (16, 23) }
    /// DoubleMatch { double_a: (17, 22), double_b: (18, 21) }
    /// --------------
    /// Round number: 1
    /// DoubleMatch { double_a: (20, 21), double_b: (2, 0) }
    /// DoubleMatch { double_a: (3, 38), double_b: (7, 34) }
    /// DoubleMatch { double_a: (4, 37), double_b: (6, 35) }
    /// DoubleMatch { double_a: (5, 36), double_b: (9, 32) }
    /// DoubleMatch { double_a: (8, 33), double_b: (10, 31) }
    /// DoubleMatch { double_a: (11, 30), double_b: (15, 26) }
    /// DoubleMatch { double_a: (12, 29), double_b: (14, 27) }
    /// DoubleMatch { double_a: (13, 28), double_b: (17, 24) }
    /// DoubleMatch { double_a: (16, 25), double_b: (18, 23) }
    /// --------------
    /// ...
    /// */
    /// ```
    pub fn draw(&mut self) -> Result<()> {
        match self.config.clone() {
            TournamentConfig::Single { number_of_players, number_of_rounds, .. } => {
                self.rounds = Some(draw_singles(number_of_players, number_of_rounds))
            }
            TournamentConfig::Double { number_of_players, number_of_rounds, draw_option, .. } => {
                self.rounds = Some(draw_doubles(number_of_players, number_of_rounds, draw_option))
            }
        }
        Ok(())
    }


    /// Public interface to distribute the tables in a room or gym for the drawn tournament.
    ///
    /// Provides an interface to pass the drawn tournament [Round] and the number of your
    /// `available_tables`.
    /// Sometimes you may have more tables, but not enough space. Specify how
    /// many [Table] you can provide for the tournament in your room or gym. The algorithm ensures
    /// that enough sub-rounds are formed. You can specify the forming method by providing the
    /// [DistributionOption]. Depending on the option you choose, can have as many matches as possible
    /// in a sub-round or mainly even matches in each sub-round.
    /// Make sure to call the draw method first.
    ///
    /// # Example Single Tournament
    /// ```
    /// use social_tournament::{Round, SocialTournament, TournamentConfig, TableConfig};
    /// use social_tournament::table::Table;
    ///
    /// let mut tournament = SocialTournament::new(TournamentConfig::Single {
    ///     number_of_players: 12,
    ///     number_of_rounds: 2,
    ///     table_config: TableConfig { available_tables: 10, distribution_option: None }
    /// });
    ///
    /// tournament.draw().unwrap();
    /// tournament.distribute().unwrap();
    ///
    /// let tables: Vec<Vec<Table>> = tournament.tables.unwrap();
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
    /// # Example Double Tournament
    /// ```
    /// use social_tournament::{Round, SocialTournament, TournamentConfig, TableConfig};
    /// use social_tournament::table::Table;
    ///
    /// let mut tournament = SocialTournament::new(TournamentConfig::Double {
    ///     number_of_players: 24,
    ///     number_of_rounds: 2,
    ///     draw_option: None,
    ///     table_config: TableConfig { available_tables: 10, distribution_option: None }
    /// });
    ///
    /// tournament.draw().unwrap();
    /// tournament.distribute().unwrap();
    ///
    /// let tables: Vec<Vec<Table>> = tournament.tables.unwrap();
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
    pub fn distribute(&mut self) -> Result<()> {
        match self.rounds.clone() {
            None => {
                Err(DefaultError())
            }
            Some(rounds) => {
                match self.config.clone() {
                    TournamentConfig::Single { table_config, .. } => {
                        self.tables = Some(distribute_tables(&rounds, table_config.available_tables, table_config.distribution_option))
                    }
                    TournamentConfig::Double { table_config, .. } => {
                        self.tables = Some(distribute_tables(&rounds, table_config.available_tables, table_config.distribution_option))
                    }
                }
                Ok(())
            }
        }
    }

    /// Public interface to generate route cards for a drawn tournament with distributed tables.
    ///
    /// Available [Language] are English and German. Make sure you have drawn and distributed before
    /// calling this method.
    ///
    /// # Example Double Tournament
    /// ```
    /// use social_tournament::{Round, SocialTournament, TournamentConfig, TableConfig};
    /// use social_tournament::table::Table;
    /// use social_tournament::pdf::language::Language;
    ///
    /// let mut tournament = SocialTournament::new(TournamentConfig::Double {
    ///     number_of_players: 24,
    ///     number_of_rounds: 2,
    ///     draw_option: None,
    ///     table_config: TableConfig { available_tables: 10, distribution_option: None }
    /// });
    ///
    /// tournament.draw().unwrap();
    /// tournament.distribute().unwrap();
    /// tournament.create_route_cards_pdf(Language::EN).unwrap();
    ///
    /// let pdf: Vec<u8> = tournament.pdf.unwrap();
    pub fn create_route_cards_pdf(&mut self, language: Language) -> Result<()> {
        match (self.rounds.clone(), self.tables.clone()) {
            (Some(rounds), Some(tables)) => {
                match self.config.clone() {
                    TournamentConfig::Single { .. } => {
                        //TODO
                    }
                    TournamentConfig::Double { number_of_players, draw_option, table_config, .. } => {
                        let draw_o = draw_option.unwrap_or(DrawOption::AllInAction);
                        self.pdf = Some(create_route_cards_for_double(&rounds, &tables, number_of_players, draw_o, table_config.available_tables, language))
                    }
                }
                Ok(())
            }
            _ => Err(DefaultError())
        }
    }
}