use crate::types;
use crate::auth;

use axum::response::{Response, Redirect, IntoResponse};
use axum_template::RenderHtml;
use axum::Form;
use minijinja::context;
use types::AppEngine;

pub async fn get(engine: AppEngine) -> Response {
    RenderHtml("login.html.j2", engine, context!{}).into_response()
}

pub async fn post(Form(sign_in): Form<types::SignIn>) -> Response {
    match auth::authenticate(sign_in) {
        Ok(_) => Redirect::to("/dashboard").into_response(),
        Err(_) => Redirect::to("/login").into_response(),
    }
}
