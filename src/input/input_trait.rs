//!

/**
    InputTrait is designed to get input from a source (pipe, file, stdio etc)
*/
pub trait InputTrait {
    fn read_all() -> Vec<String>;
}