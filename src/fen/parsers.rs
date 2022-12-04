// FEN Parsing Functions -----------------------------------------------------------------
//----------------------------------------------------------------------------------------
pub fn parse_position(position: &str) -> Vec<Vec<String>> {
    // take current position fen string and split on /
    let rank_pieces: Vec<&str> = position.split("/").collect();
    // instantiate the vector to be filled
    let mut all_ranks: Vec<Vec<String>> = Vec::with_capacity(8);
    // for each rank parse

    
    for rnk in rank_pieces.iter().rev() {
        let rnk_i = parse_rank(rnk);
        all_ranks.push(rnk_i);
    }
    all_ranks
}

pub fn parse_active_color(color: &str) -> String {
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
pub fn parse_rank(rank_chr: &str) -> Vec<String> {
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
pub fn parse_castling(castling: &String) -> crate::fen::structs::CastleRights {
    // parse castling rights
    let castling_rights
     = crate::fen::structs::CastleRights(
        castling.contains("K"),
        castling.contains("Q"),
        castling.contains("k"),
        castling.contains("q"),
    );

    castling_rights
}



// PARSE ENTIRE FEN STRING
pub fn parse_fen(fen: String) -> crate::fen::structs::Fen {
    let fen_parts: Vec<&str> = fen.
       // strip_suffix("\n").
       // unwrap().
        split(" ").
        collect();
        
    println!("{:?}", fen_parts);
    let cur_position = fen_parts[0];
    let res = crate::fen::parsers::parse_position(cur_position);
    let fen = crate::fen::structs::Fen {
     position: res,
     active: fen_parts[1].to_string(),
     castling: fen_parts[2].to_string(),
     en_passant: fen_parts[3].to_string(),
     halfmoves: fen_parts[4].to_string().parse().unwrap(),
     fullmoves: fen_parts[5].to_string().parse().unwrap(),
    };

    fen
}
