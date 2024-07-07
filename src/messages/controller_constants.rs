// GLOBAL CONTROLLER CONSTANTS

pub const STATUS: &str = "status";
pub const MESSAGE: &str = "message";
pub const DATA: &str = "data";

// AUTH CONTROLLER MESSAGES

pub const LOGIN_SUCCESSFULLY_MESSAGE: &str = "Login successfully";

// AUTH CONTROLLER ERRORS

pub const GENERATING_JWT_ERROR: &str = "Error generating JWT token.";
pub const WRONG_PASSWORD_ERROR: &str = "Wrong password.";



// USER CONTROLLER MESSAGES
pub const USER_FETCH_SUCCESSFUL_MESSAGE: &str = "User fetched successfully.";
pub const USER_CREATED_MESSAGE: &str = "User created successfully";
pub const USER_DELETED_SUCCESSFULLY_MESSAGE: &str = "User deleted successfully";
pub const USER_UPDATED_SUCCESSFULLY_MESSAGE: &str = "User updated successfully";
pub const USER_UPDATED_ERROR_MESSAGE: &str = "User updating error";


// USER CONTROLLER ERRORS
pub const USER_NOT_FOUND_ERROR: &str = "User not found!";
pub const USER_CREATE_BAD_REQUEST_ERROR: &str = "Bad request.";
pub const USER_EMAIL_EXISTS_ERROR: &str = "E-mail is already exist";
pub const INTERNAL_SERVER_ERROR: &str = "Internal server error";
pub const USER_DELETE_ERROR: &str = "Error with deleting user";





