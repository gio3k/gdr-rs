use crate::assert_token_kind;
use crate::script::Location;
use crate::sponge::absorbers::statements::Statement;
use crate::sponge::Sponge;
use crate::stage0::tokens::TokenKind;

pub struct BlockStatement {
    pub location: Location,
    pub depth: u32,
    pub body: Vec<Statement>,
}

impl<'a> Sponge<'a> {
    /// Absorb a block statement
    /// The current sponge position should be at the "line start" above an indent token
    pub fn absorb_block_statement(&mut self) -> Statement {
        assert_token_kind!(self.token, TokenKind::IndentTab | TokenKind::IndentSpaces);

        let depth = self.absorb_indents_for_depth();

        // Set up block
        let mut block = BlockStatement {
            location: self.token.location,
            depth,
            body: vec![],
        };

        // Attempt to read first statement
        block.body.push(
            self.absorb_statement()
        );

        loop {
            // Check depth - make sure the next statement should be handled here
            let statement_depth =
            self.absorb_statement()
        }
    }
}