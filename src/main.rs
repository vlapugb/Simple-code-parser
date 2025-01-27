use clap::Parser;
use std::fs;
use std::io::Read;
use std::path::Path;
use pyo3::prelude::*;
mod modules;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        long = "filename",
        help = "Set path-file from root of the project to parse it"
    )]
    name: String,
}

#[macro_use]
extern crate lalrpop_util;

use crate::modules::dot_tree::to_dot;
use crate::modules::graphviz::plotting_graphviz;

lalrpop_mod!(
    #[allow(clippy::all)]
    grammar
);

fn main() {
    pyo3::prepare_freethreaded_python();

    let args = Args::parse();
    let file_path = Path::new(args.name.as_str());
    let mut file = fs::File::open(file_path).expect("File not found");
    let mut content = String::new();

    let parser = grammar::ProgramGrammarParser::new();
    let _ = file.read_to_string(&mut content);

    match parser.parse(content.as_str()) {
        Ok(stmts) => {
            let dot_tree = to_dot(&stmts);
            plotting_graphviz(dot_tree.as_str(), "ast.png").expect("Failed to plot graph");
            println!("Parsed: {}", dot_tree)
        }
        Err(e) => println!("Error: {:?}", e),
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_grammar_parser_stmt1() {
        let file_path = Path::new("./tests/test_grammar_parser_stmt1_dot_tree.txt");
        let mut file = fs::File::open(file_path).expect("File not found");
        let mut content = String::new();
        file.read_to_string(&mut content);
        let parser1 = grammar::ProgramGrammarParser::new();
        let source1 = "let x = 42";
        let parsed_statements = parser1.parse(source1).expect("Parsing failed");
        let dot_representation = to_dot(&parsed_statements);
        assert_eq!(dot_representation, content,);
    }
    #[test]
    fn test_grammar_parser_stmt2() {
        let file_path = Path::new("./tests/test_grammar_parser_stmt2_dot_tree.txt");
        let mut file = fs::File::open(file_path).expect("File not found");
        let mut content = String::new();
        file.read_to_string(&mut content);
        let parser2 = grammar::ProgramGrammarParser::new();
        let source2 = "print x";
        let parsed_statements = parser2.parse(source2).expect("Parsing failed");
        let dot_representation = to_dot(&parsed_statements);
        assert_eq!(dot_representation, content,);
    }
    #[test]
    fn test_grammar_parser_stmt3() {
        let file_path = Path::new("./tests/test_grammar_parser_stmt3_dot_tree.txt");
        let mut file = fs::File::open(file_path).expect("File not found");
        let mut content = String::new();
        file.read_to_string(&mut content);
        let parser3 = grammar::ProgramGrammarParser::new();
        let source3 = "let z = (y + 6) * 2";
        let parsed_statements = parser3.parse(source3).expect("Parsing failed");
        let dot_representation = to_dot(&parsed_statements);
        assert_eq!(dot_representation, content,);
    }
    #[test]
    fn test_grammar_parser_stmt4() {
        let file_path = Path::new("./tests/test_grammar_parser_stmt4_dot_tree.txt");
        let mut file = fs::File::open(file_path).expect("File not found");
        let mut content = String::new();
        file.read_to_string(&mut content);
        let parser4 = grammar::ProgramGrammarParser::new();
        let source4 = "let x = 42+3*5 let y = x+10 let z = (y*3) + (x+2) print z let a = (z+10)*(2+3) let b = (a+(x+3)*z) print b";
        let parsed_statements = parser4.parse(source4).expect("Parsing failed");
        let dot_representation = to_dot(&parsed_statements);
        assert_eq!(dot_representation, content,);
    }
}
