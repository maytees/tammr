use tammr::lexer::{KeywordType, TokenType};
use tammr::Lexer;

#[cfg(test)]
mod test {

    #[test]
    fn test_string() {
        use super::Lexer;

        let input = String::from(r#""Hello, World!""#);
        let mut l = Lexer::new(input);
        let tokens = l.gen_tokens();

        assert_eq!(tokens[0].ttype, super::TokenType::String);
        assert_eq!(tokens[0].literal, String::from("Hello, World!"));
    }

    #[test]
    fn lexer_test() {
        use super::{KeywordType, Lexer, TokenType};

        let input = String::from(
            r#"
            let five = 5;
            let ten = 10;
            "#,
        );
        let mut l = Lexer::new(input);
        let tokens = l.gen_tokens();

        assert_eq!(tokens[0].ttype, TokenType::Keyword(KeywordType::Let));
        assert_eq!(tokens[0].literal, String::from("let"));
        assert_eq!(tokens[1].ttype, TokenType::Ident);
        assert_eq!(tokens[1].literal, String::from("five"));
        assert_eq!(tokens[2].ttype, TokenType::Assign);
        assert_eq!(tokens[2].literal, String::from("="));
        assert_eq!(tokens[3].ttype, TokenType::Number);
        assert_eq!(tokens[3].literal, String::from("5"));
        assert_eq!(tokens[4].ttype, TokenType::Semicolon);
        assert_eq!(tokens[4].literal, String::from(";"));

        assert_eq!(tokens[5].ttype, TokenType::Keyword(KeywordType::Let));
        assert_eq!(tokens[5].literal, String::from("let"));
        assert_eq!(tokens[6].ttype, TokenType::Ident);
        assert_eq!(tokens[6].literal, String::from("ten"));
        assert_eq!(tokens[7].ttype, TokenType::Assign);
        assert_eq!(tokens[7].literal, String::from("="));
        assert_eq!(tokens[8].ttype, TokenType::Number);
        assert_eq!(tokens[8].literal, String::from("10"));
        assert_eq!(tokens[9].ttype, TokenType::Semicolon);
        assert_eq!(tokens[9].literal, String::from(";"));
    }
}
