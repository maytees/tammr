// #[cfg(test)]
// mod parser_tests {
//     use tammr::ast::Statement;
//     use tammr::lexer::Lexer;
//     use tammr::Parser;

//     #[test]
//     fn test_empty_hash() {
//         let input = String::from("{}");

//         let mut l = Lexer::new(input);
//         let tokens = l.gen_tokens();

//         let mut p = Parser::new(tokens);

//         let program = p.parse_program();

//         if let Some(program) = program {
//             if program.len() != 1 {
//                 panic!(
//                     "Program does not contain 1 statement, got {}, prgm: {:?}",
//                     program.len(),
//                     program
//                 );
//             }
//             let stmt = &program[0];
//             match stmt {
//                 Statement::Expression { value, .. } => {
//                     if value.to_string() != "{}" {
//                         panic!("Expected value to be {{}}, got {}", value);
//                     }
//                 }
//                 _ => {
//                     panic!("Expected statement to be expression, got {:?}", stmt);
//                 }
//             }
//         }
//     }

//     #[test]
//     fn test_hash_literal() {
//         let input = String::from(r#"{"one": 1, "two": 2, "three": 3}"#);

//         let mut l = Lexer::new(input);
//         let tokens = l.gen_tokens();

//         let mut p = Parser::new(tokens);

//         let program = p.parse_program();

//         if let Some(program) = program {
//             if program.len() != 1 {
//                 panic!(
//                     "Program does not contain 1 statement, got {}, prgm: {:?}",
//                     program.len(),
//                     program
//                 );
//             }
//             let stmt = &program[0];
//             match stmt {
//                 Statement::Expression { value, .. } => {
//                     if value.to_string() != r#"{one: 1, two: 2, three: 3}"# {
//                         panic!(
//                             "Expected value to be {{one: 1, two: 2, three: 3}}, got {}",
//                             value
//                         );
//                     }
//                 }
//                 _ => {
//                     panic!("Expected statement to be expression, got {:?}", stmt);
//                 }
//             }
//         }
//     }

//     #[test]
//     fn test_array_index() {
//         let input = String::from("myArray[1 + 1];");

//         let mut l = Lexer::new(input);
//         let tokens = l.gen_tokens();

//         let mut p = Parser::new(tokens);
//         let program = p.parse_program();
//         println!("{:?}", program);
//         if let Some(program) = program {
//             if program.len() != 1 {
//                 panic!(
//                     "Program does not contain 1 statement, got {}",
//                     program.len()
//                 );
//             }
//             let stmt = &program[0];
//             match stmt {
//                 Statement::Expression { value, .. } => {
//                     if value.to_string() != "(myArray[(1 + 1)])" {
//                         panic!("Expected value to be (myArray[(1 + 1)]), got {}", value);
//                     }
//                 }
//                 _ => {
//                     panic!("Expected statement to be expression, got {:?}", stmt);
//                 }
//             }
//         }
//     }

//     #[test]
//     fn test_array_literal() {
//         let input = String::from("[1, 2 * 2, 3 + 3]");

//         let mut l = Lexer::new(input);
//         let tokens = l.gen_tokens();

//         let mut p = Parser::new(tokens);
//         let program = p.parse_program();
//         if let Some(program) = program {
//             if program.len() != 1 {
//                 panic!(
//                     "Program does not contain 1 statement, got {}",
//                     program.len()
//                 );
//             }

//             let stmt = &program[0];

//             match stmt {
//                 Statement::Expression { value, .. } => {
//                     if value.to_string() != "[1, (2 * 2), (3 + 3)]" {
//                         panic!("Expected value to be [1, (2 * 2), (3 + 3)], got {}", value);
//                     }
//                 }
//                 _ => {
//                     panic!("Expected statement to be expression, got {:?}", stmt);
//                 }
//             }
//         } else {
//             panic!("Parse program returned None");
//         }
//     }

//     #[test]
//     fn test_string_literal() {
//         let input = String::from("\"hello world\";");

