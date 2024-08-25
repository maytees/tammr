#[cfg(test)]
mod lexer_test {

    #[test]
    fn test_double_quote_string() {
        use tammr::Lexer;

        let input = String::from(r#""Hello, World!""#);
        let mut l = Lexer::new(input);
        let tokens = l.gen_tokens();

        assert_eq!(tokens[0].ttype, tammr::lexer::TokenType::String);
        assert_eq!(tokens[0].literal, String::from("Hello, World!"));
    }

    #[test]
    fn test_single_quote_string() {
        use tammr::Lexer;

        let input = String::from(r#"'Hello, World!'"#);
        let mut l = Lexer::new(input);
        let tokens = l.gen_tokens();

        assert_eq!(tokens[0].ttype, tammr::lexer::TokenType::String);
        assert_eq!(tokens[0].literal, String::from("Hello, World!"));
    }

    #[test]
    fn test_single_quote_string_with_double_quote() {
        use tammr::Lexer;

        let input = String::from("'Hello, \"World\"!'");
        let mut l = Lexer::new(input);
        let tokens = l.gen_tokens();

        assert_eq!(tokens[0].ttype, tammr::lexer::TokenType::String);
        assert_eq!(tokens[0].literal, String::from("Hello, \"World\"!"));
    }

    #[test]
    fn test_double_quote_string_with_single_quote() {
        use tammr::Lexer;

        let input = String::from("\"Hello, 'World'!\"");
        let mut l = Lexer::new(input);
        let tokens = l.gen_tokens();

        assert_eq!(tokens[0].ttype, tammr::lexer::TokenType::String);
        assert_eq!(tokens[0].literal, String::from("Hello, 'World'!"));
    }

    #[test]
    fn simple_tokens_test() {
        use tammr::lexer::{KeywordType, Lexer, TokenType};

        let input = String::from(
            r#"
    // Comment Test
    import "math.tmr" as math
    let x = 5
    let y = 10
    function add(a, b) do
        return a + b
    end
    if x > y do
        println("x is greater")
    else do
        println("y is greater or equal")
    end
    let array = [1, 2, 3]
    let hash = {"key": "value"}
    "#,
        );

        let mut l = Lexer::new(input);
        let tokens = l.gen_tokens();

        let expected_tokens: Vec<(TokenType, &str)> = vec![
            (TokenType::Keyword(KeywordType::Import), "import"),
            (TokenType::String, "math.tmr"),
            (TokenType::Keyword(KeywordType::As), "as"),
            (TokenType::Ident, "math"),
            (TokenType::Keyword(KeywordType::Let), "let"),
            (TokenType::Ident, "x"),
            (TokenType::Assign, "="),
            (TokenType::Number, "5"),
            (TokenType::Keyword(KeywordType::Let), "let"),
            (TokenType::Ident, "y"),
            (TokenType::Assign, "="),
            (TokenType::Number, "10"),
            (TokenType::Keyword(KeywordType::Fn), "function"),
            (TokenType::Ident, "add"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "a"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "b"),
            (TokenType::RParen, ")"),
            (TokenType::Keyword(KeywordType::Do), "do"),
            (TokenType::Keyword(KeywordType::Return), "return"),
            (TokenType::Ident, "a"),
            (TokenType::Add, "+"),
            (TokenType::Ident, "b"),
            (TokenType::Keyword(KeywordType::End), "end"),
            (TokenType::Keyword(KeywordType::If), "if"),
            (TokenType::Ident, "x"),
            (TokenType::Gt, ">"),
            (TokenType::Ident, "y"),
            (TokenType::Keyword(KeywordType::Do), "do"),
            (TokenType::Ident, "println"),
            (TokenType::LParen, "("),
            (TokenType::String, "x is greater"),
            (TokenType::RParen, ")"),
            (TokenType::Keyword(KeywordType::Else), "else"),
            (TokenType::Keyword(KeywordType::Do), "do"),
            (TokenType::Ident, "println"),
            (TokenType::LParen, "("),
            (TokenType::String, "y is greater or equal"),
            (TokenType::RParen, ")"),
            (TokenType::Keyword(KeywordType::End), "end"),
            (TokenType::Keyword(KeywordType::Let), "let"),
            (TokenType::Ident, "array"),
            (TokenType::Assign, "="),
            (TokenType::LBracket, "["),
            (TokenType::Number, "1"),
            (TokenType::Comma, ","),
            (TokenType::Number, "2"),
            (TokenType::Comma, ","),
            (TokenType::Number, "3"),
            (TokenType::RBracket, "]"),
            (TokenType::Keyword(KeywordType::Let), "let"),
            (TokenType::Ident, "hash"),
            (TokenType::Assign, "="),
            (TokenType::LBrace, "{"),
            (TokenType::String, "key"),
            (TokenType::Colon, ":"),
            (TokenType::String, "value"),
            (TokenType::RBrace, "}"),
            (TokenType::Eof, ""),
        ];

        assert_eq!(
            tokens.len(),
            expected_tokens.len(),
            "Unexpected number of tokens"
        );

        for (i, (expected_type, expected_literal)) in expected_tokens.iter().enumerate() {
            assert_eq!(
                tokens[i].ttype, *expected_type,
                "Token type mismatch at index {}",
                i
            );
            assert_eq!(
                tokens[i].literal, *expected_literal,
                "Token literal mismatch at index {}",
                i
            );
        }
    }

    #[test]
    fn test_keywords() {
        use tammr::lexer::{KeywordType, Lexer, TokenType};

        let input = String::from(
        "let function return if else do end loop exit true false null try catch throw and or not is"
    );
        let mut l = Lexer::new(input);
        let tokens = l.gen_tokens();

        let expected_keywords = vec![
            (KeywordType::Let, "let"),
            (KeywordType::Fn, "function"), // Note: KeywordType::Fn for "function"
            (KeywordType::Return, "return"),
            (KeywordType::If, "if"),
            (KeywordType::Else, "else"),
            (KeywordType::Do, "do"),
            (KeywordType::End, "end"),
            (KeywordType::Loop, "loop"),
            (KeywordType::Exit, "exit"),
            (KeywordType::True, "true"),
            (KeywordType::False, "false"),
            (KeywordType::Null, "null"),
            (KeywordType::Try, "try"),
            (KeywordType::Catch, "catch"),
            (KeywordType::Throw, "throw"),
            (KeywordType::And, "and"),
            (KeywordType::Or, "or"),
            (KeywordType::Not, "not"),
            (KeywordType::Is, "is"),
        ];

        assert_eq!(tokens.len(), expected_keywords.len() + 1); // +1 for EOF token

        for (i, (expected_type, expected_literal)) in expected_keywords.iter().enumerate() {
            assert_eq!(
                tokens[i].ttype,
                TokenType::Keyword(expected_type.clone()),
                "Mismatch at token {}",
                i
            );
            assert_eq!(
                tokens[i].literal,
                expected_literal.to_string(),
                "Mismatch at token {}",
                i
            );
        }

        // Check the last token is EOF
        assert_eq!(tokens.last().unwrap().ttype, TokenType::Eof);
    }
}
