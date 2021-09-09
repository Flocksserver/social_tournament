use crate::double::draw_doubles;

pub mod model;
pub mod single;
pub mod double;


pub fn run() {
    let number_of_player = 144;
    let number_of_rounds = 12;

    let _rounds = draw_doubles(number_of_player, number_of_rounds);

    // TODO distribute matches in each round to number of tables
}

