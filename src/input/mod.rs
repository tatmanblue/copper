//! Input mod contains the functions that aggregate test result data so that it be processed.
//! Input can come from stdin or a file.

pub mod factory;
pub mod from_file;
pub mod from_stdin;
pub mod input_trait;