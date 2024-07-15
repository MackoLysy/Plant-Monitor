use log::info;
pub fn enable_logger() {
    pretty_env_logger::init_timed();
    // pretty_env_logger::formatted_builder()
    //     .format(|buf, record| {
    //         writeln!(
    //             buf,
    //             "{} [{}] - {}",
    //             Local::now().format("%Y-%m-%dT%H:%M:%S"),
    //             record.level(),
    //             record.args()
    //         )
    //     })
    //     .init();
    info!("Init logger!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
