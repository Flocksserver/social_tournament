use std::collections::HashMap;
use crate::pdf::language::LanguageKey::*;


#[derive(Clone, Copy, PartialEq)]
pub enum Language {
    DE,
    EN,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub enum LanguageKey {
    ROUND,
    TABLE,
    RESULT,
    BYE,
    TITLE_SOCIAL_DOUBLE,
    INTRO_SOCIAL_DOUBLE,
    SUBTITLE_SOCIAL_SUM_DOUBLE,
    IMPL_PROPOSAL,
    IMPL_PROPOSAL_SOCIAL_DOUBLE,
    BENEFITS_TITLE_SOCIAL_DOUBLE,
    BENEFITS_CONTENT_SOCIAL_DOUBLE,
    FUN_CONTENT_SOCIAL_DOUBLE,
    SPECIAL_BEFORE_ROUND_TITLE_SOCIAL_DOUBLE,
    SPECIAL_BEFORE_ROUND_CONTENT_SOCIAL_DOUBLE,
    SPECIAL_AFTER_ROUND_TITLE_SOCIAL_DOUBLE,
    SPECIAL_AFTER_ROUND_CONTENT_SOCIAL_DOUBLE,
    ALTERNATIVE_ONE_SOCIAL_DOUBLE,
    ALTERNATIVE,
    ALTERNATIVE_TWO_SOCIAL_DOUBLE,
    ALTERNATIVE_THREE_SOCIAL_DOUBLE,
    HINT_TITLE_SUM_SOCIAL_DOUBLE,
    HINT_CONTENT_SUM_SOCIAL_DOUBLE,
    HINT_CONTENT_SUM_EXAMPLE_ONE_SOCIAL_DOUBLE,
    HINT_CONTENT_SUM_EXAMPLE_TWO_SOCIAL_DOUBLE,
    HINT_TITLE_PRINT_SOCIAL_DOUBLE,
    HINT_CONTENT_PRINT_SOCIAL_DOUBLE,
    GREETING,
}

lazy_static! {
    static ref DE_MAP: HashMap<LanguageKey, &'static str> = {
        let mut m = HashMap::new();
        m.insert(ROUND, "Runde");
        m.insert(TABLE, "Tisch");
        m.insert(RESULT, "Ergebnis");
        m.insert(BYE, "Freilos");
        m.insert(TITLE_SOCIAL_DOUBLE, "Das soziale Doppelturnier");
        m.insert(INTRO_SOCIAL_DOUBLE, "In diesem PDF-Satz findest du die Laufzettel, um ein soziales Doppelturnier zu spielen. Doch wieso \"sozial\"?\nDer Algorithmus hat versucht, möglichst viele unterschiedliche Spielerinnen und Spieler jeder Runde neu an\ndie Tische zu verteilen. Dabei ist sichergestellt, dass dir, solange möglich, immer ein anderer Doppelpartner\noder eine andere Doppelpartnerin zugelost wird.");
        m.insert(SUBTITLE_SOCIAL_SUM_DOUBLE, "Das soziale Doppel-Summen-Turnier");
        m.insert(IMPL_PROPOSAL, "Umsetzungsvorschlag: ");
        m.insert(IMPL_PROPOSAL_SOCIAL_DOUBLE, "In der Summen-Variante des Turniers geht es darum, am Ende die meisten Punkte\nzu haben. Jede Runde wird in einer von dir definierten Zeit gespielt. Beispiel 5 Minuten. Da es im Doppel\nwichtig ist, dass (wie im Satzwechsel) auch der andere Gegner deinen Aufschlag annehmen muss, wäre bei\n5 Minuten Spielzeit nach 2:30 Minuten \"Annahmenwechsel\". Die Punkte werden weitergezählt, sodass am\nEnde der 5 Minuten ein Ergebnis (Beispiel: 18:12) feststeht. Dieses wird in dem Laufzettel eingetragen. Den\nAnnahmenwechsel und das Rundenende werden durch die Turnierleitung angesagt. Ich würde die Regel\nempfehlen, den laufenden Ballwechsel bei \"Rundenende\" oder \"Annahmenwechsel\" noch ausspielen zu lassen.");
        m.insert(BENEFITS_TITLE_SOCIAL_DOUBLE, "Vorteile der Summen-Variante sind unter anderem:");
        m.insert(BENEFITS_CONTENT_SOCIAL_DOUBLE, "Jede Runde (und damit das Turnier) ist zeitlich sehr gut abzuschätzen\nAlle spielen zur gleichen Zeit und keiner muss warten\nDu kannst auch nach einer beliebigen Anzahl an Runden abbrechen\nJeder gemachte Punkt zählt\nErst am Ende nach der Auswertung wird klar, wer gewonnen hat");
        m.insert(FUN_CONTENT_SOCIAL_DOUBLE, "Diese Turnierform ist als reines Spaßturnier gedacht und sollte auch so durchgeführt werden. Lass doch neben-\nbei Musik laufen. Außerdem können Spezialrunden noch zusätzliche Abwechslung in das Turnier bringen.");
        m.insert(SPECIAL_BEFORE_ROUND_TITLE_SOCIAL_DOUBLE, "Beispiele zu Ansagen vor einer Runde:");
        m.insert(SPECIAL_BEFORE_ROUND_CONTENT_SOCIAL_DOUBLE, "Alle spielen mit ihrer Nicht-Schlaghand\nDas Doppel teilt sich einen Schläger\nAufschlagpunkte zählen doppelt\nEs darf nur über die VH oder RH-Diagonale gespielt werden");
        m.insert(SPECIAL_AFTER_ROUND_TITLE_SOCIAL_DOUBLE, "Beispiele zu Ansagen nach einer Runde:");
        m.insert(SPECIAL_AFTER_ROUND_CONTENT_SOCIAL_DOUBLE, "Das Doppel mit weniger Punkten darf seine Punkte verdoppeln\nDie Punkte der Doppel werden getauscht\nAlle mit Schuhgröße 37 und kleiner bekommen 10 extra Punkte");
        m.insert(ALTERNATIVE_ONE_SOCIAL_DOUBLE, "Du kannst im sozialen Doppel-Summen-Turnier statt einer festen Zeit eine exakt vorgegebene Anzahl\nan Punkten ausspielen lassen. Beispiel: Jede Runde werden genau 40 Punkte ausgespielt. Dementsprechend\nist nach 20 ausgespielten Punkten \"Annahmenwechsel\".");
        m.insert(ALTERNATIVE, "Alternative: ");
        m.insert(ALTERNATIVE_TWO_SOCIAL_DOUBLE, "Das klassische Doppel-Kreuzchen-Turnier - Hierbei lässt du eine beliebige Anzahl an Sätzen\nspielen. Bei einem Sieg darf die Doppelpaarung ein Kreuz und bei Niederlage einen Kreis eintragen.");
        m.insert(ALTERNATIVE_THREE_SOCIAL_DOUBLE, "Das klassische Doppel-Turnier - Hierbei lässt du eine beliebige Anzahl an Sätzen spielen\nund das Ergebnis im Laufzettel vermerken.");
        m.insert(HINT_TITLE_SUM_SOCIAL_DOUBLE, "Mein Tipp zum Auswerten: ");
        m.insert(HINT_CONTENT_SUM_SOCIAL_DOUBLE, "Vor allem, wenn viele Teilnehmenden das Turnier spielen, ist bei Laufzetteln eine\nAuswertung am Ende zeitaufwendig. Daher ist mein Tipp, die Gruppe mit einzubinden. Lass alle Teilnehmenden\nam Tisch des letzten Spiels bleiben. Dort können sie selbstständig oder mit gegenseitiger Hilfe die Laufzettel aus-\nwerten. Die Gesamtpunktzahl bzw. Gesamtsiege (je nach Variante) kann auf dem Laufzettel vermerkt werden.\nDie Siegerehrung könnte dann beispielsweise wie folgt ablaufen, bis die Siegertreppchen ermittelt wurden.");
        m.insert(HINT_CONTENT_SUM_EXAMPLE_ONE_SOCIAL_DOUBLE, "\"Wer hat mehr als 50 Punkte\" —>  Alle melden sich");
        m.insert(HINT_CONTENT_SUM_EXAMPLE_TWO_SOCIAL_DOUBLE, "\"Wer hat mehr als 100 Punkte\" —>  Weniger melden sich");
        m.insert(HINT_TITLE_PRINT_SOCIAL_DOUBLE, "Mein Tipp zum Drucken: ");
        m.insert(HINT_CONTENT_PRINT_SOCIAL_DOUBLE, "Drucke die Zettel auf normalem A4 Papier mit dem Layout \"2 auf eine Seite\". So\nhaben die Zettel eine handliche Größe haben für alle Teilnehmenden. Solltest du mehr als 12 Runden generiert\nhaben, so kannst du das resultierende A4 Blatt knicken - oder A5 mit Vorder- und Rückseite bedrucken.");
        m.insert(GREETING, "Viel Spaß beim Spielen und Ausprobieren!");
        m
    };
}

