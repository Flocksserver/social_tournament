use round_robin_tournament::round_robin_tournament::retrieve_encounters;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Player {
    id: u32,
}

#[derive(Debug, Clone)]
pub struct Round {
    pub round_number: usize,
    pub games: Vec<Game>,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub double_a: (u32, u32),
    pub double_b: (u32, u32),
}

fn main() {
    let number_of_player = 144;
    let number_of_rounds = 12;

    let _rounds = create_double_tournament(number_of_player, number_of_rounds);

    // TODO distribute games in each round to number of tables
}

fn create_double_tournament(number_of_player: usize, number_of_rounds: usize) -> Vec<Round> {
    let mut players_map: HashMap<u32, Player> = HashMap::new();
    let mut former_opponents: HashMap<u32, Vec<u32>> = HashMap::new();
    for i in 0..number_of_player {
        players_map.insert(i as u32, Player { id: i as u32 });
        former_opponents.insert(i as u32, vec![]);
    }
    let tournament = retrieve_encounters(number_of_player as u32);
    let mut tournament_rounds_used: Vec<Vec<(u32, u32)>> = Vec::new();


    let mut rounds: Vec<Round> = Vec::new();

    for i in 0..number_of_rounds {
        let mut has_solution = false;
        let mut meeting_score = 0;

        while has_solution == false {
            let tru = tournament_rounds_used.clone();
            let mut t_iter = tournament.clone().into_iter().filter(|d| tru.contains(d) == false).peekable();
            while t_iter.peek().is_some() && has_solution == false {
                let mut current_round = t_iter.next().unwrap();
                for _rotations in 0..current_round.len() {
                    match get_games(&mut current_round, &mut former_opponents, meeting_score) {
                        None => {}
                        Some(g) => {
                            tournament_rounds_used.push(current_round);
                            rounds.push(Round { round_number: i, games: g.clone() });
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

fn get_games(r: &mut Vec<(u32, u32)>, former_opponents: &mut HashMap<u32, Vec<u32>>, meeting_score: i32) -> Option<Vec<Game>> {
    let mut games = Vec::new();
    let mut return_none = false;

    let mut conflict_list: Vec<&(u32, u32)> = Vec::new();

    for p in 0..r.len() {
        if p % 2 == 0 {
            let pos1 = p;
            let pos2 = p + 1;
            let double_a: &(u32, u32) = r.get(pos1 as usize).unwrap();
            let double_b: &(u32, u32) = r.get(pos2 as usize).unwrap();

            if has_conflict(former_opponents, double_a, double_b, meeting_score) {
                conflict_list.push(double_a);
                conflict_list.push(double_b)
            } else {
                games.push(Game { double_a: double_a.clone(), double_b: double_b.clone() });
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
                    games.push(Game { double_a: double_to_check.clone().clone(), double_b: opponents.clone().clone() });
                    set_former_opponents(former_opponents, double_to_check, opponents);
                    conflict_partner_list.retain(|e| e != double_to_check);
                    conflict_partner_list.retain(|e| e != opponents);
                }
            }
        }
    }

    r.rotate_right(1);
    if return_none { None } else { Some(games) }
}

fn has_conflict(former_opponents: &HashMap<u32, Vec<u32>>, double_a: &(u32, u32), double_b: &(u32, u32), meeting_score: i32) -> bool {
    check_player_conflicts(former_opponents, &double_a.0, &double_b.0, &double_b.1, meeting_score) ||
        check_player_conflicts(former_opponents, &double_a.1, &double_b.0, &double_b.1, meeting_score) ||
        check_player_conflicts(former_opponents, &double_b.0, &double_a.0, &double_a.1, meeting_score) ||
        check_player_conflicts(former_opponents, &double_b.1, &double_a.0, &double_a.1, meeting_score)
}

fn check_player_conflicts(former_opponents: &HashMap<u32, Vec<u32>>, player: &u32, opponent_1: &u32, opponent_2: &u32, meeting_score: i32) -> bool {
    match former_opponents.get(player) {
        None => false,
        Some(fo) => {
            let a = fo.iter().filter(|p| { p == &opponent_1 }).collect::<Vec<&u32>>();
            let b = fo.iter().filter(|p| { p == &opponent_2 }).collect::<Vec<&u32>>();
            a.len() > meeting_score as usize || b.len() > meeting_score as usize
        }
    }
}

fn set_former_opponents(former_opponents: &mut HashMap<u32, Vec<u32>>, double_a: &(u32, u32), double_b: &(u32, u32)) {
    set_player_opponents(former_opponents, &double_a.0, &double_b.0, &double_b.1);
    set_player_opponents(former_opponents, &double_a.1, &double_b.0, &double_b.1);
    set_player_opponents(former_opponents, &double_b.0, &double_a.0, &double_a.1);
    set_player_opponents(former_opponents, &double_b.1, &double_a.0, &double_a.1);
}

fn set_player_opponents(former_opponents: &mut HashMap<u32, Vec<u32>>, player: &u32, opponent_1: &u32, opponent_2: &u32) {
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
    use crate::{create_double_tournament, Game};

    #[test]
    fn test_36_4_12() {
        let number_of_player = 144;
        let number_of_rounds = 12;

        let rounds = create_double_tournament(number_of_player, number_of_rounds);

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.games.len(), number_of_player / 4);
            for i in 0..number_of_player {
                let fp = r.games.iter().filter(|p| {
                    p.double_a.0 == i as u32 || p.double_a.1 == i as u32 || p.double_b.0 == i as u32 || p.double_b.1 == i as u32
                }).collect::<Vec<&Game>>();
                assert_eq!(fp.len(), 1);
            }
        });
    }
    #[test]
    fn test_10_4_12() {
        let number_of_player = 40;
        let number_of_rounds = 12;

        let rounds = create_double_tournament(number_of_player, number_of_rounds);

        assert_eq!(rounds.len(), number_of_rounds);

        rounds.iter().for_each(|r| {
            assert_eq!(r.games.len(), number_of_player / 4);
            for i in 0..number_of_player {
                let fp = r.games.iter().filter(|p| {
                    p.double_a.0 == i as u32 || p.double_a.1 == i as u32 || p.double_b.0 == i as u32 || p.double_b.1 == i as u32
                }).collect::<Vec<&Game>>();
                assert_eq!(fp.len(), 1);
            }
        });
    }
}