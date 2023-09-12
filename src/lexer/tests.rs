#[cfg(test)]
mod lexer_tests {
    use crate::lexer;
    use crate::lexer::core::token::{TokenKind, TokenValue};

    #[test]
    fn basic_structure() {
        let mut lexer = lexer::Lexer::new("@annotation func test():".chars());

        assert!(matches!(lexer.next_token().unwrap().kind, TokenKind::LanguageIndent));
        assert!(matches!(lexer.next_token().unwrap().kind, TokenKind::LanguageAnnotation));
        lexer.next_token();
        lexer.next_token();
        lexer.next_token();
        lexer.next_token();
        assert!(matches!(lexer.next_token().unwrap().kind, TokenKind::Colon));
    }

    #[test]
    fn short_string_value() {
        let input = "'Hello, world!'";
        let expected_string = "Hello, world!";
        let mut lexer = lexer::Lexer::new(input.chars());

        // Skip indent
        lexer.next_token();

        match lexer.next_token()
            .expect("String token not found")
            .value {
            TokenValue::Symbol(su32) => {
                assert_eq!(
                    lexer.resolve_symbol(su32).expect("Failed to resolve symbol"),
                    expected_string
                );
            }
            v => {
                panic!("Invalid TokenValue {:?} for token", v);
            }
        }
    }

    #[test]
    fn indent_values() {
        let input = "            var a = 123";
        let mut lexer = lexer::Lexer::new(input.chars());

        let token = lexer.next_token()
            .expect("Failed to get first token");

        assert!(matches!(token.kind, TokenKind::LanguageIndent));
        assert!(matches!(token.value, TokenValue::Integer(3)));
    }

    #[test]
    fn number_literal_types() {
        let input = "30 40.123 -8000";
        let mut lexer = lexer::Lexer::new(input.chars());

        // Skip indent
        lexer.next_token();

        match lexer.next_token()
            .expect("Failed to get token")
            .kind {
            TokenKind::IntegerLiteral => {}
            v => {
                panic!("Invalid TokenKind {:?} for token", v);
            }
        }

        match lexer.next_token()
            .expect("Failed to get token")
            .kind {
            TokenKind::FloatLiteral => {}
            v => {
                panic!("Invalid TokenKind {:?} for token", v);
            }
        }

        match lexer.next_token()
            .expect("Failed to get token")
            .kind {
            TokenKind::IntegerLiteral => {}
            v => {
                panic!("Invalid TokenKind {:?} for token", v);
            }
        }
    }
}