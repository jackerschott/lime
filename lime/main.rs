extern crate pest;

#[macro_use]
extern crate pest_derive;

use pest::{Parser, iterators::Pair};
use std::{fs,env, vec};

#[derive(Parser)]
#[grammar = "lime/grammar.pest"]
struct ScriptParser;

fn parse(script: &String) -> Vec<Pair<'_, Rule>> {
    // TODO: handle parsing errors
    let pairs = ScriptParser::parse(Rule::program, &script).unwrap();
    return pairs.collect::<Vec<Pair<'_, Rule>>>();
}

#[derive(Debug, PartialEq)]
enum CmdLineOption {
    ScriptFile(String), Script(String), Help,
}

fn parse_args() -> Vec<CmdLineOption> {
    let args: Vec<String> = env::args().collect();
    let mut options = vec!();
    
    for i in 0..args.len() {
        if !args[i].starts_with('-') {
            continue
        }

        match args[i].as_str(){
            "-f" => options.push(CmdLineOption::ScriptFile(args[i+1].to_string())),
            "-s" => options.push(CmdLineOption::Script(args[i+1].to_string())),
            "-h" => options.push(CmdLineOption::Help),
            _ => continue,
        }
    }

    return options
}

fn main() {
    // 1. call program as `lime <script>`
    let options = parse_args();
    if options.contains(&CmdLineOption::Help) {
        println!(r#"lime - a simple image manipulation language
            "    usage: lime [flag] <data>
            "    -f <file>   parse script from file
            "    -s <script> parse script from string
            "    -h          print this help text"#);
        return;
    }

    let mut script_content = String::new();
    for option in options {
        match option {
            CmdLineOption::ScriptFile(script_path) => {
                // TODO: handle read errors
                script_content = fs::read_to_string(&script_path).unwrap();
            }
            CmdLineOption::Script(content) => script_content = content,
            _ => continue
        }
    }

    if script_content.is_empty() {
        println!("[main] No script provided");
    }

    let matches = parse(&script_content);

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