//         let mut l = Lexer::new(input);
//         let tokens = l.gen_tokens();

//         let mut p = Parser::new(tokens);
//         let program = p.parse_program();
//         if let Some(program) = program {
//             if program.len() != 1 {
//                 panic!(
//                     "Program does not contain 1 statement, got {}",
//                     program.len()
//                 );
//             }

//             let stmt = &program[0];
//             match stmt {
//                 Statement::Expression { value, .. } => {
//                     if value.to_string() != "hello world" {
//                         panic!("Expected value to be hello world, got {}", value);
//                     }
//                 }
//                 _ => {
//                     panic!("Expected statement to be expression, got {:?}", stmt);
//                 }
//             }
//         }
//     }

//     #[test]
//     fn eq_test() {
//         let input = String::from("5 == 5;");

//         let mut l = Lexer::new(input);
//         let tokens = l.gen_tokens();

//         let mut p = Parser::new(tokens);

//         let program = p.parse_program();

//         if let Some(program) = program {
//             if program.len() != 1 {
//                 panic!(
//                     "Program does not contain 1 statement, got {}, prgm: {:?}",
//                     program.len(),
//                     program
//                 );
//             }
//             let stmt = &program[0];
//             match stmt {
//                 Statement::Expression { value, .. } => {
//                     if value.to_string() != "(5 == 5)" {
//                         panic!("Expected value to be (5 == 5), got {}", value);
//                     }
//                 }
//                 _ => {
//                     panic!("Expected statement to be expression, got {:?}", stmt);
//                 }
//             }
//         }
//     }

//     #[test]
//     fn fn_call() {
//         let input = String::from("add(1, 2 * 3, 4 + 5);");

//         let mut l = Lexer::new(input);
//         let tokens = l.gen_tokens();

//         let mut p = Parser::new(tokens);
//         let program = p.parse_program();
//         if let Some(program) = program {
//             if program.len() != 1 {
//                 panic!(
//                     "Program does not contain 1 statement, got {}",
//                     program.len()
//                 );
//             }

//             let stmt = &program[0];

//             match stmt {
//                 Statement::Expression { value, .. } => {
//                     if value.to_string() != "add(1, (2 * 3), (4 + 5))" {
//                         panic!(
//                             "Expected value to be add(1, (2 * 3), (4 + 5)), got {}",
//                             value
//                         );
//                     }
//                 }
//                 _ => {
//                     panic!("Expected statement to be expression, got {:?}", stmt);
//                 }
//             }
//         } else {
//             panic!("Parse program returned None");
//         }
//     }

//     #[test]
//     fn fn_literal() {
//         let input = String::from("fn(x, y) { x + y; }");

//         let mut l = Lexer::new(input);
//         let tokens = l.gen_tokens();

//         let mut p = Parser::new(tokens);
//         let program = p.parse_program();
//         if let Some(program) = program {
//             if program.len() != 1 {
//                 panic!(
//                     "Program does not contain 1 statement, got {}",
//                     program.len()
//                 );
//             }

//             let stmt = &program[0];

//             match stmt {
//                 Statement::Expression { value, .. } => {
//                     if value.to_string() != "fn(x, y) {[(x + y)]}" {
//                         panic!("Expected value to be fn(x, y) {{[(x + y)]}}, got {}", value);
//                     }
//                 }
//                 _ => {
//                     panic!("Expected statement to be expression, got {:?}", stmt);
//                 }
//             }
//         } else {
//             panic!("Parse program returned None");
//         }
//     }

//     #[test]
//     fn if_statement() {
//         let input = String::from(
//             r#"
//             if x < y {
//                 return x;
//             } else {
//                 return y;
//             }
//             "#,
//         );
//         let mut l = Lexer::new(input);
//         let tokens = l.gen_tokens();

//         let mut p = Parser::new(tokens);
//         let program = p.parse_program();
//         if let Some(program) = program {
//             if program.len() != 1 {
//                 panic!(
//                     "Program does not contain 1 statement, got {}",
//                     program.len()
//                 );
//             }

