use serde::Deserialize;
use sha2::{digest::Update, Digest, Sha512};

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

    fn encrypt_password(password: String) -> String {
        let salt = std::env::var("APP_SALT").expect("APP_SALT must be set");
        let mut hasher = Sha512::default();

        hasher.update(format!("{}{}", password, salt));
        hasher.finalize()
    }
}