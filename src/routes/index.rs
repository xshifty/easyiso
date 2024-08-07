use crate::types;
use crate::auth;
use axum::response::{Response, IntoResponse};
use axum_template::RenderHtml;
use minijinja::context;

pub async fn get(engine: types::AppEngine) -> Response {
    let navbar = true;
    let authenticated = auth::is_user_authenticated();

    RenderHtml("index.html.j2", engine, context! { navbar, authenticated }).into_response()
}
