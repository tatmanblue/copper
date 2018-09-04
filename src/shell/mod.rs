//! shell module is responsible for displaying test results formatted from an OutputTrait implementation
//! in the output mod.  ShellTypes are related to OutputTypes in that it makes no sense to
//! use console shell type when the output is html.  therefore OutputType implementations return
//! which shell type to use

pub mod browser;
pub mod console;
pub mod console_browser;
pub mod factory;
pub mod shell_trait;