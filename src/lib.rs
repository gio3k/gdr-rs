pub mod lexer;

#[cfg(test)]
mod tests {
    use crate::lexer;

    #[test]
    fn create_lexer_and_parse() {
        let script = "@hello('hi') func my_function(a, b, c):";
        let mut lexer = lexer::Lexer::new(script.chars());

        loop {
            println!("{}", lexer.parse());

            match lexer.token() {
                None => break,
                Some(token) => {
                    println!("Token: {:?}", token);
                }
            }
        }

        println!("OK!");
    }
}