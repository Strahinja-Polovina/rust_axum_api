mod tests {
    use axumapi::config::db::get_connection;
    use diesel::r2d2::R2D2Connection;

    #[test]
    fn db_connection_test() {
        dotenv::dotenv().expect("Cant find env file.");
        let mut result = get_connection().unwrap();

        let is_broken = result.is_broken();

        assert_eq!(is_broken, false);
    }
}
