//!

use open::that;
use shell::shell_trait::ShellTrait;

pub struct Browser {
    pub file : String
}

impl ShellTrait for Browser {
    fn open(&self) {
        trace!("opening html page {}", self.file);
        that(self.file.to_string()).unwrap();
    }
}