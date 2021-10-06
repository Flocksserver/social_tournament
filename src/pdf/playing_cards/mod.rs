use crate::pdf::playing_cards::clubs_ace::CLUBS_ACE;
use crate::pdf::playing_cards::spades_ace::SPADES_ACE;
use crate::pdf::playing_cards::hearts_ace::HEARTS_ACE;
use crate::pdf::playing_cards::diamonds_ace::DIAMONDS_ACE;
use crate::pdf::playing_cards::clubs_king::CLUBS_KING;
use crate::pdf::playing_cards::spades_king::SPADES_KING;
use crate::pdf::playing_cards::hearts_king::HEARTS_KING;
use crate::pdf::playing_cards::diamonds_king::DIAMONDS_KING;
use crate::pdf::playing_cards::clubs_queen::CLUBS_QUEEN;
use crate::pdf::playing_cards::spades_queen::SPADES_QUEEN;
use crate::pdf::playing_cards::hearts_queen::HEARTS_QUEEN;
use crate::pdf::playing_cards::diamonds_queen::DIAMONDS_QUEEN;
use crate::pdf::playing_cards::clubs_jack::CLUBS_JACK;
use crate::pdf::playing_cards::spades_jack::SPADES_JACK;
use crate::pdf::playing_cards::hearts_jack::HEARTS_JACK;
use crate::pdf::playing_cards::diamonds_jack::DIAMONDS_JACK;
use crate::pdf::playing_cards::clubs_10::CLUBS_10;
use crate::pdf::playing_cards::spades_10::SPADES_10;
use crate::pdf::playing_cards::hearts_10::HEARTS_10;
use crate::pdf::playing_cards::diamonds_10::DIAMONDS_10;
use crate::pdf::playing_cards::clubs_9::CLUBS_9;
use crate::pdf::playing_cards::spades_9::SPADES_9;
use crate::pdf::playing_cards::hearts_9::HEARTS_9;
use crate::pdf::playing_cards::diamonds_9::DIAMONDS_9;
use crate::pdf::playing_cards::clubs_8::CLUBS_8;
use crate::pdf::playing_cards::spades_8::SPADES_8;
use crate::pdf::playing_cards::hearts_8::HEARTS_8;
use crate::pdf::playing_cards::diamonds_8::DIAMONDS_8;
use crate::pdf::playing_cards::clubs_7::CLUBS_7;
use crate::pdf::playing_cards::spades_7::SPADES_7;
use crate::pdf::playing_cards::hearts_7::HEARTS_7;
use crate::pdf::playing_cards::diamonds_7::DIAMONDS_7;
use crate::pdf::playing_cards::clubs_6::CLUBS_6;
use crate::pdf::playing_cards::spades_6::SPADES_6;
use crate::pdf::playing_cards::hearts_6::HEARTS_6;
use crate::pdf::playing_cards::diamonds_6::DIAMONDS_6;
use crate::pdf::playing_cards::clubs_5::CLUBS_5;
use crate::pdf::playing_cards::spades_5::SPADES_5;
use crate::pdf::playing_cards::hearts_5::HEARTS_5;
use crate::pdf::playing_cards::diamonds_5::DIAMONDS_5;
use crate::pdf::playing_cards::clubs_4::CLUBS_4;
use crate::pdf::playing_cards::spades_4::SPADES_4;
use crate::pdf::playing_cards::hearts_4::HEARTS_4;
use crate::pdf::playing_cards::diamonds_4::DIAMONDS_4;
use crate::pdf::playing_cards::clubs_3::CLUBS_3;
use crate::pdf::playing_cards::spades_3::SPADES_3;
use crate::pdf::playing_cards::hearts_3::HEARTS_3;
use crate::pdf::playing_cards::diamonds_3::DIAMONDS_3;
use crate::pdf::playing_cards::clubs_2::CLUBS_2;
use crate::pdf::playing_cards::spades_2::SPADES_2;
use crate::pdf::playing_cards::hearts_2::HEARTS_2;
use crate::pdf::playing_cards::diamonds_2::DIAMONDS_2;
use crate::pdf::playing_cards::joker::JOKER;

mod clubs_ace;
mod clubs_king;
mod clubs_queen;
mod clubs_jack;
mod clubs_10;
mod clubs_9;
mod clubs_8;
mod clubs_7;
mod clubs_6;
mod clubs_5;
mod clubs_4;
mod clubs_3;
mod clubs_2;

mod spades_ace;
mod spades_king;
mod spades_queen;
mod spades_jack;
mod spades_10;
mod spades_9;
mod spades_8;
mod spades_7;
mod spades_6;
mod spades_5;
mod spades_4;
mod spades_3;
mod spades_2;

mod hearts_ace;
mod hearts_king;
mod hearts_queen;
mod hearts_jack;
mod hearts_10;
mod hearts_9;
mod hearts_8;
mod hearts_7;
mod hearts_6;
mod hearts_5;
mod hearts_4;
mod hearts_3;
mod hearts_2;

