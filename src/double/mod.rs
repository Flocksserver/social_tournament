use std::collections::HashMap;
use round_robin_tournament::round_robin_tournament::draw;
use std::iter::Peekable;
use std::ops::Range;
use crate::{Round, Match};
use crate::Match::DoubleMatch;

/// Draw options take effect only if the number of players is not completely divisible by 4
/// Or in other words `number_of_players % 4 != 0`
/// If no option is provided [DrawOption::AllInAction] is the default one
#[derive(PartialEq, Clone, Debug)]
pub enum DrawOption {
    /// [DrawOption::AllInAction] ensures that there are no byes
    /// For 3 players short -> there are one single match and one double with 3 players
    /// For 2 players short -> there is one single match
    /// For 1 player short -> there is one double with 3 players
    /// This is the default option
    AllInAction,
    /// [DrawOption::ForceDoubleOnly] ensures that there are only full valid double matches with 4 player
    /// That implies some byes
    /// For 3 players short -> 1 byes
    /// For 2 players short -> 2 byes
    /// For 1 player short -> 3 bye
    ForceDoubleOnly,
    /// [DrawOption::ValidCompositionsOnly] ensures that only valid table tennis matches can take place
    /// In other words: A double have to be played with 4 players
    /// That implies some byes
    /// For 3 players short -> 1 bye
    /// For 2 players short -> there is one single match
    /// For 1 player short -> there are one single match and 1 bye
    ValidCompositionsOnly,
}

/// Interface to create the double tournament.
///
/// For a given `number_of_players` and `number_of_rounds` it returns a schedule of Rounds
/// with the corresponding matches.
/// For number of players that are not completely divisible by 4 you can choose between three
/// [DrawOption].
/// Depending on the selected option you can have doubles with only 3 players, single matches or
/// player with byes. You have to make sure that the player ids >= `number_of_players` in the
/// schedule post processed correctly. So that you can mark them as byes for example.
pub(crate) fn draw_doubles(number_of_players: usize, number_of_rounds: usize, draw_option: Option<DrawOption>) -> Vec<Round> {
    let option= if number_of_players % 4 == 0 {
        None
    } else {
        Some(draw_option.unwrap_or(DrawOption::AllInAction))
    };

    let mut former_opponents: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..number_of_players {
        former_opponents.insert(i, vec![]);
    }
    let round_robin_pairs = draw(number_of_players);
    let mut tournament_rounds_used: Vec<Vec<(usize, usize)>> = Vec::new();


    let mut rounds: Vec<Round> = Vec::new();

    for i in 0..number_of_rounds {
        let mut has_solution = false;
        let mut meeting_score = 0;

        if tournament_rounds_used.clone().len() == round_robin_pairs.len() {
            tournament_rounds_used = Vec::new();
        }

        while has_solution == false {
            let tru = tournament_rounds_used.clone();
            let mut t_iter = round_robin_pairs.clone().into_iter().filter(|d| {
                let mut d_copy = d.clone();
                d_copy.sort();
                tru.contains(&d_copy) == false
            }).peekable();
            while t_iter.peek().is_some() && has_solution == false {
                let mut current_round = t_iter.next().unwrap();
                for _rotations in 0..current_round.len() {
                    match get_matches(number_of_players, &mut current_round, &mut former_opponents, meeting_score, &option) {
                        None => {}
                        Some(g) => {
                            current_round.sort();
                            tournament_rounds_used.push(current_round);
                            rounds.push(Round { round_number: i, matches: g.clone() });
                            has_solution = true;
                            break;
                        }
                    }
                }
            }
            if has_solution {
                println!("Found solution for Round {}", i + 1);
            } else {
                println!("In Round {} no result was found with meeting score {}", i + 1, meeting_score);
                meeting_score = meeting_score + 1;
            }
        }
    };
    rounds
}

