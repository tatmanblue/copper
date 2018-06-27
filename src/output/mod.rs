//! Output mod is responsibly for formatting the test results after its been organized into displayable
//! data. Output can be to console or as web page, depending on settings.  Output mod is used in
//! conjunction with the shell mod

pub mod console;
pub mod factory;
pub mod html_generator;
pub mod output_trait;