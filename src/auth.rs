use crate::types;

pub fn authenticate(sign_in: types::SignIn) -> Result<(), ()> {
    if sign_in.is_equal("test".to_string(), "test".to_string()) {
        return Ok(())
    }
    Err(())
}

pub fn is_user_authenticated() -> bool {
    true
}
