// use fen module
// pub mod fen;
// For user input
// use std::io;

// for reading file
use std::fs;

// import hashmaps
use std::collections::HashMap;

use regex::Regex; 

fn main() {

    let file_path = "src/data/sample.pgn";


    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // create hash map for headers
    let mut header = HashMap::new();

    // instantiate empty string 
    let mut moves: String = String::new(); 

    for line in contents.split("\n") {

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
    let game_termination =  termination_regex.find(&moves).unwrap().as_str();

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
    let mut move_vec: Vec<String> = Vec::new();


    for split in moves {
        let move_str = split.trim().to_string();
        if move_str.is_empty() { continue; }
        let x = move_str.split(" ").collect::<Vec<_>>();
        println!("{:?}", x);
    
        move_vec.push(move_str); 
    }

    println!("{:?}", move_vec);
    

}






    // // This code chunk takes user input 
    // let mut fen_string = String::new();
    // println!("Enter FEN Code:");
    // io::stdin()
    //     .read_line(&mut fen_string)
    //     .expect("Failed to read line");
    
    // // parse the fen
    // let fen = crate::fen::parsers::parse_fen(fen_string);
    // // print it
    // print!("{}", fen);


// validate that hash map has the appropriate keys
// let required_keys = ["Event", "Site", "Date", "Round", "White", "Black", "Result"];
// for key in required_keys {
//     let has_required_key = header.contains_key(key);
//     println!("{key}");
// }


//let burl = "https://explorer.lichess.ovh/masters?fen=";
//let query = format!("{burl}{fen}");
//let resp = reqwest::get(&query);
// https://explorer.lichess.ovh/masters?fen=rnbqkbnr/pp2pppp/2p5/3p4/2PP4/8/PP2PPPP/RNBQKBNR%20w%20KQkq%20-%200%203

// Query lichess database from fen