mod diamonds_ace;
mod diamonds_king;
mod diamonds_queen;
mod diamonds_jack;
mod diamonds_10;
mod diamonds_9;
mod diamonds_8;
mod diamonds_7;
mod diamonds_6;
mod diamonds_5;
mod diamonds_4;
mod diamonds_3;
mod diamonds_2;
mod joker;

pub static CARD_NAME_MAP_DE: [&str; 52] = [
    "Kreuz Ass", "Pik Ass", "Herz Ass", "Karo Ass",
    "Kreuz König", "Pik König", "Herz König", "Karo König",
    "Kreuz Dame", "Pik Dame", "Herz Dame", "Karo Dame",
    "Kreuz Bube", "Pik Bube", "Herz Bube", "Karo Bube",
    "Kreuz 10", "Pik 10", "Herz 10", "Karo 10",
    "Kreuz 9", "Pik 9", "Herz 9", "Karo 9",
    "Kreuz 8", "Pik 8", "Herz 8", "Karo 8",
    "Kreuz 7", "Pik 7", "Herz 7", "Karo 7",
    "Kreuz 6", "Pik 6", "Herz 6", "Karo 6",
    "Kreuz 5", "Pik 5", "Herz 5", "Karo 5",
    "Kreuz 4", "Pik 4", "Herz 4", "Karo 4",
    "Kreuz 3", "Pik 3", "Herz 3", "Karo 3",
    "Kreuz 2", "Pik 2", "Herz 2", "Karo 2",
];

pub static CARD_NAME_MAP_EN: [&str; 52] = [
    "Clubs Ace", "Spades Ace", "Hearts Ace", "Diamonds Ace",
    "Clubs King", "Spades King", "Hearts King", "Diamonds King",
    "Clubs Queen", "Spades Queen", "Hearts Queen", "Diamonds Queen",
    "Clubs Jack", "Spades Jack", "Hearts Jack", "Diamonds Jack",
    "Clubs 10", "Spades 10", "Hearts 10", "Diamonds 10",
    "Clubs 9", "Spades 9", "Hearts 9", "Diamonds 9",
    "Clubs 8", "Spades 8", "Hearts 8", "Diamonds 8",
    "Clubs 7", "Spades 7", "Hearts 7", "Diamonds 7",
    "Clubs 6", "Spades 6", "Hearts 6", "Diamonds 6",
    "Clubs 5", "Spades 5", "Hearts 5", "Diamonds 5",
    "Clubs 4", "Spades 4", "Hearts 4", "Diamonds 4",
    "Clubs 3", "Spades 3", "Hearts 3", "Diamonds 3",
    "Clubs 2", "Spades 2", "Hearts 2", "Diamonds 2",
];

pub fn get_card_image(position: usize) -> String {
    match position {
        0 => String::from(CLUBS_ACE),
        1 => String::from(SPADES_ACE),
        2 => String::from(HEARTS_ACE),
        3 => String::from(DIAMONDS_ACE),
        4 => String::from(CLUBS_KING),
        5 => String::from(SPADES_KING),
        6 => String::from(HEARTS_KING),
        7 => String::from(DIAMONDS_KING),
        8 => String::from(CLUBS_QUEEN),
        9 => String::from(SPADES_QUEEN),
        10 => String::from(HEARTS_QUEEN),
        11 => String::from(DIAMONDS_QUEEN),
        12 => String::from(CLUBS_JACK),
        13 => String::from(SPADES_JACK),
        14 => String::from(HEARTS_JACK),
        15 => String::from(DIAMONDS_JACK),
        16 => String::from(CLUBS_10),
        17 => String::from(SPADES_10),
        18 => String::from(HEARTS_10),
        19 => String::from(DIAMONDS_10),
        20 => String::from(CLUBS_9),
        21 => String::from(SPADES_9),
        22 => String::from(HEARTS_9),
        23 => String::from(DIAMONDS_9),
        24 => String::from(CLUBS_8),
        25 => String::from(SPADES_8),
        26 => String::from(HEARTS_8),
        27 => String::from(DIAMONDS_8),
        28 => String::from(CLUBS_7),
        29 => String::from(SPADES_7),
        30 => String::from(HEARTS_7),
        31 => String::from(DIAMONDS_7),
        32 => String::from(CLUBS_6),
        33 => String::from(SPADES_6),
        34 => String::from(HEARTS_6),
        35 => String::from(DIAMONDS_6),
        36 => String::from(CLUBS_5),
        37 => String::from(SPADES_5),
        38 => String::from(HEARTS_5),
        39 => String::from(DIAMONDS_5),
        40 => String::from(CLUBS_4),
        41 => String::from(SPADES_4),
        42 => String::from(HEARTS_4),
        43 => String::from(DIAMONDS_4),
        44 => String::from(CLUBS_3),
        45 => String::from(SPADES_3),
        46 => String::from(HEARTS_3),
        47 => String::from(DIAMONDS_3),
        48 => String::from(CLUBS_2),
        49 => String::from(SPADES_2),
        50 => String::from(HEARTS_2),
        51 => String::from(DIAMONDS_2),
        _ => String::from(JOKER)
    }
}