//             let stmt = &program[0];
//             match stmt {
//                 Statement::Expression { value, .. } => {
//                     if value.to_string() != "((x < y) {[return x;]} else [return y;])" {
//                         panic!(
//                             "Expected value to be ((x < y) {{[ return true; ]}} else {{[ return false; ]}}), got {}",
//                             value
//                         );
//                     }
//                 }
//                 _ => {
//                     panic!("Expected statement to be expression, got {:?}", stmt);
//                 }
//             }
//         } else {
//             panic!("Parse program returned None");
//         }
//     }

//     #[test]
//     fn group_expr() {
//         let input = String::from("(5 + 5) * 2;");
//         let mut l = Lexer::new(input);
//         let tokens = l.gen_tokens();

//         let mut p = Parser::new(tokens);
//         let program = p.parse_program();

//         if let Some(program) = program {
//             if program.len() != 1 {
//                 panic!(
//                     "Program does not contain 1 statement, got {}, program: {:?}",
//                     program.len(),
//                     program
//                 );
//             }

//             let stmt = &program[0];
//             match stmt {
//                 Statement::Expression { value, .. } => {
//                     if value.to_string() != "((5 + 5) * 2)" {
//                         panic!("Expected value to be ((5 + 5) * 2), got {}", value);
//                     }
//                 }
//                 _ => {
//                     panic!("Expected statement to be expression, got {:?}", stmt);
//                 }
//             }
//         } else {
//             panic!("Parse program returned None");
//         }
//     }

//     #[test]
//     fn boolean_expr() {
//         let input = String::from("true;");

//         let mut l = Lexer::new(input);
//         let tokens = l.gen_tokens();

//         let mut p = Parser::new(tokens);
//         let program = p.parse_program();

//         if let Some(program) = program {
//             if program.len() != 1 {
//                 panic!(
//                     "Program does not contain 1 statement, got {}",
//                     program.len()
//                 );
//             }

//             let stmt = &program[0];
//             match stmt {
//                 Statement::Expression { value, .. } => {
//                     if value.to_string() != "true" {
//                         panic!("Expected value to be true, got {}", value);
//                     }
//                 }
//                 _ => {
//                     panic!("Expected statement to be expression, got {:?}", stmt);
//                 }
//             }
//         } else {
//             panic!("Parse program returned None");
//         }
//     }

//     #[test]
//     fn infix_expr() {
//         let input = String::from("5 + 5 * 2;");
//         let mut l = Lexer::new(input);
//         let tokens = l.gen_tokens();

//         let mut p = Parser::new(tokens);
//         let program = p.parse_program();

//         if let Some(program) = program {
//             if program.len() != 1 {
//                 panic!(
//                     "Program does not contain 1 statement, got {}",
//                     program.len()
//                 );
//             }

//             let stmt = &program[0];
//             match stmt {
//                 Statement::Expression { value, .. } => {
//                     if value.to_string() != "(5 + (5 * 2))" {
//                         panic!("Expected value to be (5 + (5 * 2)), got {}", value);
//                     }
//                 }
//                 _ => {
//                     panic!("Expected statement to be expression, got {:?}", stmt);
//                 }
//             }
//         } else {
//             panic!("Parse program returned None");
//         }
//     }

//     #[test]
//     fn prefix_expr() {
//         let input = String::from("-5;");
//         let mut l = Lexer::new(input);
//         let tokens = l.gen_tokens();

//         let mut p = Parser::new(tokens);
//         let program = p.parse_program();

//         if let Some(program) = program {
//             if program.len() != 1 {
//                 panic!(
//                     "Program does not contain 1 statement, got {}",
//                     program.len()
//                 );
//             }

//             let stmt = &program[0];
//             match stmt {
//                 Statement::Expression { value, .. } => {
//                     if value.to_string() != "(-5)" {
//                         panic!("Expected value to be -5, got {}", value);
//                     }
//                 }
//                 _ => {
//                     panic!("Expected statement to be expression, got {:?}", stmt);
//                 }
//             }
//         } else {
//             panic!("Parse program returned None");
//         }
//     }

