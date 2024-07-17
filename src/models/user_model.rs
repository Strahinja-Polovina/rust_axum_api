use crate::schema::users;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub roles: String,
}
#[derive(Queryable, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct GetUsersDTO {
    pub id: i32,
    pub email: String,
    pub roles: String,
}

pub trait GetUserDtoConstructor {
    fn new(id: i32, email: String, roles: String) -> Self;
}

impl GetUserDtoConstructor for GetUsersDTO {
    fn new(id: i32, email: String, roles: String) -> Self {
        GetUsersDTO { id, email, roles }
    }
}

#[derive(Deserialize, Insertable, Clone)]
#[diesel(table_name = users)]
pub struct CreateUserDTO {
    pub email: String,
    pub password: String,
    pub roles: Option<String>,
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUserDTO {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Queryable)]
#[diesel(table_name = users)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Queryable)]
#[diesel(table_name = users)]
pub struct LoginResponseDTO {
    pub token: String,
}
