extern crate pest;

#[macro_use]
extern crate pest_derive;
use pest::Parser;


#[derive(Parser)]
#[grammar = "grammar.pest"]
struct IdentParser;



fn parse(input:String) {
    let pairs = IdentParser::parse(Rule::program, &input).unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
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
                _ => unreachable!()
            };
        }
    }
}

fn main() {
    let example:String = "// hi \nwhats = test()\r".to_string();
    // 1. call program as `lime <script>`
    let args = std::env::args();
    println!("{:?}",args);
    parse(example);
    
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