lazy_static! {
    static ref EN_MAP: HashMap<LanguageKey, &'static str> = {
        let mut m = HashMap::new();
        m.insert(ROUND, "Round");
        m.insert(TABLE, "Table");
        m.insert(RESULT, "Result");
        m.insert(BYE, "Bye");
        m.insert(TITLE_SOCIAL_DOUBLE, "The social doubles tournament");
        m.insert(INTRO_SOCIAL_DOUBLE, "In this PDF set you will find the routing cards to play a social doubles tournament. But why \"social\"?\nThe algorithm has tried to reassign as many different players as possible to the tables each round. This\nensures that, as long as possible, you will always be assigned a different doubles partner.");
        m.insert(SUBTITLE_SOCIAL_SUM_DOUBLE, "The social doubles sum-up tournament");
        m.insert(IMPL_PROPOSAL, "Implementation proposal: ");
        m.insert(IMPL_PROPOSAL_SOCIAL_DOUBLE, "In the summation variant of the tournament, the goal is to have the most points\nat the end. Each round is played in a time defined by you. Example 5 minutes. Since it is important in\ndoubles that (as in set change) the other opponent must also accept your serve, in 5 minutes of play there\nwould be a \"change of service\" after 2:30 minutes. The points will continue to be counted so that at the end\nof the 5 minutes there is a result (example: 18:12). This is entered in the running card. The change of service\nand the end of the round are announced by the tournament management. I would recommend the rule to let\nthe current rally still play out at \"end of round\" or \"change of service\".");
        m.insert(BENEFITS_TITLE_SOCIAL_DOUBLE, "Advantages of the sum variant include:");
        m.insert(BENEFITS_CONTENT_SOCIAL_DOUBLE, "Each round (and thus the tournament) is very well timed\nEverybody plays at the same time and nobody has to wait\nYou can also stop after any number of rounds\nEvery made point counts\nOnly at the end after the evaluation it is clear who has won");
        m.insert(FUN_CONTENT_SOCIAL_DOUBLE, "This type of tournament is intended as a pure fun tournament and should be conducted as such. You\ncan play music on the side. In addition, special rounds can bring additional variety into the tournament.");
        m.insert(SPECIAL_BEFORE_ROUND_TITLE_SOCIAL_DOUBLE, "Examples of announcements before a round:");
        m.insert(SPECIAL_BEFORE_ROUND_CONTENT_SOCIAL_DOUBLE, "All players play with their non-stroke hand\nThe playing partners share one racket\nService points count double\nIt is allowed to play only over the VH or RH diagonal");
        m.insert(SPECIAL_AFTER_ROUND_TITLE_SOCIAL_DOUBLE, "Examples of announcements after a round:");
        m.insert(SPECIAL_AFTER_ROUND_CONTENT_SOCIAL_DOUBLE, "The double with less points may double their points\nThe points of the doubles are exchanged\nAll with shoe size 37 and smaller get 10 extra points");
        m.insert(ALTERNATIVE_ONE_SOCIAL_DOUBLE, "You can have an exact number of points played out in the social double-sum tournament instead\nof a fixed time. Example: Exactly 40 points are played out each round. Accordingly, after 20 points have been\nplayed out, it is \"change of service\".");
        m.insert(ALTERNATIVE, "Alternative: ");
        m.insert(ALTERNATIVE_TWO_SOCIAL_DOUBLE, "The classic doubles-cross tournament - Here you let play any number of sets. In case of a win\nthe doubles pair may enter a cross and in case of a loss a circle.");
        m.insert(ALTERNATIVE_THREE_SOCIAL_DOUBLE, "The classic doubles tournament - Here you let play any number of sets and record the result in\nthe running card.");
        m.insert(HINT_TITLE_SUM_SOCIAL_DOUBLE, "My tip for evaluating: ");
        m.insert(HINT_CONTENT_SUM_SOCIAL_DOUBLE, "Especially if many participants play the tournament, an evaluation at the end\nis time-consuming with running cards. Therefore, my tip is to involve the group. Let all participants stay at\nthe table of the last game. There they can evaluate the running card independently or with each other's help.\nThe total number of points or total victories (depending on the variant) can be noted on the running card. The\naward ceremony could then proceed as follows. For example, until the winners' podiums have been determined\ndo the following.");
        m.insert(HINT_CONTENT_SUM_EXAMPLE_ONE_SOCIAL_DOUBLE, "\"Who has more than 50 points\" —> All give hand signal");
        m.insert(HINT_CONTENT_SUM_EXAMPLE_TWO_SOCIAL_DOUBLE, "\"Who has more than 100 points\" —>  Less give hand signal");
        m.insert(HINT_TITLE_PRINT_SOCIAL_DOUBLE, "My tip for printing: ");
        m.insert(HINT_CONTENT_PRINT_SOCIAL_DOUBLE, "Print the sheets on normal A4 paper with the layout \"2 on a page\". This way the sheets\nwill have a handy size for all participants. If you have generated more than 12 rounds, you can fold the\nresulting A4 sheet - or print A5 with front and back.");
        m.insert(GREETING, "Have fun playing and trying it out!");
        m
    };
}

pub fn get_language_value(lang: Language, key: LanguageKey) -> String {
    match lang {
        Language::DE => {
            DE_MAP.get(&key).unwrap().to_string()
        }
        Language::EN => {
            EN_MAP.get(&key).unwrap().to_string()
        }
    }
}