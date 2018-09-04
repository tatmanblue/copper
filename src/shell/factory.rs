//! ShellTypes identify the different means of displaying formatted test results
//! ShellTypes implement ShellTrait and open their display

use shell::shell_trait::ShellTrait;
use shell::browser::Browser;
use shell::console::Console;
use shell::console_browser::ConsoleBrowser;

/**
    ShellTypes enum.  Abstracts the means of creating the display of the formatted test results

    Unlike other enums/factories patterns in the application, ShellTypes is used a little differently.
    Environment configuration does not determine which ShellType to use.  Rather, ShellType
    is determined by OutputType

    It solves for the O in solid:  open for extension.  add a new enum and you add a new behavior into the system
    without breaking existing implementations
*/
pub enum ShellTypes {
    /**
        represents the standard console
    */
    Console(Console),
    /**
        represents html display in a browser
    */
    Browser(Browser),
    /**
        dittly do it does both
    */
    CombinedConsoleBrowser(ConsoleBrowser)
}


/**
    Think of this ShellTrait implementation as something along the lines of an abstract class pointer
    or a v-table pointer in an OO language.

    The factory returns one of the ShellTypes enums along with the associated implementation type allocated
    within the enum itself.

    Since consumers have the ShellTypes enum, the ShellTrait implementation on the ShellTypes enum allows the
    consumer to work with the enum as a "class pointer" and not have to know about the actual implementation
    details; thereby making the consumer decoupled from the implementation.

    The concrete implementations for ShellTrait are in their respective files.

*/
impl ShellTrait for ShellTypes {
    /**
    */
    fn open(&self) {
        match *self {
            ShellTypes::Console(ref console) => console.open(),
            ShellTypes::Browser(ref browser) => browser.open(),
            ShellTypes::CombinedConsoleBrowser(ref combined) => combined.open(),
        }
    }
}

/**
*/
pub struct ShellFactory {}

impl ShellFactory {
    /**
        allocates a ShellType
        name = the shell expected
        data = specific to the shell type, data it needs to correctly spin up.

        Since the OutputTypes knows which shell type it wants, it understands what data means.
    */
    pub fn get(name : &str, data : &str) -> ShellTypes {
        match name.as_ref() {
            "console" => ShellTypes::Console(Console {} ),
            "browser" => ShellTypes::Browser(Browser { file : data.to_string() } ),
            "combined" => ShellTypes::CombinedConsoleBrowser(ConsoleBrowser {file : data.to_string() }),
            &_ => {
                eprintln!("Error in ShellFactory processing get for name of '{}'", name);
                panic!("Error in ShellFactory processing get request")
            },
        }
    }
}