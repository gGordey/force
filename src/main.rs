use force::precompiler::tokenizer::*;
use force::precompiler::validity_checker::*;
fn main() {
    let text_block: &str = 
    "global func() main { 
        var byte vector = 255;
        ret vector; 
    }";
    let mut programm_tokens = tokenizer::to_token_list(&tokenizer::split_string(text_block));
    if !validity_checker::are_brackets_valid(&programm_tokens) {
        println!("Syntax Error: Brackets are not closed correctly!");
        return;
    }
    tokenizer::connect_brackets(&mut programm_tokens);
    println!("{:#?}", programm_tokens);
}
