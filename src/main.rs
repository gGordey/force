use force::precompiler::tokenizer::*;
use force::precompiler::validity_checker::*;
use force::interpreter::encode::encode::mark_fields;
fn main() {
    let text_block: &str = 
    "global func(var int argc) int main { 
        var byte vector = { 255 * { 1 } };
        {}
        ret vector; 
    }";
    let mut programm_tokens = tokenizer::to_token_list(&tokenizer::split_string(text_block));
    if !validity_checker::are_brackets_valid(&programm_tokens) {
        println!("Syntax Error: Brackets are not closed correctly!");
        return;
    }
    tokenizer::connect_brackets(&mut programm_tokens);
    let fields = mark_fields(&programm_tokens);
    println!("{:#?}", programm_tokens);
    println!("{:#?}", fields);
}