// C# - Exceptions can be thrown from anywhere
/*
public async Task<User> GetUserAsync(int userId)
 {
    // Each of these might throw different exceptions
    var user = await userRepository.GetAsync(userId);        // SqlException
    var permissions = await permissionService.GetAsync(user); // HttpRequestException
    var preferences = await preferenceService.GetAsync(user); // TimeoutException

    return new User(user, permissions, preferences);
    // Caller has no idea what exceptions to expect
} */

use std::error::Error;

// Rust - No hidden exceptions, all errors are explicit
// The # [derive(Debug)] attribute allows us to print the UserError enum using the {:?} formatter in println!
#[derive(Debug)]
pub enum UserError {
    DatabaseError(String),
    NetworkError(String),
    Timeout,
}
#[derive(Debug)]
pub struct UserData {
    pub user: User,
    pub permissions: Vec<String>,
    pub preferences: String,
}
pub struct HiddenExeceptions {}
impl HiddenExeceptions {
    pub async fn get_user_data(user_id: i32) -> Result<UserData, UserError> {
        let user = UserRepository::get(user_id)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        let permissions = PermissionService::get(&user)
            .await
            .map_err(|e| UserError::NetworkError(e.to_string()))?;

        let preferences = PreferenceService::get(&user)
            .await
            .map_err(|_| UserError::Timeout)?;

        Ok(UserData {
            user,
            permissions,
            preferences,
        })
    }
}

#[derive(Debug)]
pub struct User {
    pub user_id: i32,
    pub permissions: Vec<String>,
}

struct UserRepository {}
impl UserRepository {
    async fn get(user_id: i32) -> Result<User, Box<dyn Error>> {
        // Simulate a database error
        let first_user: User = User {
            user_id: 1,
            permissions: vec!["read".to_string()],
        };
        let second_user: User = User {
            user_id: 2,
            permissions: vec!["write".to_string()],
        };

        let list_users = Vec::<User>::from([first_user, second_user]);

        list_users
            .into_iter()
            .find(|user| user.user_id == user_id)
            .ok_or("User not found".into())
    }
}

struct PermissionService {}
impl PermissionService {
    async fn get(user: &User) -> Result<Vec<String>, Box<dyn Error>> {
        Ok(user.permissions.clone())
    }
}

struct PreferenceService {}
impl PreferenceService {
    async fn get(user: &User) -> Result<String, Box<dyn Error>> {
        match user.user_id {
            1 => Ok("dark mode".to_string()),
            2 => Ok("light mode".to_string()),
            _ => Err("Preferences not found".into()),
        }
    }
}
