use round_robin_tournament::round_robin_tournament::draw;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RoundDoubles {
    pub round_number: usize,
    pub matches: Vec<DoubleMatch>,
}

#[derive(Debug, Clone)]
pub struct RoundSingles {
    pub round_number: usize,
    pub matches: Vec<SingleMatch>,
}

#[derive(Debug, Clone)]
pub struct DoubleMatch {
    pub double_a: (usize, usize),
    pub double_b: (usize, usize),
}

#[derive(Debug, Clone)]
pub struct SingleMatch {
    pub a: usize,
    pub b: usize,
}

pub fn run() {
    let number_of_player = 144;
    let number_of_rounds = 12;

    let _rounds = draw_doubles(number_of_player, number_of_rounds);

    // TODO distribute matches in each round to number of tables
}

pub fn draw_singles(number_of_player: usize, number_of_rounds: usize) -> Vec<RoundSingles> {
    let round_robin_pairs = draw(number_of_player);
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

pub fn draw_doubles(number_of_player: usize, number_of_rounds: usize) -> Vec<RoundDoubles> {
    let mut former_opponents: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..number_of_player {
        former_opponents.insert(i, vec![]);
    }
    let round_robin_pairs = draw(number_of_player);
    let mut tournament_rounds_used: Vec<Vec<(usize, usize)>> = Vec::new();


    let mut rounds: Vec<RoundDoubles> = Vec::new();

    for i in 0..number_of_rounds {
        let mut has_solution = false;
        let mut meeting_score = 0;

        while has_solution == false {
            let tru = tournament_rounds_used.clone();
            let mut t_iter = round_robin_pairs.clone().into_iter().filter(|d| tru.contains(d) == false).peekable();
            while t_iter.peek().is_some() && has_solution == false {
                let mut current_round = t_iter.next().unwrap();
                for _rotations in 0..current_round.len() {
                    match get_matches(&mut current_round, &mut former_opponents, meeting_score) {
                        None => {}
                        Some(g) => {
                            tournament_rounds_used.push(current_round);
                            rounds.push(RoundDoubles { round_number: i, matches: g.clone() });
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

fn get_matches(r: &mut Vec<(usize, usize)>, former_opponents: &mut HashMap<usize, Vec<usize>>, meeting_score: usize) -> Option<Vec<DoubleMatch>> {
    let mut matches = Vec::new();
    let mut return_none = false;

    let mut conflict_list: Vec<&(usize, usize)> = Vec::new();

    for p in 0..r.len() {
        if p % 2 == 0 {
            let pos1 = p;
            let pos2 = p + 1;
            let double_a: &(usize, usize) = r.get(pos1).unwrap();
            let double_b: &(usize, usize) = r.get(pos2).unwrap();

            if has_conflict(former_opponents, double_a, double_b, meeting_score) {
                conflict_list.push(double_a);
                conflict_list.push(double_b)
            } else {
                matches.push(DoubleMatch { double_a: double_a.clone(), double_b: double_b.clone() });
                set_former_opponents(former_opponents, double_a, double_b)
            }
        }
    }


    let mut iter_conflict_list = conflict_list.iter().peekable();
    let mut conflict_partner_list = conflict_list.clone();

    while iter_conflict_list.peek().is_some() && conflict_partner_list.clone().is_empty() == false {
        let double_to_check = iter_conflict_list.next().unwrap();
        if conflict_partner_list.clone().contains(double_to_check) {
            match conflict_partner_list.clone().iter().filter(|filter_same| { filter_same != &double_to_check }).find(|d_opponents| {
                has_conflict(former_opponents, double_to_check, d_opponents, meeting_score) == false
            }) {
                None => {
                    return_none = true;
                    break;
                }
                Some(opponents) => {
                    matches.push(DoubleMatch { double_a: double_to_check.clone().clone(), double_b: opponents.clone().clone() });
                    set_former_opponents(former_opponents, double_to_check, opponents);
                    conflict_partner_list.retain(|e| e != double_to_check);
                    conflict_partner_list.retain(|e| e != opponents);
                }
            }
        }
    }

    r.rotate_right(1);
    if return_none { None } else { Some(matches) }
}

fn has_conflict(former_opponents: &HashMap<usize, Vec<usize>>, double_a: &(usize, usize), double_b: &(usize, usize), meeting_score: usize) -> bool {
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

fn set_former_opponents(former_opponents: &mut HashMap<usize, Vec<usize>>, double_a: &(usize, usize), double_b: &(usize, usize)) {
    set_player_opponents(former_opponents, &double_a.0, &double_b.0, &double_b.1);
    set_player_opponents(former_opponents, &double_a.1, &double_b.0, &double_b.1);
    set_player_opponents(former_opponents, &double_b.0, &double_a.0, &double_a.1);
    set_player_opponents(former_opponents, &double_b.1, &double_a.0, &double_a.1);
}

fn set_player_opponents(former_opponents: &mut HashMap<usize, Vec<usize>>, player: &usize, opponent_1: &usize, opponent_2: &usize) {
    match former_opponents.get_mut(player) {
        None => {}
        Some(fo) => {
            fo.push(opponent_1.clone());
            fo.push(opponent_2.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{draw_doubles, DoubleMatch, draw_singles, SingleMatch};

    #[test]
    fn test_36_4_12() {
        let number_of_player = 144;
        let number_of_rounds = 12;

        let rounds = draw_doubles(number_of_player, number_of_rounds);

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), number_of_player / 4);
            for i in 0..number_of_player {
                let fp = r.matches.iter().filter(|p| {
                    p.double_a.0 == i || p.double_a.1 == i || p.double_b.0 == i || p.double_b.1 == i
                }).collect::<Vec<&DoubleMatch>>();
                assert_eq!(fp.len(), 1);
            }
        });
    }
    #[test]
    fn test_10_4_12() {
        let number_of_player = 40;
        let number_of_rounds = 12;

        let rounds = draw_doubles(number_of_player, number_of_rounds);

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), number_of_player / 4);
            for i in 0..number_of_player {
                let fp = r.matches.iter().filter(|p| {
                    p.double_a.0 == i || p.double_a.1 == i || p.double_b.0 == i || p.double_b.1 == i
                }).collect::<Vec<&DoubleMatch>>();
                assert_eq!(fp.len(), 1);
            }
        });
    }
    #[test]
    fn test_10_2_12_even() {
        let number_of_player = 20;
        let number_of_rounds = 12;

        let rounds = draw_singles(number_of_player, number_of_rounds);

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), number_of_player / 2);
            for i in 0..number_of_player {
                let fp = r.matches.iter().filter(|p| {
                    p.a == i || p.b == i
                }).collect::<Vec<&SingleMatch>>();
                assert_eq!(fp.len(), 1);
            }
        });
    }
    #[test]
    fn test_10_2_12_odd() {
        let number_of_player = 19;
        let number_of_rounds = 12;

        let rounds = draw_singles(number_of_player, number_of_rounds);

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.matches.len(), (number_of_player + 1) / 2);
            for i in 0..(number_of_player + 1) {
                let fp = r.matches.iter().filter(|p| {
                    p.a == i || p.b == i
                }).collect::<Vec<&SingleMatch>>();
                assert_eq!(fp.len(), 1);
            }
        });
    }
}