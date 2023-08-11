use printpdf::{Mm, Line, Point, Image, BuiltinFont, PdfDocument};
use crate::pdf::language::{get_language_value, Language};
use crate::pdf::language::LanguageKey::{BYE, TABLE, ROUND, GREETING, HINT_CONTENT_PRINT_SOCIAL_DOUBLE, HINT_TITLE_PRINT_SOCIAL_DOUBLE, HINT_CONTENT_SUM_EXAMPLE_TWO_SOCIAL_DOUBLE, HINT_CONTENT_SUM_EXAMPLE_ONE_SOCIAL_DOUBLE, HINT_TITLE_SUM_SOCIAL_DOUBLE, HINT_CONTENT_SUM_SOCIAL_DOUBLE, ALTERNATIVE_THREE_SOCIAL_DOUBLE, ALTERNATIVE, ALTERNATIVE_TWO_SOCIAL_DOUBLE, ALTERNATIVE_ONE_SOCIAL_DOUBLE, SPECIAL_AFTER_ROUND_CONTENT_SOCIAL_DOUBLE, SPECIAL_AFTER_ROUND_TITLE_SOCIAL_DOUBLE, SPECIAL_BEFORE_ROUND_CONTENT_SOCIAL_DOUBLE, SPECIAL_BEFORE_ROUND_TITLE_SOCIAL_DOUBLE, BENEFITS_CONTENT_SOCIAL_DOUBLE, BENEFITS_TITLE_SOCIAL_DOUBLE, IMPL_PROPOSAL_SOCIAL_DOUBLE, IMPL_PROPOSAL, FUN_CONTENT_SOCIAL_DOUBLE, SUBTITLE_SOCIAL_SUM_DOUBLE, INTRO_SOCIAL_DOUBLE, TITLE_SOCIAL_DOUBLE};
use crate::pdf::playing_cards::{get_card_image, CARD_NAME_MAP_DE, CARD_NAME_MAP_EN};
use std::io::Cursor;
use base64::decode;
use printpdf::image::codecs::jpeg::JpegDecoder;
use std::collections::HashMap;
use crate::double::DrawOption;
use crate::table::Table;
use crate::{Round, Match};

#[derive(Debug, Clone)]
struct MatchContainerDouble {
    pub is_bye: bool,
    pub me: usize,
    pub mate: usize,
    pub opponents_a: usize,
    pub opponents_b: usize,
    pub table: String,
    pub iteration: Option<String>,
}

#[derive(Debug, Clone)]
struct Card {
    pub name: String,
    pub image: String,
}

#[derive(Debug, Clone)]
struct PlayerContainerDouble {
    pub id: usize,
    pub card: Card,
    pub rounds: Vec<MatchContainerDouble>,
}

pub(crate) fn create_route_cards_for_double(tournament: &Vec<Round>, tables: &Vec<Vec<Table>>, number_of_players: usize, draw_option: DrawOption, number_of_tables: usize, language: Language) -> Vec<u8> {
    let document_name = format!("Double Tournament for {} players on {} tables", number_of_players, number_of_tables);

    let mut players: HashMap<usize, PlayerContainerDouble> = HashMap::new();

    let ciel_number_of_players = if number_of_players % 4 == 0 { number_of_players } else { number_of_players + 4 - (number_of_players % 4) };

    for i in 0..ciel_number_of_players {
        let mut player = PlayerContainerDouble {
            id: i,
            card: if is_dummy_double_player(i, number_of_players) {
                Card { name: "-".to_string(), image: "".to_string() }
            } else {
                get_player_card(number_of_players, i, language)
            },
            rounds: vec![],
        };
        tournament.iter().for_each(|r| {
            let dm = r.matches.iter().find(|dm| {
                match dm {
                    Match::SingleMatch { .. } => {false}
                    Match::DoubleMatch { double_a, double_b } => {
                        double_a.0 == i || double_a.1 == i || double_b.0 == i || double_b.1 == i
                    }
                }
            });
            let match_container = if is_byes(number_of_players, dm, draw_option.clone()) {
                MatchContainerDouble {
                    is_bye: true,
                    me: i,
                    mate: i,
                    opponents_a: i,
                    opponents_b: i,
                    table: String::from(""),
                    iteration: None,
                }
            } else {
                let p = dm.unwrap();
                let other_player = get_other_player(number_of_players, i, p);
                let position = r.matches.iter().position(|dm| {
                    match (p, dm) {
                        (Match::DoubleMatch { double_a: p_double_a, double_b: p_double_b }, Match::DoubleMatch { double_a: dm_double_a, double_b: dm_double_b }) => {
                            dm_double_a == p_double_a && dm_double_b == p_double_b
                        }
                        _ => false
                    }
                }).unwrap();
                let table = tables.get(r.round_number).unwrap().get(position).unwrap();
                MatchContainerDouble {
                    is_bye: false,
                    me: other_player.0,
                    mate: other_player.1,
                    opponents_a: other_player.2,
                    opponents_b: other_player.3,
                    table: format!("{}", table.table_number + 1),
                    iteration: if number_of_tables >= number_of_players / 4 { None } else { Some(format!("{}", table.occupied_number + 1)) },
                }
            };
            player.rounds.push(match_container)
        });
        players.insert(i, player);
    }

    players.iter().for_each(|p| println!("Player: {} -> {}", p.1.id, p.1.card.name));

    create_pdf_double_tournament(document_name, players, language)
}


