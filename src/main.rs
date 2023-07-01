extern crate pest;

#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::{fs,env, vec};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ProgramParser;

#[derive(Debug)]
struct CmdArgument{
    flag:String,
    value:String
}


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

fn file_mode(input:String){
    let file = fs::read_to_string(&input).unwrap_or_else(|e| panic!("{}", e) );
    println!("debug{:?}",file);
    parse(file);
}

fn arg_parser() {
    let args: Vec<String> = env::args().collect();
    let number_arguments: usize = args.len();
    let mut counter: usize = 0;
    let mut arguments:Vec<CmdArgument> = vec!();
    for arg in &args {
        if !(counter < number_arguments) {
            break;
        }
        if arg.starts_with('-') {
            arguments.push(CmdArgument { flag: arg.to_string(), value: args[counter+1].to_string() })
        }
        counter +=1;
    }
    println!("{:?}",arguments);
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