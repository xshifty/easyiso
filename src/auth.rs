use serde::Deserialize;

pub fn authenticate(sign_in: SignIn) -> Result<(), ()> {
    if sign_in.is_equal("test".to_string(), "test".to_string()) {
        return Ok(())
    }
    Err(())
}

pub fn is_user_authenticated() -> bool {
    true
}

#[derive(Deserialize)]
pub struct SignIn {
    username: String,
    password: String,
}

impl SignIn {
    pub fn get_username(self) -> String {
        self.username
    }

    pub fn get_password(self) -> String {
        self.password
    }

    pub fn is_equal(self, username: String, password: String) -> bool {
        username == self.username && password == self.password
    }
}