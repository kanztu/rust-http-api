use crate::{dto::user_dto::CreateUser, services::user};

#[test]
fn test_create_user() -> Result<(), String> {
    let s = user::User::new();
    let res = s.create_user(CreateUser {
        username: String::from("test"),
    });
    match res {
        Ok(user) => {
            assert_eq!(user.username, String::from("test"));
            Ok(())
        }
        Err(error) => Err(error),
    }
}
