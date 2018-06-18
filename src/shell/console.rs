//!

use shell::shell_trait::ShellTrait;

pub struct Console {

}

impl ShellTrait for Console {
    fn open(&self) {
        trace!("opening at console...output already created");
    }
}