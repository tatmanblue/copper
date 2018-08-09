//! behaviors for displaying the results

/**
    Defines the behaviors for displaying the results (aka output).  Since the system is meant to be flexible there
    may be different ways of displaying it.

    Current implementations of different methods are:  browser for html results, console for text.
*/
pub trait ShellTrait {
    fn open(&self);
}