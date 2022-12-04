// This file contains parsers for PGN codes
use regex::Regex;

// for the header
// the header can be populated easily while reading lines
use std::collections::HashMap;


pub fn parse_moves(moves: String) -> Vec<Vec<String>> {
    // TODO: handle game termination markers
    // TODO: handle comments
    // TODO: handle RAV

    // For the sake of simplicity, don't include RAV and comments
    // to parse the moves, just remove the Recursive Annotation Variation (RAV)
    // and comments. at a later point we will have to get them in the right place
    // extract RAV and Comments
    // NOTE: need to remove annotations too
    // comments, annotations, and RAV, can be stored in another vector
    // thinking alot the lines of a dataframe here tbh
    let rav_regex = Regex::new("\\((?:[^{}])*\\)").unwrap();
    let comment_regex = Regex::new("\\{(?:[^{}])*\\}").unwrap();
    let annotation_regex = Regex::new("\\$[0-9]{1,3}").unwrap();
    // extract game termination marker
    let termination_regex = Regex::new("0-1|1-0|1/2-1/2|\\*").unwrap();
    //let game_termination =  termination_regex.find(&moves).unwrap().as_str();

    //let comment = comment_regex.find(&moves).unwrap().as_str(); 
    //let rav = rav_regex.find(&moves).unwrap().as_str();


    // remove RAV, comments, annotations, termination, from moves
    let moves = comment_regex.
        replace_all(&moves, "").
        to_string();
    
    let moves: String = annotation_regex.
        replace_all(&moves, "").
        to_string();

    let moves: String = termination_regex.
        replace_all(&moves, "").
        to_string();
   
    let moves = rav_regex.
        replace_all(&moves, "");

    // regex to split by move number
    let moves_regex = Regex::new("[0-9]{1,}\\.{1}").unwrap();

    let moves = moves_regex.split(&moves);

    // instantiate a vector for moves
    // each element is a vector of length two
    // first element is the white move the second is the black move
    let mut move_vec: Vec<Vec<String>> = Vec::new();
    
    for split in moves {
        let move_str = split.trim().to_string();
        if move_str.is_empty() { continue; }
        let x = move_str.split(" ").collect::<Vec<&str>>();
        let y: Vec<String> = x.iter().map(|s| s.to_string()).collect();
        //println!("{:?}", y);
        //println!("{:?}", x);
    
        move_vec.push(y); 
    }

    move_vec
}



// parse raw fen string into a struct PGN 
pub fn parse_pgn(pgn: String) -> PGN {
        // create hash map for headers
        let mut header = HashMap::new();

        // instantiate empty string 
        let mut moves: String = String::new(); 
    
        for line in pgn.split("\n") {
    
            // let is_header: bool = true;
            // println!("{:?}", line);
            let is_header = line.starts_with("[");
            if  is_header {
                // split the string to get key values
                let split: Vec<&str> = line.split(" ").collect(); 
    
                // get the tag names
                let key = &split[0][1..];
    
                // get the value pair
                let val = split[1..split.len()].join(" ");
                let val = &val[1..val.len()-2];
    
                header.insert(
                    key.to_string(),
                    val.to_string(),
                );
    
                //println!("{:?}", header);
                //let val = &split[1][1..value_len];
                //println!("{key}: {val}");
    
            } else {
    
                // add space to each line to ensure no annotations are
                // prepended to a move-number
                let line = line.to_owned() + " ";
                moves.push_str(&line)
            }
    
        };
    
    
        // use pgn parser which takes a full string
        let all_moves = crate::pgn::parsers::parse_moves(moves);
    
    
        //println!("{:?}", all_moves);
    
        let pgn_res = crate::pgn::parsers::PGN {
            header: header,
            moves: all_moves
        };

    pgn_res
}

// define struct to store header and moves 
// moves are a raw vector until i figure out how to get
// them into fen format
#[derive(Debug)]
pub struct PGN {
    pub header: HashMap<String, String>,
    pub moves: Vec<Vec<String>>
}