extern crate pest;

#[macro_use]
extern crate pest_derive;

use pest::{Parser, iterators::{Pairs, Pair}};
use std::{fs,env, vec, clone};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ProgramParser;

const FLAGS: &'static [&'static str] = &["-f", "-h","-s"];

const HELPTEXT: &'static str = "lime - a simple image manipulation language\n
    usage: lime [flag] <data>\n
    -f <file>   parse script from file\n
    -s <script> parse script from commandline\n
    -h          print this help text\n";

// enum for commandline arguments and their values
#[derive(Debug)]
enum Flag {
    F(String),
    S(String),
    H,
    D,
}

// parse input string using pest grammar
fn parse(input:&String) -> Vec<Pair<'_, Rule>> {
    let pairs:Pairs<'_, Rule> = ProgramParser::parse(Rule::program, &input).unwrap_or_else(|e| panic!("{}", e));
    pairs.collect::<Vec<Pair<'_, Rule>>>()
}

fn direct_mode(input:&String,debug: bool) {
    println!("will parse script as raw input");
    let matches = parse(input);
    if !debug {return}
    for rule in matches {
        match rule.as_rule() {
            Rule::COMMENT => println!("Comment:  {}", rule.as_str()),
            Rule::expression => println!("Expression:   {}", rule.as_str()),
            Rule::functionCall => println!("Functioncall:   {}", rule.as_str()),
            Rule::EOI => println!("reached EOF"),
            _ => unreachable!()
        };
    }
}

fn file_mode(input:&String,debug: bool){
    println!("will parse script from file");
    let file = fs::read_to_string(&input).unwrap_or_else(|e| panic!("{}", e) );
    parse(&file);
}

fn arg_parser() {
    let args: Vec<String> = env::args().collect();                              // collect imput from commandline
    let mut counter: usize = 0;                                                 // counter for iterating over arguments
    let mut arguments:Vec<Flag> = vec!();                                       // vector for storing parsed arguments
    let mut debug = false;                                                // debug flag
    
    // iterate over arguments and parse them into the arguments vector
    for arg in &args {
        // break if all arguments are parsed
        if !(counter < args.len()) {
            break;
        }
        // check if argument is a flag and if it is a valid flag
        if arg.starts_with('-') && FLAGS.contains(&arg.as_str()) {
            match arg.as_str(){
                "-f" => arguments.push(Flag::F(args[counter+1].to_string())),
                "-s" => arguments.push(Flag::S(args[counter+1].to_string())),
                "-h" => arguments.push(Flag::H),
                "-d" => arguments.push(Flag::D),
                _ => println!("you should not be able to read this")
            }
        }
        counter +=1;
    }
    println!("{:?}",arguments);

    for argument in arguments {
        match argument {
            Flag::D => debug = true,
            Flag::F(file) => file_mode(&file,debug),
            Flag::S(script) => direct_mode(&script,debug),
            Flag::H => println!("{}",HELPTEXT),
        }
    }

}





fn main() {


    // 1. call program as `lime <script>`
    arg_parser();

    // 2. load script as lines

    // 3. parse whole script into some appropriate format (do this first completely,
    //      because its cheap; processing an image is expensive)

    // 4. execute parsing result
    // 4.1 when a function is called, extract function name, arguments
    //      and apply appropriate rust routine
    // 4.2 when a variable is assigned, evaluate right hand side expression and
    //      store result in some lookup table (definitely needed for layers as in
    //      e.g. Photoshop)
    // 4.3 ignore interactive calls for now
    // 4.4 opening and closing images are simply handled as functions
}