fn get_matches(number_of_players: usize, r: &mut Vec<(usize, usize)>, former_opponents: &mut HashMap<usize, Vec<usize>>, meeting_score: usize, option: &Option<DrawOption>) -> Option<Vec<Match>> {
    let mut matches = Vec::new();
    let mut return_none = false;

    let mut conflict_list: Vec<(usize, usize)> = Vec::new();

    let ghost_double = if option.is_some() { Some((r.len() * 2, r.len() * 2 + 1)) } else { None };

    let mut iter_r = (0..r.len()).peekable();
    loop {
        let pos1 = iter_r.next();
        let pos2 = iter_r.next();
        if pos1.is_some() && pos2.is_some() {
            let double_a = get_next_double(number_of_players, r, &mut iter_r, pos1, option);
            let double_b = get_next_double(number_of_players, r, &mut iter_r, pos2, option);
            if double_a.is_some() && double_b.is_some() {
                if has_conflict(former_opponents, &double_a.unwrap(), &double_b.unwrap(), meeting_score, option, &ghost_double) {
                    conflict_list.push(double_a.unwrap());
                    conflict_list.push(double_b.unwrap())
                } else {
                    matches.push(DoubleMatch { double_a: double_a.unwrap().clone(), double_b: double_b.unwrap().clone() });
                    set_former_opponents(former_opponents, double_a.unwrap().clone(), double_b.unwrap().clone())
                }
            } else {
                break;
            }
        } else {
            // should add single opponents?
            if option.is_some() && pos1.is_some() && number_of_players % 4 != 3 && option.clone().unwrap() != DrawOption::ForceDoubleOnly {
                if number_of_players % 4 == 1 && option.clone().unwrap() == DrawOption::ValidCompositionsOnly {
                    // DO NOTHING
                } else {
                    let double_a: (usize, usize) = r.get(pos1.unwrap()).unwrap().clone();
                    if has_conflict(former_opponents, &double_a, &ghost_double.unwrap(), meeting_score, option, &ghost_double) {
                        conflict_list.push(double_a);
                        conflict_list.push(ghost_double.unwrap())
                    } else {
                        matches.push(DoubleMatch { double_a: double_a.clone(), double_b: ghost_double.unwrap().clone() });
                        set_former_opponents(former_opponents, double_a.clone(), ghost_double.unwrap())
                    }
                }

            }
            break;
        }
    }

    let mut iter_conflict_list = conflict_list.iter().peekable();
    let mut conflict_partner_list = conflict_list.clone();

    while iter_conflict_list.peek().is_some() && conflict_partner_list.clone().is_empty() == false {
        let double_to_check = iter_conflict_list.next().unwrap();
        if conflict_partner_list.clone().contains(double_to_check) {
            match conflict_partner_list.clone().iter().filter(|filter_same| { filter_same != &double_to_check }).find(|d_opponents| {
                has_conflict(former_opponents, double_to_check, d_opponents, meeting_score, option, &ghost_double) == false
            }) {
                None => {
                    return_none = true;
                    break;
                }
                Some(opponents) => {
                    matches.push(DoubleMatch { double_a: double_to_check.clone().clone(), double_b: opponents.clone().clone() });
                    set_former_opponents(former_opponents, double_to_check.clone().clone(), opponents.clone().clone());
                    conflict_partner_list.retain(|e| e != double_to_check);
                    conflict_partner_list.retain(|e| e != opponents);
                }
            }
        }
    }

    r.rotate_left(1);
    if return_none { None } else { Some(matches) }
}

fn get_next_double(number_of_player: usize, r: &mut Vec<(usize, usize)>, iter_r: &mut Peekable<Range<usize>>, pos: Option<usize>, option: &Option<DrawOption>) -> Option<(usize, usize)> {
    match option {
        None => Some(r.get(pos.unwrap()).unwrap().clone()),
        Some(o) => {
            match o {
                DrawOption::AllInAction => {
                    Some(r.get(pos.unwrap()).unwrap().clone())
                }
                _ => {
                    if pos.is_some() {
                        let mut success = true;
                        let mut a = r.get(pos.unwrap()).unwrap().clone();
                        while has_dummy_player_in_double(number_of_player, a.clone()) {
                            let pos_new = iter_r.next();
                            if pos_new.is_some() {
                                a = r.get(pos_new.unwrap()).unwrap().clone();
                            } else {
                                success = false;
                                break;
                            }
                        }
                        if success { Some(a) } else { None }
                    } else {
                        None
                    }
                }
            }
        }
    }
}

