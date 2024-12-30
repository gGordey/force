pub mod tokenizer {
    pub const TOKEN_SEPARATORS: [char; 21] = [' ', '/', '+', '-', '=', '_', '!', '&', '?', '*', '(', ')', '[', ']', '{', '}', '>', '<', ';', '.', '\n'];
    #[derive(PartialEq, Debug, Clone)]
    pub enum Token {
        None,
        FieldName(String),
        IntLit(i32),
        StrLit(String),
        BoolLit(bool),
        Semicolon, // ;
        Dot, // .
        MathEqual, // =
        MathMultiply, // *
        MathDivide, // /
        MathPlus, // +
        MathMinus, // -
        MathPower, // **
        Incriment, // ++
        Decriment, // --
        LogicEqual, // ==
        LogicAnd, // &&
        LogicOr, // ||
        LogicXor, // ^
        LogicNot, // !
        LogicGreater, // >
        LogicLess, // <
        LogicGreaterOrEqual, // >=
        LogicLessOrEqual, // <=
        BitXor, // ^^
        BitLeftShift, // <<
        BitRightShift, // >>
        Return, // ret
        VarInit, // var
        FnInit, // func
        TypeSpec(crate::Datatypes),
        GlobalExtension, // global
        CurlyBracketOpen(i32), // {  i32 = connected closed curly bracket
        CurlyBracketClose(i32), // }  i32 = connected opened curly bracket
        SquareBracketOpen(i32), // {  i32 = connected closed square bracket
        SquareBracketClose(i32), // }  i32 = connected opened square bracket
        RoundBracketOpen(i32), // {  i32 = connected closed round bracket
        RoundBracketClose(i32), // }  i32 = connected opened round bracket
    }
    impl Token {
        pub fn variant(&self, check: &Token) -> bool {
            std::mem::discriminant(self) == std::mem::discriminant(check)
        }
    }
    pub fn split_string(block: &str) -> Vec<&str> {
        let mut tokens: Vec<&str> = Vec::new();
        let mut last_token_pos = 0;
        for (i, c) in block.chars().enumerate() {
            if TOKEN_SEPARATORS.contains(&c) {
                if !&block[last_token_pos..i].is_empty()
                {
                    tokens.push(&block[last_token_pos..i]);
                }
                last_token_pos = i+1;
                tokens.push(&block[i..=i]); // adding symbols
            }
        }
        tokens.push(&block[last_token_pos..]);
        tokens
    }
    pub fn to_token_list(st: &Vec<&str>) -> Vec<Token> {
        let mut res: Vec<Token> = Vec::new();
        let mut i: usize = 0;
        while i < st.len(){
            let tok = st[i];
            let mut ct: Token = Token::None;
            if tok.is_empty() { }
            else if tok == "ret" { ct = Token::Return; }
            else if tok == ";" { ct = Token::Semicolon; }
            else if tok == "var" { ct = Token::VarInit; }
            else if tok == "func" { ct = Token::FnInit; }
            else if tok == "bool" { ct = Token::TypeSpec(crate::Datatypes::Bool); }
            else if tok == "int" { ct = Token::TypeSpec(crate::Datatypes::Int); }
            else if tok == "uint" { ct = Token::TypeSpec(crate::Datatypes::UInt); }
            else if tok == "byte" { ct = Token::TypeSpec(crate::Datatypes::Byte); }
            else if tok == "sbyte" { ct = Token::TypeSpec(crate::Datatypes::SByte); }
            else if tok == "global" { ct = Token::GlobalExtension; }
            else if tok == "+" { if st[i+1] == "+" { ct = Token::Incriment; i += 1 }  else { ct = Token::MathPlus; } }
            else if tok == "-" { if st[i+1] == "-" { ct = Token::Decriment; i += 1 }  else { ct = Token::MathMinus; } }
            else if tok == "=" { if st[i+1] == "=" { ct = Token::LogicEqual; i += 1 }  else { ct = Token::MathEqual; } }
            else if tok == "*" { if st[i+1] == "*" { ct = Token::MathPower; i += 1 }  else { ct = Token::MathMultiply; } }
            else if tok == "&" { if st[i+1] == "&" { ct = Token::LogicAnd; i += 1 }  else { /* Syntax error here */ } }
            else if tok == "|" { if st[i+1] == "|" { ct = Token::LogicOr; i += 1 }  else { /* Syntax error here */ } }
            else if tok == "^" { if st[i+1] == "^" { ct = Token::BitXor; i += 1 }  else { ct = Token::LogicXor; } }
            else if tok == "<" { if st[i+1] == "<" { ct = Token::BitLeftShift; i += 1 }  else { ct = Token::LogicLess; } }
            else if tok == ">" { if st[i+1] == ">" { ct = Token::BitRightShift; i += 1 }  else { ct = Token::LogicGreater; } }
            else if tok == "/" { ct = Token::MathDivide; }
            else if tok == "." { ct = Token::Dot; }
            else if tok == "{" { ct = Token::CurlyBracketOpen(0); }
            else if tok == "}" { ct = Token::CurlyBracketClose(0); }
            else if tok == "(" { ct = Token::RoundBracketOpen(0); }
            else if tok == ")" { ct = Token::RoundBracketClose(0); }
            else if tok == "[" { ct = Token::SquareBracketOpen(0); }
            else if tok == "]" { ct = Token::SquareBracketClose(0); }
            else if tok.chars().all(char::is_alphabetic) { ct = Token::FieldName(String::from(tok)); }
            else if tok.chars().all(char::is_numeric) { ct = Token::IntLit(tok.parse().expect("Bad")); }
            if ct != Token::None {
                res.push(ct);
            }
            i += 1;
        }
        res
    }
    pub fn connect_brackets(tklist: &mut Vec<Token>) { // used after cheking for all brackets to be closed correctly, we dont expect errors here
        let mut brc_stack: Vec<(Token, usize)> = Vec::new();
        for i in 0..tklist.len() {
            if tklist[i].variant(&Token::CurlyBracketOpen(0)) {
                brc_stack.push((Token::CurlyBracketOpen(0), i));
            } else if tklist[i].variant(&Token::CurlyBracketClose(0)) {
                tklist[i] = Token::CurlyBracketClose(brc_stack[brc_stack.len()-1].1 as i32);
                tklist[brc_stack[brc_stack.len()-1].1] = Token::CurlyBracketOpen(i as i32);
                brc_stack.pop();
            }
            if tklist[i].variant(&Token::RoundBracketOpen(0)) {
                brc_stack.push((Token::RoundBracketOpen(0), i));
            } else if tklist[i].variant(&Token::RoundBracketClose(0)) {
                tklist[i] = Token::RoundBracketClose(brc_stack[brc_stack.len()-1].1 as i32);
                tklist[brc_stack[brc_stack.len()-1].1] = Token::RoundBracketOpen(i as i32);
                brc_stack.pop();
            }
            if tklist[i].variant(&Token::SquareBracketOpen(0)) {
                brc_stack.push((Token::SquareBracketOpen(0), i));
            } else if tklist[i].variant(&Token::SquareBracketClose(0)) {
                tklist[i] = Token::SquareBracketClose(brc_stack[brc_stack.len()-1].1 as i32);
                tklist[brc_stack[brc_stack.len()-1].1] = Token::SquareBracketOpen(i as i32);
                brc_stack.pop();
            }
        }
    }
}