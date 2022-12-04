
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



// reads a pgn in and turns into a struct
// let file_path = "src/data/sample.pgn";
// let contents = fs::read_to_string(file_path)
//     .expect("cannot read pgn file");
// let pgn = crate::pgn::parsers::parse_pgn(contents);
// println!("{:?}", pgn);

//let burl = "https://explorer.lichess.ovh/masters?fen=";
//let query = format!("{burl}{fen}");
//let resp = reqwest::get(&query);
// https://explorer.lichess.ovh/masters?fen=rnbqkbnr/pp2pppp/2p5/3p4/2PP4/8/PP2PPPP/RNBQKBNR%20w%20KQkq%20-%200%203

// Query lichess database from fen
