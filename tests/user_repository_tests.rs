#[cfg(test)]
mod tests {
    use axumapi::config::db::establish_connection;
    use axumapi::models::user_model::{CreateUserDTO, GetUsersDTO, UpdateUserDTO};
    use axumapi::repositories::user_repository::UserRepository;
    use diesel::result::Error;
    use diesel::PgConnection;
    use dotenv::dotenv;
    use std::thread::sleep;
    use std::time::Duration;
    use diesel::r2d2::{ConnectionManager, PooledConnection};

    fn setup() -> PooledConnection<ConnectionManager<PgConnection>> {
        dotenv().ok();
        establish_connection()
    }

    fn create_unique_test_user(conn: &mut PgConnection, suffix: &str) -> Result<GetUsersDTO, Error> {
        let user = CreateUserDTO {
            email: format!("test{}@test.com", suffix),
            password: "test_password".to_string(),
            roles: None,
        };

        UserRepository::create_user(conn, user)
    }

    #[test]
    fn user_repository_create_user_test() {
        let mut conn = setup();
        let timestamp = chrono::Utc::now().timestamp_micros();
        let user = create_unique_test_user(&mut conn, &timestamp.to_string())
            .expect("Failed to create test user");

        assert_eq!(user.email, format!("test{}@test.com", timestamp));

        let _ = UserRepository::delete_user(&mut conn, user.id);
    }

    #[test]
    fn user_repository_delete_user_test() {
        let mut conn = setup();
        let timestamp = chrono::Utc::now().timestamp_micros();
        let user = create_unique_test_user(&mut conn, &timestamp.to_string())
            .expect("Failed to create test user");

        let _ = UserRepository::delete_user(&mut conn, user.id);

        let find_user_result = UserRepository::find_one(&mut conn, user.id);
        assert!(find_user_result.is_err());
    }

    #[test]
    fn user_repository_find_by_email_test() {
        let mut conn = setup();
        let timestamp = chrono::Utc::now().timestamp_micros();
        let user = create_unique_test_user(&mut conn, &timestamp.to_string())
            .expect("Failed to create test user");

        let user_found = UserRepository::find_by_email(&mut conn, &user.email)
            .expect("Failed to find user by email");

        assert_eq!(user.email, user_found.email);

        let _ = UserRepository::delete_user(&mut conn, user.id);
    }

    #[test]
    fn user_repository_find_by_id_test() {
        let mut conn = setup();
        let timestamp = chrono::Utc::now().timestamp_micros();
        let user = create_unique_test_user(&mut conn, &timestamp.to_string())
            .expect("Failed to create test user");

        let user_found = UserRepository::find_one(&mut conn, user.id)
            .expect("Failed to find user by ID");

        assert_eq!(user.email, user_found.email);

        let _ = UserRepository::delete_user(&mut conn, user.id);
    }

    #[test]
    fn user_repository_find_all_tests() {
        let mut conn = setup();
        let initial_users = UserRepository::find_all(&mut conn)
            .expect("Failed to fetch initial users");

        let timestamp = chrono::Utc::now().timestamp_micros();
        let user = create_unique_test_user(&mut conn, &timestamp.to_string())
            .expect("Failed to create test user");

        sleep(Duration::from_millis(1));

        let timestamp2 = chrono::Utc::now().timestamp_micros();
        let user1 = create_unique_test_user(&mut conn, &timestamp2.to_string())
            .expect("Failed to create test user");

        sleep(Duration::from_millis(1));

        let timestamp3 = chrono::Utc::now().timestamp_micros();
        let user2 = create_unique_test_user(&mut conn, &timestamp3.to_string())
            .expect("Failed to create test user");

        let users = UserRepository::find_all(&mut conn)
            .expect("Failed to fetch all users");

        assert_eq!(initial_users.len() + 3, users.len());

        let _ = UserRepository::delete_user(&mut conn, user.id);
        let _ = UserRepository::delete_user(&mut conn, user1.id);
        let _ = UserRepository::delete_user(&mut conn, user2.id);
    }

    #[test]
    fn user_repository_update_user_test() {
        let mut conn = setup();
        let timestamp = chrono::Utc::now().timestamp_micros();
        let user = create_unique_test_user(&mut conn, &timestamp.to_string())
            .expect("Failed to create test user");

        let update_user_data = UpdateUserDTO {
            email: "test2@test.com".to_string(),
            password: "test".to_string(),
        };

        let updated_user = UserRepository::update_user(&mut conn, user.id, update_user_data)
            .expect("Failed to update user");

        assert_eq!(&updated_user.email, "test2@test.com");

        let _ = UserRepository::delete_user(&mut conn, updated_user.id);
    }
}
