// use fen module
// pub mod fen;

// For user input
// use std::io;

use std::fs;

// import hashmaps
use std::collections::HashMap;

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
            moves.push_str(&line)
        }

    };

    println!("{moves}");

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
    // // let move = "Ne4";    












//let burl = "https://explorer.lichess.ovh/masters?fen=";
//let query = format!("{burl}{fen}");
//let resp = reqwest::get(&query);
// https://explorer.lichess.ovh/masters?fen=rnbqkbnr/pp2pppp/2p5/3p4/2PP4/8/PP2PPPP/RNBQKBNR%20w%20KQkq%20-%200%203

// Query lichess database from fen
