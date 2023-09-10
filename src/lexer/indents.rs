use crate::error_here;
use crate::lexer::{Lexer, LexerError};

impl<'a> Lexer<'a> {
    /// Get the current indent / scope depth
    pub(crate) fn parse_current_indent_depth(&mut self) -> Result<i32, LexerError> {
        let mut is_spaces: bool = false;
        let mut count: i32 = 0;

        loop {
            match self.see() {
                Some('\t') => {
                    if is_spaces {
                        // We're given tabs but we've already seen spaces? Error!
                        return error_here!(self, IndentTypeMismatch);
                    }

                    count += 1;
                }
                Some(' ') => {
                    is_spaces = true;

                    count += 1;
                }
                None | _ => break,
            }

            self.advance();
        }

        let space_tab_size: i32 = 4;
        if is_spaces {
            Ok(count / space_tab_size)
        } else {
            Ok(count)
        }
    }
}