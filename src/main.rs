use force::precompiler::tokenizer::*;

fn main() {
    let text_block: &str = 
    "global func main() { 
        var byte vector = 12.13;
        ret 0; 
    }";
    let mut programm_tokens = tokenizer::to_token_list(&tokenizer::split_string(text_block));
    tokenizer::connect_brackets(&mut programm_tokens);
    println!("{:#?}", programm_tokens);
}
