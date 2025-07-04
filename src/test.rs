#[cfg(test)]
use super::*;

#[test]
fn handle_one_char_token() {
    let source = "(( ))";
    let mut scanner = Lexer::new(source);
    let tokens = scanner.tokenize();
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0], Token::LParen);
    assert_eq!(tokens[1], Token::LParen);
    assert_eq!(tokens[2], Token::RParen);
    assert_eq!(tokens[3], Token::RParen);
    assert_eq!(tokens[4], Token::Eof);
}

#[test]
fn handle_int_value_decleration() {
    let source = "int a = 10;";
    let mut scanner = Lexer::new(source);
    let tokens = scanner.tokenize();
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0], Token::Int);
    assert_eq!(tokens[1], Token::Ident("a".to_string()));
    assert_eq!(tokens[2], Token::Assign);
    assert_eq!(tokens[3], Token::NumberLiteral(10.0));
    assert_eq!(tokens[4], Token::Semicolon);
    assert_eq!(tokens[5], Token::Eof);
}
#[test]
fn handle_neg_int_value_decleration() {
    let source = "int a = -10;";
    let mut scanner = Lexer::new(source);
    let tokens = scanner.tokenize();
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0], Token::Int);
    assert_eq!(tokens[1], Token::Ident("a".to_string()));
    assert_eq!(tokens[2], Token::Assign);
    assert_eq!(tokens[3], Token::NumberLiteral(-10.0));
    assert_eq!(tokens[4], Token::Semicolon);
    assert_eq!(tokens[5], Token::Eof);
}
#[test]
fn handle_long_value_decleration() {
    let source = "long b = 100;";
    let mut scanner = Lexer::new(source);
    let tokens = scanner.tokenize();
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0], Token::Long);
    assert_eq!(tokens[1], Token::Ident("b".to_string()));
    assert_eq!(tokens[2], Token::Assign);
    assert_eq!(tokens[3], Token::NumberLiteral(100.0));
    assert_eq!(tokens[4], Token::Semicolon);
    assert_eq!(tokens[5], Token::Eof);
}

#[test]
fn handle_float_value_decleration() {
    let source = "float c = 1.2;";
    let mut scanner = Lexer::new(source);
    let tokens = scanner.tokenize();
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0], Token::Float);
    assert_eq!(tokens[1], Token::Ident("c".to_string()));
    assert_eq!(tokens[2], Token::Assign);
    assert_eq!(tokens[3], Token::NumberLiteral(1.2));
    assert_eq!(tokens[4], Token::Semicolon);
    assert_eq!(tokens[5], Token::Eof);
}
#[test]
fn handle_neg_float_value_decleration() {
    let source = "float c = -1.2;";
    let mut scanner = Lexer::new(source);
    let tokens = scanner.tokenize();
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0], Token::Float);
    assert_eq!(tokens[1], Token::Ident("c".to_string()));
    assert_eq!(tokens[2], Token::Assign);
    assert_eq!(tokens[3], Token::NumberLiteral(-1.2));
    assert_eq!(tokens[4], Token::Semicolon);
    assert_eq!(tokens[5], Token::Eof);
}
#[test]
fn handle_double_value_decleration() {
    let source = "double d = 3.14;";
    let mut scanner = Lexer::new(source);
    let tokens = scanner.tokenize();
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0], Token::Double);
    assert_eq!(tokens[1], Token::Ident("d".to_string()));
    assert_eq!(tokens[2], Token::Assign);
    assert_eq!(tokens[3], Token::NumberLiteral(3.14));
    assert_eq!(tokens[4], Token::Semicolon);
    assert_eq!(tokens[5], Token::Eof);
}

#[test]
fn handle_string_value_decleration() {
    let source = "string e = \"this is a test\";";
    let mut scanner = Lexer::new(source);
    let tokens = scanner.tokenize();
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0], Token::String);
    assert_eq!(tokens[1], Token::Ident("e".to_string()));
    assert_eq!(tokens[2], Token::Assign);
    assert_eq!(
        tokens[3],
        Token::StringLiteral("this is a test".to_string())
    );
    assert_eq!(tokens[4], Token::Semicolon);
    assert_eq!(tokens[5], Token::Eof);
}

