#[derive(PartialEq, Debug)]
pub enum Datatypes {
    Bool,
    Int,
    UInt,
    Byte,
    SByte,
    StrLit,
}
pub mod precompiler {
    pub mod tokenizer;
}