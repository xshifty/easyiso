use std::fmt::Debug;
use axum::{extract::Path, response::Response};
use axum::response::IntoResponse;
use axum_template::RenderHtml;
use minijinja::{context, Environment, path_loader};
use uuid::{Uuid};
use crate::{
    domain::repositories::Certification,
    services::repositories,
    types::AppEngine,
};

pub async fn get(engine: AppEngine, Path((cert_uuid, _)): Path<(String, String)>) -> Response {
    let repo = repositories::CertificationRepository::new();
    let cuuid = Uuid::parse_str(&cert_uuid[..]).unwrap();

    match repo.get_certification_by_uuid(cuuid) {
        Some(c) => {
            let mut cert_dto = c.to_dto();

            let mut env = Environment::new();
            env.set_loader(path_loader(&crate::routes::get_jinja_template_path()));
            cert_dto.checklists = env.get_template("checklists_demo.html.j2")
                .unwrap()
                .render(context!{})
                .unwrap()
                .to_string();

            RenderHtml("certification_details.html.j2", engine.clone(), context! {
                certification => cert_dto,
            }).into_response()
        },
        None => RenderHtml("404.html.j2", engine, context! {}).into_response()
    }
}