//     #[test]
//     fn integer_expr() {
//         let input = String::from("5;");

//         let mut l = Lexer::new(input);
//         let tokens = l.gen_tokens();

//         let mut p = Parser::new(tokens);
//         let program = p.parse_program();

//         if let Some(program) = program {
//             if program.len() != 1 {
//                 panic!(
//                     "Program does not contain 1 statement, got {}",
//                     program.len()
//                 );
//             }

//             let stmt = &program[0];
//             match stmt {
//                 Statement::Expression { value, .. } => {
//                     if value.to_string() != "5" {
//                         panic!("Expected value to be 5, got {}", value);
//                     }
//                 }
//                 _ => {
//                     panic!("Expected statement to be expression, got {:?}", stmt);
//                 }
//             }
//         } else {
//             panic!("Parse program returned None");
//         }
//     }

//     #[test]
//     fn identifier_expr() {
//         let input = String::from("foobar;");

//         let mut l = Lexer::new(input);
//         let tokens = l.gen_tokens();

//         let mut p = Parser::new(tokens);
//         let program = p.parse_program();

//         if let Some(program) = program {
//             if program.len() != 1 {
//                 panic!(
//                     "Program does not contain 1 statement, got {}",
//                     program.len()
//                 );
//             }

//             let stmt = &program[0];
//             match stmt {
//                 Statement::Expression { value, .. } => {
//                     if value.to_string() != "foobar" {
//                         panic!("Expected value to be foobar, got {}", value);
//                     }
//                 }
//                 _ => {
//                     panic!("Expected statement to be expression, got {:?}", stmt);
//                 }
//             }
//         } else {
//             panic!("Parse program returned None");
//         }
//     }

//     #[test]
//     fn return_statement() {
//         let input = String::from(
//             r#"
//             return 5;
//             return 10;
//             return 993322;
//             "#,
//         );
//         let mut l = Lexer::new(input);
//         let tokens = l.gen_tokens();

//         let mut p = Parser::new(tokens);
//         let program = p.parse_program();
//         if let Some(program) = program {
//             if program.len() != 3 {
//                 panic!(
//                     "Program does not contain 3 statements, got {}",
//                     program.len()
//                 );
//             }

//             // let tests = vec!["5", "10", "993322"];

//             // for (i, tt) in tests.iter().enumerate() {
//             //     let stmt = &program[i];
//             //     match stmt {
//             //         Statement::return { value, .. } => {
//             //             if value.to_string() != tt.to_string() {
//             //                 panic!("Expected value to be {}, got {}", tt, value);
//             //             }
//             //         }
//             //         _ => {
//             //             panic!("Expected statement to be return, got {:?}", stmt);
//             //         }
//             //     }
//             // }
//         } else {
//             panic!("Parse program returned None");
//         }
//     }

//     #[test]
//     fn let_statement() {
//         let input = String::from(
//             r#"
//             let x = 5;
//             let y = 10;
//             let foobar = 838383;
//             "#,
//         );
//         let mut l = Lexer::new(input);
//         let tokens = l.gen_tokens();

//         let mut p = Parser::new(tokens);
//         let program = p.parse_program();

//         if let Some(program) = program {
//             if program.len() != 3 {
//                 panic!(
//                     "Program does not contain 3 statements, got {}",
//                     program.len()
//                 );
//             }

//             let tests = vec!["x", "y", "foobar"];

//             for (i, tt) in tests.iter().enumerate() {
//                 let stmt = &program[i];
//                 match stmt {
//                     Statement::Let { name, .. } => {
//                         if name.value != *tt {
//                             panic!("Expected name to be {}, got {}", tt, name);
//                         }
//                     }
//                     _ => {
//                         panic!("Expected statement to be let, got {:?}", stmt);
//                     }
//                 }
//             }
//         } else {
//             panic!("Parse program returned None");
//         }
//     }
// }
