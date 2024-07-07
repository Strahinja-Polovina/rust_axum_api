use crate::constants::repositories_constants::UPDATE_USER_ERROR;
use crate::models::user_model::{CreateUserDTO, GetUsersDTO, UpdateUserDTO, User};
use crate::schema::users::dsl::users;
use crate::schema::users::{email, id};
use crate::services::password_service::hash_password;
use diesel::prelude::*;
use diesel::PgConnection;

pub struct UserRepository;

impl UserRepository {
    pub fn find_all(conn: &mut PgConnection) -> Result<Vec<GetUsersDTO>, diesel::result::Error> {
        users.select((id, email)).load::<GetUsersDTO>(conn)
    }

    pub fn find_one(
        conn: &mut PgConnection,
        user_id: i32,
    ) -> Result<GetUsersDTO, diesel::result::Error> {
        users
            .select((id, email))
            .filter(id.eq(user_id))
            .first::<GetUsersDTO>(conn)
    }

    pub fn find_by_email(
        conn: &mut PgConnection,
        email_to_find: &str,
    ) -> Result<User, diesel::result::Error> {
        users.filter(email.eq(email_to_find)).first::<User>(conn)
    }

    pub fn create_user(
        conn: &mut PgConnection,
        new_user: CreateUserDTO,
    ) -> Result<GetUsersDTO, diesel::result::Error> {
        let hash_password = hash_password(&new_user.password);

        let data_to_insert = CreateUserDTO {
            email: new_user.email.to_lowercase(),
            password: hash_password.unwrap(),
        };
        let inserted_id = diesel::insert_into(users)
            .values(&data_to_insert)
            .returning(id)
            .get_result::<i32>(conn)?;

        users
            .filter(id.eq(inserted_id))
            .select((id, email))
            .first::<GetUsersDTO>(conn)
    }

    pub fn delete_user(conn: &mut PgConnection, target_id: i32) -> QueryResult<usize> {
        diesel::delete(users.find(target_id)).execute(conn)
    }

    pub fn update_user(
        conn: &mut PgConnection,
        target_id: i32,
        updated_data: UpdateUserDTO,
    ) -> Result<GetUsersDTO, diesel::result::Error> {
        diesel::update(users.find(target_id))
            .set(&updated_data)
            .get_result::<User>(conn)
            .expect(UPDATE_USER_ERROR);

        users
            .filter(id.eq(target_id))
            .select((id, email))
            .first::<GetUsersDTO>(conn)
    }
}