fn is_byes(number_of_players: usize, dm: Option<&Match>, draw_option: DrawOption) -> bool {
    if dm.is_none() {
        true
    } else {
        match dm.unwrap() {
            Match::DoubleMatch { double_a, double_b } => {
                if is_dummy_double_player(double_a.0, number_of_players) || is_dummy_double_player(double_a.1, number_of_players) ||
                    is_dummy_double_player(double_b.0, number_of_players) || is_dummy_double_player(double_b.1, number_of_players) {
                    match draw_option {
                        DrawOption::AllInAction => { false }
                        DrawOption::ForceDoubleOnly => { true }
                        DrawOption::ValidCompositionsOnly => {
                            let bool_vec = vec![
                                is_dummy_double_player(double_a.0, number_of_players),
                                is_dummy_double_player(double_a.1, number_of_players),
                                is_dummy_double_player(double_b.0, number_of_players),
                                is_dummy_double_player(double_b.1, number_of_players),
                            ];
                            let dummy_player_number = bool_vec.iter().filter(|b| b == &&true).collect::<Vec<&bool>>().len();
                            if dummy_player_number % 2 != 0 {
                                true
                            } else {
                                false
                            }
                        }
                    }
                } else {
                    false
                }
            }
            _ => false
        }
    }
}

fn get_other_player(number_of_players: usize, i: usize, p: &Match) -> (usize, usize, usize, usize) {
    match p {
        Match::DoubleMatch { double_a, double_b } => {
            let players = if double_a.0 == i {
                (i, double_a.1, double_b.0, double_b.1)
            } else if double_a.1 == i {
                (i, double_a.0, double_b.0, double_b.1)
            } else if double_b.0 == i {
                (i, double_b.1, double_a.0, double_a.1)
            } else {
                (i, double_b.0, double_a.0, double_a.1)
            };
            if is_dummy_double_player(players.0, number_of_players) == false && is_dummy_double_player(players.1, number_of_players) == false &&
                is_dummy_double_player(players.2, number_of_players) && is_dummy_double_player(players.3, number_of_players) {
                (players.0, players.2, players.1, players.3)
            } else {
                players
            }
        }
        _ => {(0,0,0,0)}
    }
}

