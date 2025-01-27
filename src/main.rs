
#[macro_use]
extern crate lalrpop_util;

use crate::parse::to_dot;

lalrpop_mod!(#[allow(clippy::all)] grammar);

mod ast;
mod parse;

fn main() {
    let parser = grammar::ProgramGrammarParser::new();
    let source = "let x = 42+3*5 let y = x+10 let z = (y*3) + (x+2) print z let a = (z+10)*(2+3) let b = (a+(x+3)*z) print b
";


    match parser.parse(source) {
        Ok(stmts) => {
            let dot_tree = to_dot(&stmts);
            println!("Parsed: {}", dot_tree)
        },
        Err(e) => println!("Error: {:?}", e),
    }

}


#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::{self, Read};
    use std::path::Path;
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
        assert_eq!(
            dot_representation,
            content,
        );
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
        assert_eq!(
            dot_representation,
            content,
        );
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
        assert_eq!(
            dot_representation,
            content,
        );
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
        assert_eq!(
            dot_representation,
            content,
        );
    }
}
