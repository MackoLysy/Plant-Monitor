use chrono::Local;
extern crate pretty_env_logger;
use std::io::Write;
pub fn enable_logger() {
    pretty_env_logger::init();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