#[test]
fn handle_char_value_decleration() {
    let source = "char f = 'f';";
    let mut scanner = Lexer::new(source);
    let tokens = scanner.tokenize();
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0], Token::Char);
    assert_eq!(tokens[1], Token::Ident("f".to_string()));
    assert_eq!(tokens[2], Token::Assign);
    assert_eq!(tokens[3], Token::StringLiteral("f".to_string()));
    assert_eq!(tokens[4], Token::Semicolon);
    assert_eq!(tokens[5], Token::Eof);
}

#[test]
fn handle_boolean_value_decleration() {
    let source = "bool k = true;";
    let mut scanner = Lexer::new(source);
    let tokens = scanner.tokenize();
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0], Token::Bool);
    assert_eq!(tokens[1], Token::Ident("k".to_string()));
    assert_eq!(tokens[2], Token::Assign);
    assert_eq!(tokens[3], Token::True);
    assert_eq!(tokens[4], Token::Semicolon);
    assert_eq!(tokens[5], Token::Eof);
}

#[test]
fn handle_table_value_decleration() {
    let source = r#"
        table j = {
            int t1 = 1,
            float t2 = -1.1,
            vec t3 = [1,2,3]
        };
    "#;

    let mut scanner = Lexer::new(source);
    let tokens = scanner.tokenize();

    assert_eq!(tokens[0], Token::Table);
    assert_eq!(tokens[1], Token::Ident("j".to_string()));
    assert_eq!(tokens[2], Token::Assign);
    assert_eq!(tokens[3], Token::LBrace);

    // First field
    assert_eq!(tokens[4], Token::Int);
    assert_eq!(tokens[5], Token::Ident("t1".to_string()));
    assert_eq!(tokens[6], Token::Assign);
    assert_eq!(tokens[7], Token::NumberLiteral(1.0));
    assert_eq!(tokens[8], Token::Comma);

    // Second field
    assert_eq!(tokens[9], Token::Float);
    assert_eq!(tokens[10], Token::Ident("t2".to_string()));
    assert_eq!(tokens[11], Token::Assign);
    assert_eq!(tokens[12], Token::NumberLiteral(-1.1));
    assert_eq!(tokens[13], Token::Comma);

    // Third field
    assert_eq!(tokens[14], Token::Vec);
    assert_eq!(tokens[15], Token::Ident("t3".to_string()));
    assert_eq!(tokens[16], Token::Assign);
    assert_eq!(tokens[17], Token::LBracket);
    assert_eq!(tokens[18], Token::NumberLiteral(1.0));
    assert_eq!(tokens[19], Token::Comma);
    assert_eq!(tokens[20], Token::NumberLiteral(2.0));
    assert_eq!(tokens[21], Token::Comma);
    assert_eq!(tokens[22], Token::NumberLiteral(3.0));
    assert_eq!(tokens[23], Token::RBracket);

    assert_eq!(tokens[24], Token::RBrace);
    assert_eq!(tokens[25], Token::Semicolon);
    assert_eq!(tokens[26], Token::Eof);
}

#[test]
fn handle_vec_value_decleration() {
    let source = "vec x = [1,2,3];";
    let mut scanner = Lexer::new(source);
    let tokens = scanner.tokenize();
    assert_eq!(tokens.len(), 12);
    assert_eq!(tokens[0], Token::Vec);
    assert_eq!(tokens[1], Token::Ident("x".to_string()));
    assert_eq!(tokens[2], Token::Assign);
    assert_eq!(tokens[3], Token::LBracket);
    assert_eq!(tokens[4], Token::NumberLiteral(1.0));
    assert_eq!(tokens[5], Token::Comma);
    assert_eq!(tokens[6], Token::NumberLiteral(2.0));
    assert_eq!(tokens[7], Token::Comma);
    assert_eq!(tokens[8], Token::NumberLiteral(3.0));
    assert_eq!(tokens[9], Token::RBracket);
    assert_eq!(tokens[10], Token::Semicolon);
    assert_eq!(tokens[11], Token::Eof);
}

#[test]
fn handle_void_function() {
    let source = "void function test() end";
    let mut scanner = Lexer::new(source);
    let tokens = scanner.tokenize();
    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[0], Token::Void);
    assert_eq!(tokens[1], Token::Function);
    assert_eq!(tokens[2], Token::Ident("test".to_string()));
    assert_eq!(tokens[3], Token::LParen);
    assert_eq!(tokens[4], Token::RParen);
    assert_eq!(tokens[5], Token::End);
    assert_eq!(tokens[6], Token::Eof);
}

