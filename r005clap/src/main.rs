//handle input using CLAP
//toUpperCase of the input

use clap::Parser; //needs a structure to read and use params
use r005clap::capitalize;

//keywords used for Parser
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args{
    #[clap(short, long)]
    stringa: String,
}

fn main() {
    println!("Hello, world!");

    //test and use of struct. Run from CMD using cargo run -- --stringa
    //using parse function on Args struct to fill it with the input params
    let args = Args::parse();

    println!("Input -> {}",args.stringa);

    let stringa_capitalized = capitalize(&args.stringa);
    println!("Input, capitalized -> {}", stringa_capitalized);
}

fn capitalize(s:&str) -> String{
    let mut stringa = String::new();

    for c in s.chars(){
        //        stringa.push(c.to_uppercase());  ERROR
        stringa.push(c.to_ascii_uppercase());
    }
    //prints cchar [2] of c if is not null. nth returns the nth element of the argument
    //println!("c -> {}", s.chars().nth(2).unwrap());

    stringa
}

