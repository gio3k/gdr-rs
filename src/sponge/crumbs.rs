use crate::sponge::absorbers::blocks::BlockStatement;

pub enum Expression {}

pub enum Statement {
    BlockStatement(Box<BlockStatement>)
}