fn has_dummy_player_in_double(number_of_player: usize, double_to_check: (usize, usize)) -> bool {
    match number_of_player % 4 {
        1 => double_to_check.0 == number_of_player || double_to_check.1 == number_of_player || double_to_check.0 == number_of_player + 1 || double_to_check.1 == number_of_player + 1 || double_to_check.0 == number_of_player + 2 || double_to_check.1 == number_of_player + 2,
        3 => double_to_check.0 == number_of_player || double_to_check.1 == number_of_player,
        _ => false
    }
}

fn has_conflict(former_opponents: &HashMap<usize, Vec<usize>>, double_a: &(usize, usize), double_b: &(usize, usize), meeting_score: usize, option: &Option<DrawOption>, ghost_double: &Option<(usize, usize)>) -> bool {
    has_social_conflict(former_opponents, &double_a, &double_b, meeting_score) || has_draw_option_conflict(double_a, double_b, option, ghost_double)
}

fn has_social_conflict(former_opponents: &HashMap<usize, Vec<usize>>, double_a: &(usize, usize), double_b: &(usize, usize), meeting_score: usize) -> bool {
    check_player_conflicts(former_opponents, &double_a.0, &double_b.0, &double_b.1, meeting_score) ||
        check_player_conflicts(former_opponents, &double_a.1, &double_b.0, &double_b.1, meeting_score) ||
        check_player_conflicts(former_opponents, &double_b.0, &double_a.0, &double_a.1, meeting_score) ||
        check_player_conflicts(former_opponents, &double_b.1, &double_a.0, &double_a.1, meeting_score)
}

fn check_player_conflicts(former_opponents: &HashMap<usize, Vec<usize>>, player: &usize, opponent_1: &usize, opponent_2: &usize, meeting_score: usize) -> bool {
    match former_opponents.get(player) {
        None => false,
        Some(fo) => {
            let a = fo.iter().filter(|p| { p == &opponent_1 }).collect::<Vec<&usize>>();
            let b = fo.iter().filter(|p| { p == &opponent_2 }).collect::<Vec<&usize>>();
            a.len() > meeting_score || b.len() > meeting_score
        }
    }
}

fn has_draw_option_conflict(double_a: &(usize, usize), double_b: &(usize, usize), option: &Option<DrawOption>, ghost_double: &Option<(usize, usize)>) -> bool {
    match option {
        None => false,
        Some(o) => {
            match o {
                DrawOption::AllInAction => {
                    match ghost_double {
                        None => false,
                        Some(gd) => {
                            // check that not player number (number_of_player, number_of_player+1), so the ghost double playing
                            // against a double with only one player -> this would be one bye
                            if double_b == gd && (double_a.0 == gd.0 - 1 || double_a.1 == gd.0 - 1) ||
                                double_a == gd && (double_b.0 == gd.0 - 1 || double_b.1 == gd.0 - 1) {
                                true
                            } else {
                                false
                            }
                        }
                    }
                }
                _ => false,
            }
        }
    }
}

fn set_former_opponents(former_opponents: &mut HashMap<usize, Vec<usize>>, double_a: (usize, usize), double_b: (usize, usize)) {
    set_player_opponents(former_opponents, double_a.0, double_b.0, double_b.1);
    set_player_opponents(former_opponents, double_a.1, double_b.0, double_b.1);
    set_player_opponents(former_opponents, double_b.0, double_a.0, double_a.1);
    set_player_opponents(former_opponents, double_b.1, double_a.0, double_a.1);
}

