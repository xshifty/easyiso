use std::path::PathBuf;
use axum::Router;
use axum::routing::{get, post};
use axum_template::engine::Engine;
use minijinja::{Environment, path_loader};
use minijinja_autoreload::AutoReloader;
use crate::types::AppState;

pub mod index;
pub mod login;
pub mod dashboard;
pub mod certification;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(index::get))
        .route("/login", post(login::post).get(login::get))
        .route("/dashboard", get(dashboard::get))
        .route("/certifications/:uuid/:short_name", get(certification::get))

        .with_state(AppState {
            engine: Engine::from(get_jinja_auto_reloader()),
        })
}

fn get_jinja_auto_reloader() -> AutoReloader {
    AutoReloader::new(move |notifier| {
        let template_path = get_jinja_template_path();
        let mut env = Environment::new();
        env.set_loader(path_loader(&template_path));
        notifier.set_fast_reload(true);
        notifier.watch_path(&template_path, true);

        Ok(env)
    })
}

pub fn get_jinja_template_path() -> PathBuf {
    PathBuf::from(".").join("templates")
}