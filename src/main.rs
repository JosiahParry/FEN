// use fen module
pub mod fen;


fn main() {
    let fen_string = String::from("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");

    // parse the fen
    let fen = crate::fen::parsers::parse_fen(fen_string);
    // print it
    print!("{}", fen);

    // define move
    let mv = "d5";

    // the vector of moves starts from rank 8 
    // which is somewhat confusing
    println!("{:?}", fen.position[7]);

}
