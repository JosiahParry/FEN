// Prettify Chess pieces -------------------------------------------------
// from https://github.com/jonocarroll/FEN.rs/blob/main/src/main.rs

pub fn translate_pieces(x: &Vec<String>) -> Vec<String> {
    x.iter().map(|x| translate_piece(x)).collect()
}

pub fn translate_piece(x: &String) -> String {
    let newsym: String = match x.as_str() {
        "P" => String::from("♙"),
        "N" => String::from("♘"),
        "B" => String::from("♗"),
        "R" => String::from("♖"),
        "Q" => String::from("♕"),
        "K" => String::from("♔"),
        "p" => String::from("♟"),
        "n" => String::from("♞"),
        "b" => String::from("♝"),
        "r" => String::from("♜"),
        "q" => String::from("♛"),
        "k" => String::from("♚"),
        _ => String::from(" ")
    };
    newsym.to_string()
}