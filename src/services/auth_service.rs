use crate::constants::services_constants::JWT_GENERATE_ERROR;
use crate::models::user_model::GetUsersDTO;
use crate::services::token_service::generate_jwt;

pub fn login(user: GetUsersDTO) -> String {
    let token = generate_jwt(user);
    token.unwrap_or_else(|_| String::from(JWT_GENERATE_ERROR))
}
