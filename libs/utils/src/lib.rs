use log::info;
pub fn enable_logger() {
    pretty_env_logger::init_timed();
    info!("Init logger!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