fn set_player_opponents(former_opponents: &mut HashMap<usize, Vec<usize>>, player: usize, opponent_1: usize, opponent_2: usize) {
    match former_opponents.get_mut(&player) {
        None => {}
        Some(fo) => {
            fo.push(opponent_1.clone());
            fo.push(opponent_2.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::double::{draw_doubles, DrawOption};
    use crate::Match;

    #[test]
    fn draw_144_12() {
        let number_of_players = 144;
        let number_of_rounds = 12;

        let rounds = draw_doubles(number_of_players, number_of_rounds, None);

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), number_of_players / 4);
            for i in 0..number_of_players {
                let fp = r.matches.iter().filter(|p| {
                    match p {
                        Match::SingleMatch { .. } => {false}
                        Match::DoubleMatch { double_a, double_b } => {
                            double_a.0 == i || double_a.1 == i || double_b.0 == i || double_b.1 == i
                        }
                    }
                }).collect::<Vec<&Match>>();
                assert_eq!(fp.len(), 1);
            }
        });
    }

    #[test]
    fn draw_28_12() {
        let number_of_players = 28;
        let number_of_rounds = 12;

        let rounds = draw_doubles(number_of_players, number_of_rounds, None);

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), number_of_players / 4);
            for i in 0..number_of_players {
                let fp = r.matches.iter().filter(|p| {
                    match p {
                        Match::SingleMatch { .. } => {false}
                        Match::DoubleMatch { double_a, double_b } => {
                            double_a.0 == i || double_a.1 == i || double_b.0 == i || double_b.1 == i
                        }
                    }
                }).collect::<Vec<&Match>>();
                println!("Player number {}", i);
                println!("{:?}",fp);
                println!("--------------");
                assert_eq!(fp.len(), 1);
            }
        });
    }

    #[test]
    fn draw_40_12() {
        let number_of_players = 40;
        let number_of_rounds = 12;

        let rounds = draw_doubles(number_of_players, number_of_rounds, None);

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), number_of_players / 4);
            for i in 0..number_of_players {
                let fp = r.matches.iter().filter(|p| {
                    match p {
                        Match::SingleMatch { .. } => {false}
                        Match::DoubleMatch { double_a, double_b } => {
                            double_a.0 == i || double_a.1 == i || double_b.0 == i || double_b.1 == i
                        }
                    }
                }).collect::<Vec<&Match>>();
                assert_eq!(fp.len(), 1);
            }
        });
    }

    #[test]
    fn draw_39_12_option_none() {
        let number_of_players = 39;
        let number_of_rounds = 12;

        let rounds = draw_doubles(number_of_players, number_of_rounds, None);

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), (number_of_players + 1) / 4);
            for i in 0..(number_of_players + 1) {
                let fp = r.matches.iter().filter(|p| {
                    match p {
                        Match::SingleMatch { .. } => {false}
                        Match::DoubleMatch { double_a, double_b } => {
                            double_a.0 == i || double_a.1 == i || double_b.0 == i || double_b.1 == i
                        }
                    }
                }).collect::<Vec<&Match>>();
                assert_eq!(fp.len(), 1);
            }
        });
    }

    #[test]
    fn draw_38_12_option_none() {
        let number_of_players = 38;
        let number_of_rounds = 12;

        let rounds = draw_doubles(number_of_players, number_of_rounds, None);

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), (number_of_players + 2) / 4);
            for i in 0..(number_of_players + 2) {
                println!("{:?}", r);
                let fp = r.matches.iter().filter(|p| {
                    match p {
                        Match::SingleMatch { .. } => {false}
                        Match::DoubleMatch { double_a, double_b } => {
                            double_a.0 == i || double_a.1 == i || double_b.0 == i || double_b.1 == i
                        }
                    }
                }).collect::<Vec<&Match>>();
                assert_eq!(fp.len(), 1);
            }
        });
    }

    #[test]
    fn draw_37_12_option_none() {
        let number_of_players = 37;
        let number_of_rounds = 12;

        let rounds = draw_doubles(number_of_players, number_of_rounds, None);

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), (number_of_players + 3) / 4);
            for i in 0..(number_of_players + 3) {
                let fp = r.matches.iter().filter(|p| {
                    match p {
                        Match::SingleMatch { .. } => {false}
                        Match::DoubleMatch { double_a, double_b } => {
                            double_a.0 == i || double_a.1 == i || double_b.0 == i || double_b.1 == i
                        }
                    }
                }).collect::<Vec<&Match>>();
                assert_eq!(fp.len(), 1);
            }
        });
    }

    #[test]
    fn draw_39_12_option_all_in_action() {
        let number_of_players = 39;
        let number_of_rounds = 12;

        let rounds = draw_doubles(number_of_players, number_of_rounds, Some(DrawOption::AllInAction));

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), (number_of_players + 1) / 4);
            for i in 0..(number_of_players + 1) {
                let fp = r.matches.iter().filter(|p| {
                    match p {
                        Match::SingleMatch { .. } => {false}
                        Match::DoubleMatch { double_a, double_b } => {
                            double_a.0 == i || double_a.1 == i || double_b.0 == i || double_b.1 == i
                        }
                    }
                }).collect::<Vec<&Match>>();
                assert_eq!(fp.len(), 1);
            }
        });
    }

    #[test]
    fn draw_38_12_option_all_in_action() {
        let number_of_players = 38;
        let number_of_rounds = 12;

        let rounds = draw_doubles(number_of_players, number_of_rounds, Some(DrawOption::AllInAction));

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), (number_of_players + 2) / 4);
            for i in 0..(number_of_players + 2) {
                println!("{:?}", r);
                let fp = r.matches.iter().filter(|p| {
                    match p {
                        Match::SingleMatch { .. } => {false}
                        Match::DoubleMatch { double_a, double_b } => {
                            double_a.0 == i || double_a.1 == i || double_b.0 == i || double_b.1 == i
                        }
                    }
                }).collect::<Vec<&Match>>();
                assert_eq!(fp.len(), 1);
            }
        });
    }

    #[test]
    fn draw_37_12_all_in_action() {
        let number_of_players = 37;
        let number_of_rounds = 12;

        let rounds = draw_doubles(number_of_players, number_of_rounds, Some(DrawOption::AllInAction));

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), (number_of_players + 3) / 4);
            for i in 0..(number_of_players + 3) {
                let fp = r.matches.iter().filter(|p| {
                    match p {
                        Match::SingleMatch { .. } => {false}
                        Match::DoubleMatch { double_a, double_b } => {
                            double_a.0 == i || double_a.1 == i || double_b.0 == i || double_b.1 == i
                        }
                    }
                }).collect::<Vec<&Match>>();
                assert_eq!(fp.len(), 1);
            }
        });
    }


    #[test]
    fn draw_39_12_option_force_double_only() {
        let number_of_players = 39;
        let number_of_rounds = 12;

        let rounds = draw_doubles(number_of_players, number_of_rounds, Some(DrawOption::ForceDoubleOnly));

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), ((number_of_players + 1) / 4) - 1);
            assert_eq!(r.matches.len(), ((number_of_players + 3) / 4) - 1);
            // ghost player should not be in the games list
            let fp = r.matches.iter().filter(|p| {
                match p {
                    Match::SingleMatch { .. } => {false}
                    Match::DoubleMatch { double_a, double_b } => {
                        double_a.0 == number_of_players || double_a.1 == number_of_players || double_b.0 == number_of_players || double_b.1 == number_of_players
                    }
                }
            }).collect::<Vec<&Match>>();
            assert_eq!(fp.len(), 0);

            // ghost player should not be in the games list
            let fp = r.matches.iter().filter(|p| {
                match p {
                    Match::SingleMatch { .. } => {false}
                    Match::DoubleMatch { double_a, double_b } => {
                        double_a.0 == number_of_players + 1 || double_a.1 == number_of_players + 1 || double_b.0 == number_of_players + 1 || double_b.1 == number_of_players + 1
                    }
                }
            }).collect::<Vec<&Match>>();
            assert_eq!(fp.len(), 0);

            // ghost player should not be in the games list
            let fp = r.matches.iter().filter(|p| {
                match p {
                    Match::SingleMatch { .. } => {false}
                    Match::DoubleMatch { double_a, double_b } => {
                        double_a.0 == number_of_players + 2 || double_a.1 == number_of_players + 2 || double_b.0 == number_of_players + 2 || double_b.1 == number_of_players + 2
                    }
                }
            }).collect::<Vec<&Match>>();
            assert_eq!(fp.len(), 0);
        });
    }

    #[test]
    fn draw_38_12_option_force_double_only() {
        let number_of_players = 38;
        let number_of_rounds = 12;

        let rounds = draw_doubles(number_of_players, number_of_rounds, Some(DrawOption::ForceDoubleOnly));

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), ((number_of_players + 2) / 4) - 1);

            // ghost player should not be in the games list
            let fp = r.matches.iter().filter(|p| {
                match p {
                    Match::SingleMatch { .. } => {false}
                    Match::DoubleMatch { double_a, double_b } => {
                        double_a.0 == number_of_players || double_a.1 == number_of_players || double_b.0 == number_of_players || double_b.1 == number_of_players
                    }
                }
            }).collect::<Vec<&Match>>();
            assert_eq!(fp.len(), 0);

            // ghost player should not be in the games list
            let fp = r.matches.iter().filter(|p| {
                match p {
                    Match::SingleMatch { .. } => {false}
                    Match::DoubleMatch { double_a, double_b } => {
                        double_a.0 == number_of_players + 1 || double_a.1 == number_of_players + 1 || double_b.0 == number_of_players + 1 || double_b.1 == number_of_players + 1
                    }
                }
            }).collect::<Vec<&Match>>();
            assert_eq!(fp.len(), 0);
        });
    }

    #[test]
    fn draw_37_12_force_double_only() {
        let number_of_players = 37;
        let number_of_rounds = 12;

        let rounds = draw_doubles(number_of_players, number_of_rounds, Some(DrawOption::ForceDoubleOnly));

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), ((number_of_players + 3) / 4) - 1);
            // ghost player should not be in the games list
            let fp = r.matches.iter().filter(|p| {
                match p {
                    Match::SingleMatch { .. } => {false}
                    Match::DoubleMatch { double_a, double_b } => {
                        double_a.0 == number_of_players || double_a.1 == number_of_players || double_b.0 == number_of_players || double_b.1 == number_of_players
                    }
                }
            }).collect::<Vec<&Match>>();
            assert_eq!(fp.len(), 0);

            // ghost player should not be in the games list
            let fp = r.matches.iter().filter(|p| {
                match p {
                    Match::SingleMatch { .. } => {false}
                    Match::DoubleMatch { double_a, double_b } => {
                        double_a.0 == number_of_players + 1 || double_a.1 == number_of_players + 1 || double_b.0 == number_of_players + 1 || double_b.1 == number_of_players + 1
                    }
                }
            }).collect::<Vec<&Match>>();
            assert_eq!(fp.len(), 0);

            // ghost player should not be in the games list
            let fp = r.matches.iter().filter(|p| {
                match p {
                    Match::SingleMatch { .. } => {false}
                    Match::DoubleMatch { double_a, double_b } => {
                        double_a.0 == number_of_players + 2 || double_a.1 == number_of_players + 2 || double_b.0 == number_of_players + 2 || double_b.1 == number_of_players + 2
                    }
                }
            }).collect::<Vec<&Match>>();
            assert_eq!(fp.len(), 0);
        });
    }


    #[test]
    fn draw_39_12_option_valid_compositions_only() {
        let number_of_players = 39;
        let number_of_rounds = 12;

        let rounds = draw_doubles(number_of_players, number_of_rounds, Some(DrawOption::ValidCompositionsOnly));

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), ((number_of_players + 1) / 4) - 1);

            // ghost player should not be in the games list
            let fp = r.matches.iter().filter(|p| {
                match p {
                    Match::SingleMatch { .. } => {false}
                    Match::DoubleMatch { double_a, double_b } => {
                        double_a.0 == number_of_players || double_a.1 == number_of_players || double_b.0 == number_of_players || double_b.1 == number_of_players
                    }
                }
            }).collect::<Vec<&Match>>();
            assert_eq!(fp.len(), 0);
        });
    }

    #[test]
    fn draw_38_12_option_valid_compositions_only() {
        let number_of_players = 38;
        let number_of_rounds = 12;

        let rounds = draw_doubles(number_of_players, number_of_rounds, Some(DrawOption::ValidCompositionsOnly));

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), ((number_of_players + 2) / 4));

            // ghost player should be in the games list
            let fp = r.matches.iter().filter(|p| {
                match p {
                    Match::SingleMatch { .. } => {false}
                    Match::DoubleMatch { double_a, double_b } => {
                        double_a.0 == number_of_players || double_a.1 == number_of_players || double_b.0 == number_of_players || double_b.1 == number_of_players
                    }
                }
            }).collect::<Vec<&Match>>();
            assert_eq!(fp.len(), 1);

            // ghost player should be in the games list
            let fp = r.matches.iter().filter(|p| {
                match p {
                    Match::SingleMatch { .. } => {false}
                    Match::DoubleMatch { double_a, double_b } => {
                        double_a.0 == number_of_players + 1 || double_a.1 == number_of_players + 1 || double_b.0 == number_of_players + 1 || double_b.1 == number_of_players + 1
                    }
                }
            }).collect::<Vec<&Match>>();
            assert_eq!(fp.len(), 1);
        });
    }

    #[test]
    fn draw_37_12_valid_compositions_only() {
        let number_of_players = 37;
        let number_of_rounds = 12;

        let rounds = draw_doubles(number_of_players, number_of_rounds, Some(DrawOption::ValidCompositionsOnly));

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), ((number_of_players + 3) / 4) - 1);
            // ghost player should not be in the games list
            let fp = r.matches.iter().filter(|p| {
                match p {
                    Match::SingleMatch { .. } => {false}
                    Match::DoubleMatch { double_a, double_b } => {
                        double_a.0 == number_of_players || double_a.1 == number_of_players || double_b.0 == number_of_players || double_b.1 == number_of_players
                    }
                }
            }).collect::<Vec<&Match>>();
            assert_eq!(fp.len(), 0);

            // ghost player should not be in the games list
            let fp = r.matches.iter().filter(|p| {
                match p {
                    Match::SingleMatch { .. } => {false}
                    Match::DoubleMatch { double_a, double_b } => {
                        double_a.0 == number_of_players + 1 || double_a.1 == number_of_players + 1 || double_b.0 == number_of_players + 1 || double_b.1 == number_of_players + 1
                    }
                }
            }).collect::<Vec<&Match>>();
            assert_eq!(fp.len(), 0);

            // ghost player should not be in the games list
            let fp = r.matches.iter().filter(|p| {
                match p {
                    Match::SingleMatch { .. } => {false}
                    Match::DoubleMatch { double_a, double_b } => {
                        double_a.0 == number_of_players + 2 || double_a.1 == number_of_players + 2 || double_b.0 == number_of_players + 2 || double_b.1 == number_of_players + 2
                    }
                }
            }).collect::<Vec<&Match>>();
            assert_eq!(fp.len(), 0);
        });
    }
    #[test]
    fn draw_4_5() {
        let number_of_players = 4;
        let number_of_rounds = 5;

        let rounds = draw_doubles(number_of_players, number_of_rounds, None);

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), number_of_players / 4);
            for i in 0..number_of_players {
                let fp = r.matches.iter().filter(|p| {
                    match p {
                        Match::SingleMatch { .. } => {false}
                        Match::DoubleMatch { double_a, double_b } => {
                            double_a.0 == i || double_a.1 == i || double_b.0 == i || double_b.1 == i
                        }
                    }
                }).collect::<Vec<&Match>>();
                assert_eq!(fp.len(), 1);
            }
        });
    }

}