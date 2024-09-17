#[allow(unused)]

use std::fmt::Debug;
use rocket::{response::Redirect};
use rocket_dyn_templates::{context, Template};
use crate::container::*;


#[derive(Debug,Responder)]
pub enum Response {
    Template(Template),
    Redirect(Redirect),
}

#[get("/")]
pub fn index_page() -> Response {
    let template = Template::render("index", context!{
        is_logged_in: false,
    });
    Response::Template(template)
}

#[get("/login")]
pub fn login_page() -> Response {
    let template = Template::render("login", context!{});
    Response::Template(template)
}

#[get("/component/login")]
pub fn login_component() -> Response {
    let template = Template::render("components/login", context!{});
    Response::Template(template)
}

#[post("/login")]
pub fn login_processor() -> Response {
    let redirect = Redirect::to(uri!(dashboard_component));
    Response::Redirect(redirect)
}

#[get("/dashboard")]
pub fn dashboard_page() -> Response {
    let user = UserRepo.get_user_by_id(1);

    let template = Template::render("dashboard", context!{
        full_name: user.full_name,
    });
    Response::Template(template)
}

#[get("/component/dashboard")]
pub fn dashboard_component() -> Response {
    let user = UserRepo.get_user_by_id(1);

    let template = Template::render("components/dashboard", context! {
        full_name: user.full_name,
    });
    Response::Template(template)
}