#[test]
fn handle_global_void_function() {
    let source = "global void test1 = function() end";
    let mut scanner = Lexer::new(source);
    let tokens = scanner.tokenize();
    assert_eq!(tokens.len(), 9);
    assert_eq!(tokens[0], Token::Global);
    assert_eq!(tokens[1], Token::Void);
    assert_eq!(tokens[2], Token::Ident("test1".to_string()));
    assert_eq!(tokens[3], Token::Assign);
    assert_eq!(tokens[4], Token::Function);
    assert_eq!(tokens[5], Token::LParen);
    assert_eq!(tokens[6], Token::RParen);
    assert_eq!(tokens[7], Token::End);
    assert_eq!(tokens[8], Token::Eof);
}

#[test]
fn handle_int_function() {
    let source = "int function test3() return (int)12 end";
    let mut scanner = Lexer::new(source);
    let tokens = scanner.tokenize();
    assert_eq!(tokens.len(), 12);
    assert_eq!(tokens[0], Token::Int);
    assert_eq!(tokens[1], Token::Function);
    assert_eq!(tokens[2], Token::Ident("test3".to_string()));
    assert_eq!(tokens[3], Token::LParen);
    assert_eq!(tokens[4], Token::RParen);
    assert_eq!(tokens[5], Token::Return);
    assert_eq!(tokens[6], Token::LParen);
    assert_eq!(tokens[7], Token::Int);
    assert_eq!(tokens[8], Token::RParen);
    assert_eq!(tokens[9], Token::NumberLiteral(12.0));
    assert_eq!(tokens[10], Token::End);
    assert_eq!(tokens[11], Token::Eof);
}

#[test]
fn handle_long_function() {
    let source = "long function test4() long a = 111 return a end";
    let mut scanner = Lexer::new(source);
    let tokens = scanner.tokenize();
    assert_eq!(tokens.len(), 13);
    assert_eq!(tokens[0], Token::Long);
    assert_eq!(tokens[1], Token::Function);
    assert_eq!(tokens[2], Token::Ident("test4".to_string()));
    assert_eq!(tokens[3], Token::LParen);
    assert_eq!(tokens[4], Token::RParen);
    assert_eq!(tokens[5], Token::Long);
    assert_eq!(tokens[6], Token::Ident("a".to_string()));
    assert_eq!(tokens[7], Token::Assign);
    assert_eq!(tokens[8], Token::NumberLiteral(111.0));
    assert_eq!(tokens[9], Token::Return);
    assert_eq!(tokens[10], Token::Ident("a".to_string()));
    assert_eq!(tokens[11], Token::End);
    assert_eq!(tokens[12], Token::Eof);
}

#[test]
fn handle_anonymous_void_function() {
    let source = "void function() end";
    let mut scanner = Lexer::new(source);
    let tokens = scanner.tokenize();
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0], Token::Void);
    assert_eq!(tokens[1], Token::Function);
    assert_eq!(tokens[2], Token::LParen);
    assert_eq!(tokens[3], Token::RParen);
    assert_eq!(tokens[4], Token::End);
    assert_eq!(tokens[5], Token::Eof);
}

#[test]
fn handle_void_fastcall_function() {
    let source = "void () end";
    let mut scanner = Lexer::new(source);
    let tokens = scanner.tokenize();
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0], Token::Void);
    assert_eq!(tokens[1], Token::LParen);
    assert_eq!(tokens[2], Token::RParen);
    assert_eq!(tokens[3], Token::End);
    assert_eq!(tokens[4], Token::Eof);
}

#[test]
fn handle_for_loop() {
    let source = "for k,v in pairs(j) do print(k,v) end";
    let mut scanner = Lexer::new(source);
    let tokens = scanner.tokenize();

    assert_eq!(tokens[0], Token::For);
    assert_eq!(tokens[1], Token::Ident("k".to_string()));
    assert_eq!(tokens[2], Token::Comma);
    assert_eq!(tokens[3], Token::Ident("v".to_string()));
    assert_eq!(tokens[4], Token::In);
    assert_eq!(tokens[5], Token::Ident("pairs".to_string()));
    assert_eq!(tokens[6], Token::LParen);
    assert_eq!(tokens[7], Token::Ident("j".to_string()));
    assert_eq!(tokens[8], Token::RParen);
    assert_eq!(tokens[9], Token::Do);
    assert_eq!(tokens[10], Token::Ident("print".to_string()));
    assert_eq!(tokens[11], Token::LParen);
    assert_eq!(tokens[12], Token::Ident("k".to_string()));
    assert_eq!(tokens[13], Token::Comma);
    assert_eq!(tokens[14], Token::Ident("v".to_string()));
    assert_eq!(tokens[15], Token::RParen);
    assert_eq!(tokens[16], Token::End);
    assert_eq!(tokens[17], Token::Eof);
}

