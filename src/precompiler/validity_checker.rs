pub mod validity_checker {
    use crate::precompiler::tokenizer::tokenizer;
    use tokenizer::Token;
    pub fn are_brackets_valid(tklist: &Vec<Token>) -> bool {
        let mut brc_stack: Vec<Token> = Vec::new();
        for tok in tklist {
            if  tok.variant(&Token::CurlyBracketOpen(0)) ||
                tok.variant(&Token::SquareBracketOpen(0)) ||
                tok.variant(&Token::RoundBracketOpen(0)) {
                    brc_stack.push(tok.clone());
            } else if tok.variant(&Token::CurlyBracketClose(0)) ||
                    tok.variant(&&Token::SquareBracketClose(0)) ||
                    tok.variant(&Token::RoundBracketClose(0)) {
                        if  brc_stack.is_empty() ||
                            (!brc_stack[brc_stack.len()-1].variant(&Token::CurlyBracketOpen(0)) && tok.variant(&Token::CurlyBracketClose(0))) ||
                            (!brc_stack[brc_stack.len()-1].variant(&Token::RoundBracketOpen(0)) && tok.variant(&Token::RoundBracketClose(0))) ||
                            (!brc_stack[brc_stack.len()-1].variant(&Token::SquareBracketOpen(0)) && tok.variant(&Token::SquareBracketClose(0)))
                            {
                                
                                return false;
                            }
                        brc_stack.pop();
            }
        }
        brc_stack.len() == 0
    }
}