fn is_dummy_double_player(i: usize, number_of_players: usize) -> bool {
    if number_of_players % 4 == 1 {
        if i == number_of_players || i == number_of_players + 1 || i == number_of_players + 2 {
            true
        } else {
            false
        }
    } else if number_of_players % 4 == 2 {
        if i == number_of_players || i == number_of_players + 1 {
            true
        } else {
            false
        }
    } else if number_of_players % 4 == 3 {
        if i == number_of_players {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn get_player_card(number_of_player: usize, player_number: usize, language: Language) -> Card {
    let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    let name_extension = if number_of_player >= 52 {
        let div = player_number / 51;
        if div < 26 { Some(alphabet.get(div).unwrap().to_string()) } else { None }
    } else { None };

    let name = match name_extension {
        None => {
            match language {
                Language::DE => { format!("{}", CARD_NAME_MAP_DE.get(player_number % 51).unwrap()) }
                Language::EN => { format!("{}", CARD_NAME_MAP_EN.get(player_number % 51).unwrap()) }
            }
        }
        Some(e) => {
            match language {
                Language::DE => { format!("{} {}", CARD_NAME_MAP_DE.get(player_number % 51).unwrap(), e) }
                Language::EN => { format!("{} {}", CARD_NAME_MAP_EN.get(player_number % 51).unwrap(), e) }
            }
        }
    };


    Card { name, image: get_card_image(player_number % 51) }
}

fn create_pdf_double_tournament(title: String, players: HashMap<usize, PlayerContainerDouble>, language: Language) -> Vec<u8> {
    let page_width = 297.0;
    let page_height = 210.0;

    let (doc, page1, layer1) = PdfDocument::new(&title, Mm(page_height), Mm(page_width), "Layer 1");
    let helvetica_regular = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
    let helvetica_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();

    let p1_layer = doc.get_page(page1).get_layer(layer1);
    // Header
    p1_layer.begin_text_section();
    p1_layer.set_font(&helvetica_bold, 30.0);
    p1_layer.set_line_height(34.0);
    p1_layer.set_text_cursor(Mm(20.0), Mm(275.0));
    p1_layer.write_text(get_language_value(language, TITLE_SOCIAL_DOUBLE), &helvetica_bold);
    p1_layer.add_line_break();
    // Opening
    p1_layer.set_font(&helvetica_regular, 10.0);
    p1_layer.set_line_height(12.0);
    let text_source_multiline = get_language_value(language, INTRO_SOCIAL_DOUBLE);
    for line in text_source_multiline.lines() {
        p1_layer.write_text(line, &helvetica_regular);
        p1_layer.add_line_break();
    }

    // SubHead
    p1_layer.set_font(&helvetica_bold, 18.0);
    p1_layer.set_line_height(22.0);
    p1_layer.add_line_break();
    p1_layer.set_line_height(18.0);
    p1_layer.write_text(get_language_value(language, SUBTITLE_SOCIAL_SUM_DOUBLE), &helvetica_bold);
    p1_layer.set_line_height(22.0);
    p1_layer.add_line_break();

    // Option
    p1_layer.set_font(&helvetica_bold, 10.0);
    p1_layer.set_line_height(12.0);
    p1_layer.write_text(get_language_value(language, IMPL_PROPOSAL), &helvetica_bold);
    p1_layer.set_font(&helvetica_regular, 10.0);
    let text_source_multiline = get_language_value(language, IMPL_PROPOSAL_SOCIAL_DOUBLE);
    for line in text_source_multiline.lines() {
        p1_layer.write_text(line, &helvetica_regular);
        p1_layer.add_line_break();
    }

    // Benefits
    p1_layer.add_line_break();
    p1_layer.write_text(get_language_value(language, BENEFITS_TITLE_SOCIAL_DOUBLE), &helvetica_regular);
    p1_layer.add_line_break();
    let text_source_multiline = get_language_value(language, BENEFITS_CONTENT_SOCIAL_DOUBLE);
    for line in text_source_multiline.lines() {
        p1_layer.write_text("    + ", &helvetica_regular);
        p1_layer.write_text(line, &helvetica_regular);
        p1_layer.add_line_break();
    }
    // Additions
    p1_layer.add_line_break();
    let text_source_multiline = get_language_value(language, FUN_CONTENT_SOCIAL_DOUBLE);
    for line in text_source_multiline.lines() {
        p1_layer.write_text(line, &helvetica_regular);
        p1_layer.add_line_break();
    }
    // Before Round
    p1_layer.add_line_break();
    p1_layer.write_text(get_language_value(language, SPECIAL_BEFORE_ROUND_TITLE_SOCIAL_DOUBLE), &helvetica_regular);
    p1_layer.add_line_break();
    let text_source_multiline = get_language_value(language, SPECIAL_BEFORE_ROUND_CONTENT_SOCIAL_DOUBLE);
    for line in text_source_multiline.lines() {
        p1_layer.write_text("    + ", &helvetica_regular);
        p1_layer.write_text(line, &helvetica_regular);
        p1_layer.add_line_break();
    }
    // After Round
    p1_layer.add_line_break();
    p1_layer.write_text(get_language_value(language, SPECIAL_AFTER_ROUND_TITLE_SOCIAL_DOUBLE), &helvetica_regular);
    p1_layer.add_line_break();
    let text_source_multiline = get_language_value(language, SPECIAL_AFTER_ROUND_CONTENT_SOCIAL_DOUBLE);
    for line in text_source_multiline.lines() {
        p1_layer.write_text("    + ", &helvetica_regular);
        p1_layer.write_text(line, &helvetica_regular);
        p1_layer.add_line_break();
    }
    // Alternative I
    p1_layer.add_line_break();
    p1_layer.set_font(&helvetica_bold, 10.0);
    p1_layer.write_text(get_language_value(language, ALTERNATIVE), &helvetica_bold);
    p1_layer.set_font(&helvetica_regular, 10.0);
    let text_source_multiline = get_language_value(language, ALTERNATIVE_ONE_SOCIAL_DOUBLE);
    for line in text_source_multiline.lines() {
        p1_layer.write_text(line, &helvetica_regular);
        p1_layer.add_line_break();
    }
    // Alternative II
    p1_layer.set_font(&helvetica_bold, 10.0);
    p1_layer.write_text(get_language_value(language, ALTERNATIVE), &helvetica_bold);
    p1_layer.set_font(&helvetica_regular, 10.0);
    let text_source_multiline = get_language_value(language, ALTERNATIVE_TWO_SOCIAL_DOUBLE);
    for line in text_source_multiline.lines() {
        p1_layer.write_text(line, &helvetica_regular);
        p1_layer.add_line_break();
    }
    // Alternative III
    p1_layer.set_font(&helvetica_bold, 10.0);
    p1_layer.write_text(get_language_value(language, ALTERNATIVE), &helvetica_bold);
    p1_layer.set_font(&helvetica_regular, 10.0);
    let text_source_multiline = get_language_value(language, ALTERNATIVE_THREE_SOCIAL_DOUBLE);
    for line in text_source_multiline.lines() {
        p1_layer.write_text(line, &helvetica_regular);
        p1_layer.add_line_break();
    }
    // Hint Sum Up
    p1_layer.add_line_break();
    p1_layer.set_font(&helvetica_bold, 10.0);
    p1_layer.write_text(get_language_value(language, HINT_TITLE_SUM_SOCIAL_DOUBLE), &helvetica_bold);
    p1_layer.set_font(&helvetica_regular, 10.0);
    let text_source_multiline = get_language_value(language, HINT_CONTENT_SUM_SOCIAL_DOUBLE);
    for line in text_source_multiline.lines() {
        p1_layer.write_text(line, &helvetica_regular);
        p1_layer.add_line_break();
    }
    p1_layer.write_text("    + ", &helvetica_regular);
    p1_layer.write_text(get_language_value(language, HINT_CONTENT_SUM_EXAMPLE_ONE_SOCIAL_DOUBLE), &helvetica_regular);
    p1_layer.add_line_break();
    p1_layer.write_text("    + ", &helvetica_regular);
    p1_layer.write_text(get_language_value(language, HINT_CONTENT_SUM_EXAMPLE_TWO_SOCIAL_DOUBLE), &helvetica_regular);
    p1_layer.add_line_break();
    // Hint Print
    p1_layer.add_line_break();
    p1_layer.set_font(&helvetica_bold, 10.0);
    p1_layer.write_text(get_language_value(language, HINT_TITLE_PRINT_SOCIAL_DOUBLE), &helvetica_bold);
    p1_layer.set_font(&helvetica_regular, 10.0);
    let text_source_multiline = get_language_value(language, HINT_CONTENT_PRINT_SOCIAL_DOUBLE);
    for line in text_source_multiline.lines() {
        p1_layer.write_text(line, &helvetica_regular);
        p1_layer.add_line_break();
    }

    p1_layer.add_line_break();
    p1_layer.write_text(get_language_value(language, GREETING), &helvetica_regular);
    p1_layer.add_line_break();
    p1_layer.add_line_break();
    p1_layer.set_font(&helvetica_bold, 10.0);
    p1_layer.write_text("Marcel Kaufmann", &helvetica_regular);

    p1_layer.end_text_section();

    players.iter().filter(|pf| pf.1.card.name != "-").for_each(|p| {
        let (current_page, current_page_layer1) = doc.add_page(Mm(page_width), Mm(page_height), "Page, Layer 1");

        let card = &decode(p.clone().1.card.image.clone().as_bytes()).unwrap()[..];
        let mut reader = Cursor::new(card.as_ref());

        //DEFAULT FALLBACK CARD
        let card_fallback = &decode(get_card_image(52).as_bytes()).unwrap()[..];
        let mut reader_fallback = Cursor::new(card_fallback.as_ref());

        let decoder = JpegDecoder::new(&mut reader).unwrap_or(JpegDecoder::new(&mut reader_fallback).unwrap());

        let image = Image::try_from(decoder).unwrap();

        let mut current_layer = doc.get_page(current_page).get_layer(current_page_layer1);
        image.add_to_layer(current_layer.clone(), Some(Mm(2.0 * 58.5)), Some(Mm(210.0 - 2.0 * 41.5)), Some(90.0), Some(2.0), Some(2.0), Some(300.0));


        // Card Name
        current_layer.use_text(p.1.card.name.clone(), if language == Language::DE { 65.0 } else { 50.0 }, Mm(122.0), Mm(186.0), &helvetica_bold);

        // Player Name
        current_layer.use_text("Name: _______________", 40.0, Mm(122.0), Mm(128.0), &helvetica_regular);


        let rounds_per_page = 4;
        let padding_x = 5.0;
        let mut padding_y = page_height * 0.59;
        let padding_y_row = 4.0;
        let mut available_row_on_page = 3;
        let mut magic_row_break_addition = 0;

        let width_one_round_table = page_width * 0.228;
        let height_one_round_table = page_height * 0.17;


        for r in p.1.rounds.iter().enumerate() {
            //for r in 0..70{
            let i = r.0;
            current_layer = if i >= 12 && (i - 12) % 20 == 0 {
                let (current_page, current_page_layer1) = doc.add_page(Mm(page_width), Mm(page_height), "Page, Layer 1");
                padding_y = page_height * 0.98;
                available_row_on_page = 5;
                magic_row_break_addition = 2;
                doc.get_page(current_page).get_layer(current_page_layer1)
            } else {
                current_layer
            };

            let p1_x = ((i % rounds_per_page) as f64 + 1.0) * padding_x + ((i % rounds_per_page) as f64) * width_one_round_table;
            let p1_y = padding_y - (((i / rounds_per_page + magic_row_break_addition) % available_row_on_page) as f64 + 1.0) * padding_y_row - ((i / rounds_per_page + magic_row_break_addition) % available_row_on_page) as f64 * height_one_round_table;

            let p2_x = ((i % rounds_per_page) as f64 + 1.0) * padding_x + ((i % rounds_per_page) as f64 + 1.0) * width_one_round_table;
            let p2_y = p1_y.clone();

            let p3_x = p2_x.clone();
            let p3_y = padding_y - height_one_round_table - (((i / rounds_per_page + magic_row_break_addition) % available_row_on_page) as f64 + 1.0) * padding_y_row - ((i / rounds_per_page + magic_row_break_addition) % available_row_on_page) as f64 * height_one_round_table;

            let p4_x = p1_x.clone();
            let p4_y = p3_y.clone();

            let points = vec![
                (Point::new(Mm(p1_x), Mm(p1_y)), false),
                (Point::new(Mm(p2_x), Mm(p2_y)), false),
                (Point::new(Mm(p3_x), Mm(p3_y)), false),
                (Point::new(Mm(p4_x), Mm(p4_y)), false),
            ];

            let line = Line {
                points,
                is_closed: true,
                has_fill: false,
                has_stroke: true,
                is_clipping_path: false,
            };

            current_layer.add_shape(line);
            if r.1.iteration.is_some() {
                current_layer.use_text(format!("{} {:?}-{}", get_language_value(language, ROUND), r.0 + 1, r.1.iteration.clone().unwrap()), 18.0, Mm(p1_x + 2.0), Mm(p1_y - 6.0), &helvetica_bold);
            } else {
                current_layer.use_text(format!("{} {:?}", get_language_value(language, ROUND), r.0 + 1), 18.0, Mm(p1_x + 5.0), Mm(p1_y - 6.0), &helvetica_bold);
            }
            if r.1.is_bye == false {
                current_layer.use_text(format!("{} {}", get_language_value(language, TABLE), r.1.table), 18.0, Mm(p1_x + 37.0), Mm(p1_y - 6.0), &helvetica_bold);
            }
            let line2 = Line {
                points: vec![
                    (Point::new(Mm(p1_x), Mm(p1_y - 8.0)), false),
                    (Point::new(Mm(p2_x), Mm(p2_y - 8.0)), false),
                ],
                is_closed: true,
                has_fill: false,
                has_stroke: true,
                is_clipping_path: false,
            };
            current_layer.add_shape(line2);

            let dummy = get_dummy_double_player();

            let me = players.get(&r.1.me).unwrap_or(&dummy);
            let mate = players.get(&r.1.mate).unwrap_or(&dummy);
            let o_a = players.get(&r.1.opponents_a).unwrap_or(&dummy);
            let o_b = players.get(&r.1.opponents_b).unwrap_or(&dummy);

            let pos_me = (p1_x + 2.0, p1_y - 14.0);
            let pos_mate = (p1_x + 2.0, p1_y - 19.0);
            let pos_o_a = (p1_x + width_one_round_table / 2.0 + 6.0, p1_y - 14.0);
            let pos_o_b = (p1_x + width_one_round_table / 2.0 + 6.0, p1_y - 19.0);
            let pos_vs = (p1_x + width_one_round_table / 2.0 - 3.0, p1_y - 17.0);

            if r.1.is_bye {
                current_layer.use_text(get_language_value(language, BYE), 18.0, Mm(p1_x + 5.0), Mm(pos_vs.1), &helvetica_bold);
            } else {
                current_layer.use_text(format!("{}", me.card.name), 10.0, Mm(pos_me.0), Mm(pos_me.1), &helvetica_regular);
                current_layer.use_text(format!("{}", mate.card.name), 10.0, Mm(pos_mate.0), Mm(pos_mate.1), &helvetica_regular);
                current_layer.use_text(format!("{}", o_a.card.name), 10.0, Mm(pos_o_a.0), Mm(pos_o_a.1), &helvetica_regular);
                current_layer.use_text(format!("{}", o_b.card.name), 10.0, Mm(pos_o_b.0), Mm(pos_o_b.1), &helvetica_regular);
                current_layer.use_text("vs.", 12.0, Mm(pos_vs.0), Mm(pos_vs.1), &helvetica_bold);
            }


            let line3 = Line {
                points: vec![
                    (Point::new(Mm(p1_x), Mm(p1_y - 23.0)), false),
                    (Point::new(Mm(p2_x), Mm(p2_y - 23.0)), false),
                ],
                is_closed: true,
                has_fill: false,
                has_stroke: true,
                is_clipping_path: false,
            };
            current_layer.add_shape(line3);

            current_layer.use_text(":", 18.0, Mm(p1_x + width_one_round_table / 2.0 - 1.5), Mm(p1_y - 30.5), &helvetica_bold);
        }
    });


    //let (page2, _p2_layer1) = doc.add_page(Mm(297.0), Mm(210.0),"Page 2, Layer 1");
    //let _ = doc.get_page(page2).add_layer("Layer 3");

    doc.save_to_bytes().unwrap()
}


fn get_dummy_double_player() -> PlayerContainerDouble {
    PlayerContainerDouble {
        id: 0,
        card: Card { name: "-".to_string(), image: "".to_string() },
        rounds: vec![],
    }
}