use std::num::{ParseFloatError, ParseIntError};
use crate::ast::core::literals::Literal;
use crate::ast::nodes::functions::{FunctionArgument, FunctionDefinition};
use crate::ast::nodes::variables::VariableDefinition;
use crate::ast::reader::TokenReaderState;
use crate::lexer::ScriptToken;

mod nodes;
mod reader;
pub mod core;

pub struct Root {
    pub annotations: Vec<Node>,
    pub body: Vec<Node>,
}

impl Root {
    pub fn new() -> Root {
        Root {
            annotations: vec![],
            body: vec![],
        }
    }
}

pub enum Node {
    SingleLiteral(Literal),
    Root(Root),
    VariableDefinition(VariableDefinition),
    FunctionArgument(FunctionArgument),
    FunctionDefinition(FunctionDefinition),
}

pub enum SyntaxError {
    Other(String),
    UnexpectedTokenForLiteral(ScriptToken),
    InvalidFloatLiteral(ParseFloatError),
    InvalidIntLiteral(ParseIntError),
}

fn absorb_root(current: &Node, state: &mut TokenReaderState, tokens: &Vec<ScriptToken>) {
    absorb_root_annotations(&current, state, &tokens);

    loop {}
}

fn absorb_root_annotations(current: &Node, state: &mut TokenReaderState, tokens: &Vec<ScriptToken>) {
    loop {
        if state.empty() {
            break;
        }

        let token = state.peek();

        println!("tk: {:?}", token);

        state.next_no_return();
    }
}

/// Create syntax tree from tokenized script
pub fn create_syntax_tree(tokens: Vec<ScriptToken>) -> Node {
    // Create our root node
    let root = Node::Root(Root::new());

    // Set our current node to that root node
    let mut current: &Node = &root;

    let mut state = TokenReaderState::new(tokens);

    if state.empty() {
        panic!("create_syntax_tree: empty state");
    }

    absorb_root(&current, &mut state, &tokens);

    root
}