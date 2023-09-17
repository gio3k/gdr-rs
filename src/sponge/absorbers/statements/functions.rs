use crate::assert_token_kind;
use crate::core::literal::Literal;
use crate::script::Location;
use crate::sponge::absorbers::statements::Statement;
use crate::sponge::Sponge;
use crate::sponge::sponge_issues::error_kind::ErrorKind;
use crate::stage0::tokens::TokenKind;

pub struct FunctionParameter {
    pub location: Location,
    pub name: Literal,
    pub type_hint: Option<Literal>,
}

pub struct FunctionDeclaration {
    pub location: Location,
    pub name: Literal,
    pub parameters: Vec<FunctionParameter>,
    pub body: Statement,
}

impl<'a> Sponge<'a> {
    pub fn absorb_function_statement(&mut self) -> Statement {
        assert_token_kind!(self.token, TokenKind::Function);

        // Move on to the function name
        self.scan();
        if !matches!(self.token.kind, TokenKind::Identifier) {
            self.throw_error_here(ErrorKind::UnexpectedIdentifier);
            return Statement::Invalid;
        }

        let name_location = self.token.location;
        let name = self.token.value;

        // Move on to the set start
        self.scan();
        if !matches!(self.token.kind, TokenKind::BracketRoundOpen) {
            self.throw_error_here(ErrorKind::UnexpectedIdentifier);
            self.throw_error_here(ErrorKind::TokenRequirementNotFound(TokenKind::BracketRoundOpen));
            return Statement::Invalid;
        }

        let mut result = FunctionDeclaration {
            location: self.token.location.expand(name_location),
            name,
            parameters: vec![],
            body: Statement::Invalid,
        };

        // Move on to the function parameter name or set end
        loop {
            self.scan();
            match self.token.kind {
                TokenKind::BracketRoundClosed => {
                    break;
                }

                TokenKind::Comma => {}

                TokenKind::Identifier => {
                    match self.absorb_function_parameter() {
                        None => continue, // Some sort of parameter parse error occurred
                        Some(param) => {
                            result.parameters.push(param)
                        }
                    }
                }

                _ => {
                    // Invalid token - throw and return the incomplete statement
                    self.throw_error_here(ErrorKind::UnexpectedIdentifier);
                    return Statement::FunctionDeclaration(Box::from(result));
                }
            }
        }

        // This token needs to be a BracketRoundClosed - otherwise the sponge has done something wrong
        assert_token_kind!(self.token, TokenKind::BracketRoundClosed);

        // Scan to the colon token
        self.scan();

        // Update location after scan
        result.location = result.location.expand(self.token.location);

        if !matches!(self.token.kind, TokenKind::Colon) {
            self.throw_error_here(ErrorKind::TokenRequirementNotFound(TokenKind::Colon));
            self.throw_error_here(ErrorKind::UnexpectedIdentifier);
            return Statement::FunctionDeclaration(Box::from(result));
        }

        // Scan to the next token, which should be an indent
        self.scan();

        // Update location after scan
        result.location = result.location.expand(self.token.location);

        // Skip line breaks
        self.skip_line_breaks();

        match self.token.kind {
            TokenKind::IndentTab | TokenKind::IndentSpaces => {
                result.body = self.absorb_block_statement();
            }

            TokenKind::None => {
                self.throw_error_here(ErrorKind::UnexpectedEof);
                return Statement::FunctionDeclaration(Box::from(result));
            }

            _ => {
                self.throw_error_here(ErrorKind::UnexpectedIdentifier);
                self.throw_error_here(ErrorKind::FunctionBlockNotFound);
                return Statement::FunctionDeclaration(Box::from(result));
            }
        }

        // Update location after block statement was absorbed
        result.location = result.location.expand(self.token.location);

        // Return
        Statement::FunctionDeclaration(Box::from(result))
    }

    fn absorb_function_parameter(&mut self) -> Option<FunctionParameter> {
        assert_token_kind!(self.token, TokenKind::Identifier);

        let name_location = self.token.location;
        let name = self.token.value;

        // Move on
        self.scan();

        // This is either an unrelated token, type hint marker or end of set
        match self.token.kind {
            TokenKind::Colon => {}
            TokenKind::Comma | TokenKind::BracketRoundClosed => {
                return Some(
                    FunctionParameter {
                        location: name_location,
                        name,
                        type_hint: None,
                    }
                );
            }
            TokenKind::Assignment => {
                // Function defaults aren't supported
                self.throw_error_here(ErrorKind::UnexpectedIdentifier);
                self.throw_error_here(ErrorKind::FunctionParameterDefaultsUnsupported);
                return None;
            }
            TokenKind::None => {
                self.throw_error_here(ErrorKind::UnexpectedEof);
                return None;
            }
            _ => {
                self.throw_error_here(ErrorKind::UnexpectedIdentifier);
                return None;
            }
        }

        // Read the type hint
        // Skip the colon
        self.scan();

        return match self.token.kind {
            TokenKind::Identifier => {
                let type_hint_location = self.token.location;
                let type_hint = self.token.value;

                Some(
                    FunctionParameter {
                        location: Location::new(name_location.start, type_hint_location.end),
                        name,
                        type_hint: Some(type_hint),
                    }
                )
            }
            TokenKind::None => {
                self.throw_error_here(ErrorKind::UnexpectedEof);
                None
            }
            _ => {
                self.throw_error_here(ErrorKind::UnexpectedIdentifier);
                None
            }
        };
    }
}