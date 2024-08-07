use axum::response::{Response, Redirect, IntoResponse};
use axum_template::RenderHtml;
use minijinja::context;
use crate::{auth, types::AppEngine};

pub async fn get(engine: AppEngine) -> Response {
    let navbar = true;

    if auth::is_user_authenticated() {
        return RenderHtml("dashboard.html.j2", engine, context! { navbar }).into_response();
    }

    Redirect::to("/login").into_response()
}