#[test]
fn handle_operators() {
    let source = "+ - * / % ^ # == ~= <= >= < > = ( ) { } [ ] ; : :: , . .. ...";
    let mut scanner = Lexer::new(source);
    let tokens = scanner.tokenize();

    assert_eq!(tokens[0], Token::Plus);
    assert_eq!(tokens[1], Token::Minus);
    assert_eq!(tokens[2], Token::Mul);
    assert_eq!(tokens[3], Token::Div);
    assert_eq!(tokens[4], Token::Mod);
    assert_eq!(tokens[5], Token::Pow);
    assert_eq!(tokens[6], Token::Len);
    assert_eq!(tokens[7], Token::Eq);
    assert_eq!(tokens[8], Token::Ne);
    assert_eq!(tokens[9], Token::Le);
    assert_eq!(tokens[10], Token::Ge);
    assert_eq!(tokens[11], Token::Lt);
    assert_eq!(tokens[12], Token::Gt);
    assert_eq!(tokens[13], Token::Assign);
    assert_eq!(tokens[14], Token::LParen);
    assert_eq!(tokens[15], Token::RParen);
    assert_eq!(tokens[16], Token::LBrace);
    assert_eq!(tokens[17], Token::RBrace);
    assert_eq!(tokens[18], Token::LBracket);
    assert_eq!(tokens[19], Token::RBracket);
    assert_eq!(tokens[20], Token::Semicolon);
    assert_eq!(tokens[21], Token::Colon);
    assert_eq!(tokens[22], Token::DoubleColon);
    assert_eq!(tokens[23], Token::Comma);
    assert_eq!(tokens[24], Token::Dot);
    assert_eq!(tokens[25], Token::Concat);
    assert_eq!(tokens[26], Token::Ellipsis);
    assert_eq!(tokens[27], Token::Eof);
}

#[test]
fn handle_keywords() {
    let source = "and break do else elseif end false for function goto if in local global nil not or repeat return then true until while int long float double string table bool char void vec";
    let mut scanner = Lexer::new(source);
    let tokens = scanner.tokenize();

    assert_eq!(tokens[0], Token::And);
    assert_eq!(tokens[1], Token::Break);
    assert_eq!(tokens[2], Token::Do);
    assert_eq!(tokens[3], Token::Else);
    assert_eq!(tokens[4], Token::ElseIf);
    assert_eq!(tokens[5], Token::End);
    assert_eq!(tokens[6], Token::False);
    assert_eq!(tokens[7], Token::For);
    assert_eq!(tokens[8], Token::Function);
    assert_eq!(tokens[9], Token::Goto);
    assert_eq!(tokens[10], Token::If);
    assert_eq!(tokens[11], Token::In);
    assert_eq!(tokens[12], Token::Local);
    assert_eq!(tokens[13], Token::Global);
    assert_eq!(tokens[14], Token::Nil);
    assert_eq!(tokens[15], Token::Not);
    assert_eq!(tokens[16], Token::Or);
    assert_eq!(tokens[17], Token::Repeat);
    assert_eq!(tokens[18], Token::Return);
    assert_eq!(tokens[19], Token::Then);
    assert_eq!(tokens[20], Token::True);
    assert_eq!(tokens[21], Token::Until);
    assert_eq!(tokens[22], Token::While);
    assert_eq!(tokens[23], Token::Int);
    assert_eq!(tokens[24], Token::Long);
    assert_eq!(tokens[25], Token::Float);
    assert_eq!(tokens[26], Token::Double);
    assert_eq!(tokens[27], Token::String);
    assert_eq!(tokens[28], Token::Table);
    assert_eq!(tokens[29], Token::Bool);
    assert_eq!(tokens[30], Token::Char);
    assert_eq!(tokens[31], Token::Void);
    assert_eq!(tokens[32], Token::Vec);
    assert_eq!(tokens[33], Token::Eof);
}
