fn main() {

    let fen = "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1";
    let fen_parts: Vec<&str> = "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1".split(" ").collect();

    // extract current position
    let cur_position = fen_parts[0];

    // split on "/" into vector 
    let mut rank_pieces: Vec<&str> = cur_position.split("/").collect();

    // Take example of 3P4 to turn into ---P---- but its 
    // a vector of [" ", " ", " ", "P", " ", " ", " ", " "]
    let x = rank_pieces[4];

    let mut rank: Vec<&str> = Vec::with_capacity(8);;

    for chr in x.chars() {
        // determine if the character is numeric 
        let chr_num = chr.is_numeric();
        // if it is repeat the character and push back into rank
        if chr_num {
            // identify the number of blank squares
            let n_reps: usize = chr.to_string().parse().unwrap();
            // push n_reps number of blank spaces into rank
            for _ in 0..n_reps {
                rank.push(" ");
            }
            // create white space
//            let whitespace = " ".repeat(n_reps);
            // split string by white space turn into vector
            //let mut squares: Vec<&str> = whitespace.split(" ").collect();
        }
        // create str from the char
        chr.encode_utf8(&mut piece)
        

        rank.push(&piece);
    
    }

   // println!("{:?}", rank);

}


// uppercase is white
    // lowercase is black
    // starts at rank 8 -> rank 1
    //for part in rank_pieces {
    //    println!("{:?}", part)
    //}

    //println!("{cur_position}");

    //let burl = "https://explorer.lichess.ovh/masters?fen=";
    //let query = format!("{burl}{fen}");
    //let resp = reqwest::get(&query);


// Query lichess database from fen
