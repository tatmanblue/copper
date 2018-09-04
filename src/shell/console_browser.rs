//! For displaying output to both console and browser

use shell::shell_trait::ShellTrait;
use shell::factory::ShellFactory;

pub struct ConsoleBrowser {
    pub file : String
}

/**
    ConsoleBrowser is an implementation of ShellTrait that invokes both console and browser
    shell implementations
*/
impl ShellTrait for ConsoleBrowser {
    fn open(&self) {
        ShellFactory::get("console", "").open();
        ShellFactory::get("browser", &self.file).open();
    }
}