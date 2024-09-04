use askama::Template;
use axum::{
    Router,
    Form,
    http::StatusCode,
    response::{ Response, Redirect, IntoResponse },
    routing::{ get, post },
};
use crate::auth::{ SignIn, is_user_authenticated, authenticate };
use crate::templates;
use crate::templates::Base;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(show_index))
        .route("/login", post(process_login).get(show_login))
        .route("/dashboard", get(show_dashboard))
}

pub async fn show_index() -> Response {
    let index = templates::Index{
        base: Base{
            navbar: true,
            authenticated: is_user_authenticated(),
        },
    };

    Response::builder()
        .status(StatusCode::OK)
        .body(index.render().unwrap())
        .unwrap()
        .into_response()
}

pub async fn show_login() -> Response {
    let login = templates::Login{
        base: Base{
            navbar: false,
            authenticated: false,
        },
    };

    Response::builder()
        .status(StatusCode::OK)
        .body(login.render().unwrap())
        .unwrap()
        .into_response()
}

pub async fn process_login(Form(sign_in): Form<SignIn>) -> Response {
    match authenticate(sign_in) {
        Ok(_) => Redirect::to("/dashboard").into_response(),
        Err(_) => Redirect::to("/login").into_response(),
    }
}

pub async fn show_dashboard() -> Response {
    if !is_user_authenticated() {
        return Redirect::to("/login").into_response();
    }
    let dashboard = templates::Dashboard{
        base: Base{
            navbar: true,
            authenticated: false,
        },
    };

    Response::builder()
        .status(StatusCode::OK)
        .body(dashboard.render().unwrap())
        .unwrap()
        .into_response()
}

// pub async fn show_certification(engine: AppEngine, Path((cert_uuid, _)): Path<(String, String)>) -> Response {
//     let repo = services::CertificationRepository::new();
//     let cuuid = Uuid::parse_str(&cert_uuid[..]).unwrap();
//
//     match repo.get_certification_by_uuid(cuuid) {
//         Some(c) => {
//             let mut cert_dto = c.to_dto();
//
//             let mut env = Environment::new();
//             env.set_loader(path_loader(&crate::routes::get_jinja_template_path()));
//             cert_dto.checklists = env.get_template("checklists_demo.askama")
//                 .unwrap()
//                 .render(context!{})
//                 .unwrap()
//                 .to_string();
//
//             RenderHtml("certification_details.askama", engine.clone(), context! {
//                 certification => cert_dto,
//             }).into_response()
//         },
//         None => RenderHtml("404.html.j2", engine, context! {}).into_response()
//     }
// }
