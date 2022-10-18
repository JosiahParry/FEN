// For user input
use std::io;
// for formatting
use std::fmt;
fn main() {

    // This code chunk takes user input
    let mut fen = String::new();
    println!("Enter FEN Code:");
    io::stdin()
        .read_line(&mut fen)
        .expect("Failed to read line");
    // have to remove the trailing new line
    let fen_parts: Vec<&str> = fen.strip_suffix("\n").unwrap().split(" ").collect();

    // use a fixed FEN code
    //let fen_parts: Vec<&str> = "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b kq d3 0 1".split(" ").collect();
    //let fen_parts: Vec<&str> = "r1bqkb1r/pp3ppp/5n2/3pP3/5B2/2N5/PP2PPPP/R2QKB1R b KQkq - 0 8".split(" ").collect();
    // extract current position
    let cur_position = fen_parts[0];

    let res = parse_position(cur_position);

    let mut fen = Fen {
        position: res,
        active: fen_parts[1].to_string(),
        castling: fen_parts[2].to_string(),
        en_passant: fen_parts[3].to_string(),
        halfmoves: fen_parts[4].to_string().parse().unwrap(),
        fullmoves: fen_parts[5].to_string().parse().unwrap(),
    };

    //println!("{:?}", fen);
    let castling = fen_parts[2].to_string();
    //let castling_rights = parse_castling(&castling);
    //println!("{}", castling_rights);

    //fen.position[0].iter().map(|x| translate_piece(x));
    let pretty_pieces = fen.position.iter().map(|x| translate_pieces(x));
    print!("{}", fen);
    
}


// FEN Struct
#[derive(Debug)]
struct Fen {
    position: Vec<Vec<String>>,
    active: String,
    castling: String,
    en_passant: String,
    halfmoves: u32,
    fullmoves: u32,
}

// Implement printing for Fen structure
impl fmt::Display for Fen {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // handle active color 
        let active = parse_active_color(&self.active);
        write!(f, "To play: {active}\n");
        // en passant
        write!(f, "En passant: {}\n", self.en_passant);
        // moves
        write!(f, "Half moves: {}\n", self.halfmoves);
        write!(f, "Full moves: {}\n", self.fullmoves);
        // handle castling rights
        let castle = parse_castling(&self.castling);
        write!(f, "{castle}\n");        
        // handle active position printing
        let positions = &self.position;
        for item in positions.iter() {
            //write!(f, "{:?}\n", item);
            write!(f, "{:?}\n", translate_pieces(item));
        }
        Ok(())
    }
}

// Castling Struct tuple
// (White-King, White-Queen, Black-King, White-Queen)
// inherit debug
#[derive(Debug)]
struct CastleRights(bool, bool, bool, bool);

// Determine castling Rights
// CastleRights Struct keeps a boolean of castling rights and this 
// returns a tuple of String(s) that has the pretty printing method for it
fn determine_castling(castling: &CastleRights) -> (String, String) {
    let white = (castling.0, castling.1);
    let black = (castling.2, castling.3);
    let white_res = if white.0 & white.1 {
         "Kingside and Queenside"
        } else if white.0 & !white.1 {
            "Kingside"
        } else if !white.0 & white.1 {
            "Queenside"
        } else {
            "none"
        }.to_string();

    let black_res = if black.0 & black.1 {
        "Kingside and Queenside"
        } else if black.0 & !black.1 {
            "Kingside"
        } else if !black.0 & black.1 {
            "Queenside"
        } else {
            "none"
        }.to_string();

    let res = (white_res, black_res);

    res

}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for CastleRights {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // use determine_castling to create tuple to print nicely
        let rights = determine_castling(&self);
        write!(f, "Castling Rights:");
        write!(f, "\n- White: {}", rights.0);
        // note that the ; must be ommited to return a fmt::Result
        write!(f, "\n- Black: {}", rights.1)
    }
}


// FEN Parsing Functions -----------------------------------------------------------------
//----------------------------------------------------------------------------------------
fn parse_position(position: &str) -> Vec<Vec<String>> {
    // take current position fen string and split on /
    let rank_pieces: Vec<&str> = position.split("/").collect();
    // instantiate the vector to be filled
    let mut all_ranks: Vec<Vec<String>> = Vec::with_capacity(8);
    // for each rank parse
    for rnk in rank_pieces.iter() {
        let rnk_i = parse_rank(rnk);
        all_ranks.push(rnk_i);
    }
    all_ranks
}

fn parse_active_color(color: &str) -> String {
    let color = match color {
        "b" => String::from("Black"),
        "w" => String::from("White"),
        _ => String::from("error") // wtf is happening here
    };
    color
}


// Parses one row of FEN to a vector
// Take example of 3P4 to turn into ---P---- but its 
// a vector of [" ", " ", " ", "P", " ", " ", " ", " "]
fn parse_rank(rank_chr: &str) -> Vec<String> {
    // create empty vector
    let mut rank: Vec<String> = Vec::with_capacity(8);

    for chr in rank_chr.chars() {
        // determine if the character is numeric 
        let chr_num = chr.is_numeric();
        // if it is repeat the character and push back into rank
        if chr_num {
            // identify the number of blank squares
            let n_reps: usize = chr.to_string().parse().unwrap();
            // push n_reps number of blank spaces into rank
            for _ in 0..n_reps {
                rank.push(" ".to_string());
            }
        } else {
        // create str from the char
        let piece = chr.to_string();
        // it doesn't live long enough apparently and im losing my f***ing mind
        rank.push(piece);

        }
    }
    rank
}


// Parses castling rights
fn parse_castling(castling: &String) -> CastleRights {
    // parse castling rights
    let castling_rights
     = CastleRights(
        castling.contains("K"),
        castling.contains("Q"),
        castling.contains("k"),
        castling.contains("q"),
    );

    castling_rights
}

// from https://github.com/jonocarroll/FEN.rs/blob/main/src/main.rs

fn translate_pieces(x: &Vec<String>) -> Vec<String> {
    x.iter().map(|x| translate_piece(x)).collect()
}

fn translate_piece(x: &String) -> String {
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


//let burl = "https://explorer.lichess.ovh/masters?fen=";
//let query = format!("{burl}{fen}");
//let resp = reqwest::get(&query);
// https://explorer.lichess.ovh/masters?fen=rnbqkbnr/pp2pppp/2p5/3p4/2PP4/8/PP2PPPP/RNBQKBNR%20w%20KQkq%20-%200%203

// Query lichess database from fen
