// C# - Null reference exceptions are runtime bombs
/* public class UserService
{
    public string GetUserDisplayName(User user)
    {
        // Any of these could throw NullReferenceException
        return user.Profile.DisplayName.ToUpper();
        //     ^^^^^ ^^^^^^^ ^^^^^^^^^^^ ^^^^^^^
        //     Could be null at runtime
    }
    
    // Nullable reference types (C# 8+) help, but nulls can still slip through
    public string GetDisplayName(User? user)
    {
        return user?.Profile?.DisplayName?.ToUpper() ?? "Unknown";
        // This specific line is null-safe thanks to ?. and ??,
        // but NRTs are advisory — the compiler can be overridden with `!`
    }
}
 */

// Rust - Null safety guaranteed at compile time
pub struct UserService;

impl UserService {
    pub fn get_user_display_name(user: &User) -> Option<String> {

        // The compiler will ensure that we handle the None case, preventing null reference errors at runtime , because if age is none, the function will return None and the code will not attempt to access the age field, thus avoiding a null reference error.
        let get_age = user.profile.as_ref()?.age?;
        println!("User age: {} , is active: {}", get_age, user.active);
        // The first part you need ? to access the profile field, and with display_name you don't need ? because it's already an Option, so you can use map to transform it if it's Some, or return None if it's None.
        user.profile
            .as_ref()?
            .display_name
            .as_ref()
            .map(|name| name.to_uppercase())
    }
}
pub struct User {
    pub active: bool,
    pub profile: Option<Profile>,
}
pub struct Profile {
    pub display_name: Option<String>,
    pub age: Option<u32>,
}
