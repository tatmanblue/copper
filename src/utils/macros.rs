//!
#[macro_export]
macro_rules! str2string {
    ($s: expr) => {
        $s.to_string()
    };
}
