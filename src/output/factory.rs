//!

use processors::types::OrganizedTestResults;
use output::console::ConsoleOutput;
use output::html_generator::HtmlOutput;
use output::output_trait::OutputTrait;
use shell::factory::{ShellFactory, ShellTypes};

/**
    OutputTypes enums becomes a "placeholder" (so to speak) of the concrete implementation of
    OutputTrait which handles the actual generate function.   This allows getting around "known size"
    requirements rust has on return types.

    It solves for the O in solid:  open for extension.  add a new enum and you add a new behavior into the system
    without breaking existing implementations
*/
pub enum OutputTypes {
    Console(ConsoleOutput),
    Html(HtmlOutput),
}

/**
    Think of this OutputTrait implementation as something along the lines of an abstract class pointer
    or a v-table pointer in an OO language.

    The factory returns one of the enums along with the associated implementation type allocated within the enum itself.

    Since consumers have the enum, the OutputTrait implementation on the enum allows the consumer to work with the
    enum as a "class pointer" and not have to know about the actual implementation details; thereby making the
    consumer decoupled from the implementation.

    The concrete implementation are in their respective files.
*/
impl OutputTrait for OutputTypes {
    fn generate(&self, test_results : &OrganizedTestResults) -> ShellTypes {
        match *self {
            OutputTypes::Console(ref console) => return console.generate(test_results),
            OutputTypes::Html(ref html) => return html.generate(test_results),
        }
    }
}

/**
*/
pub struct OutputFactory {}

impl OutputFactory {
    pub fn get(name : &str) -> OutputTypes {
        match name.as_ref() {
            "console" => OutputTypes::Console(ConsoleOutput {} ),
            "html" => OutputTypes::Html(HtmlOutput {} ),
            "debug" =>  OutputTypes::Console(ConsoleOutput {} ),
            "default" => OutputTypes::Html(HtmlOutput {} ),
            &_ => {
                eprintln!("Error in OutputFactory processing get for name of '{}'", name);
                panic!("Error in OutputFactory processing get request")
            },
        }
    }
}