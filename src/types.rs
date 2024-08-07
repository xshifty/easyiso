use axum::extract::FromRef;
use axum_template::engine::Engine;
use minijinja_autoreload::AutoReloader;
use serde::Deserialize;

pub type AppEngine = Engine<AutoReloader>;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub engine: AppEngine,
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
