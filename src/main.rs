extern crate pest;

#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::{fs,env};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ProgramParser;



fn parse(input:String) {
    let pairs = ProgramParser::parse(Rule::program, &input).unwrap_or_else(|e| panic!("{}", e));

    // 
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::COMMENT => println!("Comment:  {}", inner_pair.as_str()),
                Rule::expression => println!("Expression:   {}", inner_pair.as_str()),
                Rule::functionCall => println!("Functioncall:   {}", inner_pair.as_str()),
                Rule::EOI => println!("reached EOF"),
                _ => unreachable!()
            };
        }
    }
}

fn direct_mode(input:String) {
    println!("will parse script as raw input");
    parse(input);
}

fn file_mode(input:&Vec<String>){
    let file = fs::read_to_string(&input[2]).unwrap_or_else(|e| panic!("{}", e) );
    println!("debug{:?}",file);
    parse(file);
}

fn main() {

    // 1. call program as `lime <script>`
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    match arg_len {
        1 => println!("please specify input or get help using -h"),
        2 => direct_mode(args[0].to_string()),
        3 => file_mode(&args),
        _ => println!("wtf is your problem ?"),
    }

    //parse(example);
    
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
