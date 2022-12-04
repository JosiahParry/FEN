// for formatting
use std::fmt;

// FEN Struct
#[derive(Debug)]
pub struct Fen {
    pub position: Vec<Vec<String>>,
    pub active: String,
    pub castling: String,
    pub en_passant: String,
    pub halfmoves: u32,
    pub fullmoves: u32,
}

// Implement printing for Fen structure
impl fmt::Display for Fen {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // handle active color 
        let active = crate::fen::parsers::parse_active_color(&self.active);
        write!(f, "To play: {active}\n")?;
        // en passant
        write!(f, "En passant: {}\n", self.en_passant)?;
        // moves
        write!(f, "Half moves: {}\n", self.halfmoves)?;
        write!(f, "Full moves: {}\n", self.fullmoves)?;
        // handle castling rights
        let castle = crate::fen::parsers::parse_castling(&self.castling);
        write!(f, "{castle}\n")?;        
        // handle active position printing
        let positions = &self.position;
        for item in positions.iter().rev() {
            //write!(f, "{:?}\n", item);
            write!(f, "{:?}\n", crate::fen::prettify::translate_pieces(item))?;
        }
        Ok(())
    }
}

// Castling Struct tuple
// (White-King, White-Queen, Black-King, White-Queen)
// inherit debug
#[derive(Debug)]
pub struct CastleRights(pub bool, pub bool, pub bool, pub bool);

// Determine castling Rights
// CastleRights Struct keeps a boolean of castling rights and this 
// returns a tuple of String(s) that has the pretty printing method for it
pub fn determine_castling(castling: &CastleRights) -> (String, String) {
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
        write!(f, "Castling Rights:")?;
        write!(f, "\n- White: {}", rights.0)?;
        // note that the ; must be ommited to return a fmt::Result
        write!(f, "\n- Black: {}", rights.1)
    }
}
