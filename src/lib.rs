#[derive(PartialEq, Debug, Clone)]
pub enum Datatypes {
    Bool,
    Int,
    UInt,
    Byte,
    SByte,
    StrLit,
    Empty,
}
pub mod precompiler {
    pub mod tokenizer;
    pub mod validity_checker;
}
pub mod interpreter {
    pub mod encode;
}