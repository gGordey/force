pub mod encode { // all methhods are used after validity check
    use crate::Datatypes;
    use crate::precompiler::tokenizer::tokenizer::Token;
    #[derive(Debug)]
    pub enum Access {
        Local(usize), // usize = parent namespace ind in Vec<Field>
        Global,
    }
    #[derive(Debug)]
    pub struct Field {
        pub datatype: Datatypes,
        pub name: Option<String>,
        pub loc : usize,
        pub acs : Access,
    }
    pub fn mark_fields(tklist: &Vec<Token>) -> Vec<Field> {
        let mut fld_list: Vec<Field> = Vec::new();
        let mut brc_stack: Vec<usize> = Vec::new();
        for i in 0..tklist.len() {
            let tok = &tklist[i];
            if tok.variant(&Token::CurlyBracketOpen(0)) {
                brc_stack.push(brc_stack.len());
                let mut new_field = Field {
                    datatype: Datatypes::Empty,
                    name: None,
                    loc: if brc_stack.len() > 0 { brc_stack[brc_stack.len() - 1] } else { 0 },
                    acs: Access::Local(0)
                };
                if i >= 1 && tklist[i - 1].variant(&Token::FieldName(String::new())) {
                    new_field.name = if let Token::FieldName(value) = &tklist[i-1] { Some(value.to_string()) } else {None};
                } 
                let mut j = i -1;
                while j > 0 {
                    if  tklist[j].variant(&Token::CurlyBracketClose(0)) { brc_stack.pop(); }
                    if  tklist[j].variant(&Token::CurlyBracketClose(0)) ||
                        tklist[j].variant(&Token::Semicolon) ||
                        tklist[j].variant(&Token::CurlyBracketOpen(0)) {
                            break;
                    } else if tklist[j].variant(&Token::RoundBracketClose(0)) {
                        j = if let Token::RoundBracketClose(start) = tklist[j] { start as usize } else { 0 }
                    } else if tklist[j].variant(&Token::GlobalExtension) {
                        new_field.acs = Access::Global;
                    } else if tklist[j].variant(&Token::TypeSpec(Datatypes::Empty)) {
                        new_field.datatype = if let Token::TypeSpec(dt) = &tklist[j] { dt.clone() } else { Datatypes::Empty };
                    } 
                    j -= 1;
                }
                fld_list.push(new_field);
            }
        }
        fld_list
    }
}