// use fen module
pub mod fen;

// For user input
use std::io;

fn main() {

    // This code chunk takes user input 
    let mut fen_string = String::new();
    println!("Enter FEN Code:");
    io::stdin()
        .read_line(&mut fen_string)
        .expect("Failed to read line");
    
    // parse the fen
    let fen = crate::fen::parsers::parse_fen(fen_string);
    // print it
    print!("{}", fen);
    // let move = "Ne4";    

}




//let burl = "https://explorer.lichess.ovh/masters?fen=";
//let query = format!("{burl}{fen}");
//let resp = reqwest::get(&query);
// https://explorer.lichess.ovh/masters?fen=rnbqkbnr/pp2pppp/2p5/3p4/2PP4/8/PP2PPPP/RNBQKBNR%20w%20KQkq%20-%200%203

// Query lichess database from fen
