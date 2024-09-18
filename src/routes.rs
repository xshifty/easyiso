#[allow(unused)]

use std::fmt::Debug;
use std::str::FromStr;
use rocket::{response::Redirect, Data, Request};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::form::Form;
use rocket::time::format_description::FormatItem::Optional;
use rocket_dyn_templates::{context, Template};
use uuid::Uuid;
use crate::container::*;

#[derive(Debug,Responder)]
pub enum Response {
    Template(Template),
    Redirect(Redirect),
}

#[get("/")]
pub fn index_page() -> Response {
    let auth = AUTH.lock().unwrap().clone();
    let is_logged_in = auth.is_logged_in;
    let user_opt = auth.user;

    if user_opt.is_some() {
        let user = user_opt.unwrap();
        if user.enabled {
            return Response::Template(Template::render("index", context!{
                is_logged_in,
                full_name: user.full_name,
            }));
        }
    }

    AUTH.lock().unwrap().update(false, None);
    Response::Template(Template::render("index", context!{
        is_logged_in: false,
    }))
}

#[derive(FromForm)]
pub struct LoginInput<'r> {
    email: &'r str,
    password: &'r str,
}

#[get("/login")]
pub fn login_page() -> Response {
    AUTH.lock().unwrap().update(false, None);
    let template = Template::render("login", context!{});
    Response::Template(template)
}

#[get("/component/login")]
pub fn login_component() -> Response {
    AUTH.lock().unwrap().update(false, None);
    let template = Template::render("components/login", context!{});
    Response::Template(template)
}

#[post("/login", data = "<login_input>")]
pub fn login_processor(login_input: Form<LoginInput<'_>>) -> Response {
    let user = UserRepo.get_user_by_login(
        login_input.email.parse().unwrap(),
        login_input.password.parse().unwrap(),
    );

    if !user.clone().enabled {
        return Response::Redirect(Redirect::to(uri!(login_component)))
    }
    AUTH.lock().unwrap().update(true, Some(user));
    Response::Redirect(Redirect::to(uri!(dashboard_component)))
}

#[get("/dashboard")]
pub fn dashboard_page() -> Response {
    if !AUTH.lock().unwrap().is_logged_in {
        return Response::Redirect(Redirect::to(uri!(login_page)))
    }
    let user_opt = AUTH.lock().unwrap().clone().user;

    if user_opt.is_some() {
        let user = user_opt.unwrap();
        return Response::Template(Template::render("dashboard", context!{
            full_name: user.full_name,
        }));
    }
    Response::Redirect(Redirect::to(uri!(login_page)))
}

#[get("/component/dashboard")]
pub fn dashboard_component() -> Response {
    if !AUTH.lock().unwrap().is_logged_in {
        return Response::Redirect(Redirect::to(uri!(login_page)))
    }
    let user_opt = AUTH.lock().unwrap().clone().user;

    if user_opt.is_some() {
        let user = user_opt.unwrap();
        return Response::Template(Template::render("components/dashboard", context!{
            full_name: user.full_name,
        }));
    }
    Response::Redirect(Redirect::to(uri!(login_component)))
}


#[get("/logout")]
pub fn logout_page() -> Response {
    AUTH.lock().unwrap().update(false, None);
    Response::Redirect(Redirect::to(uri!(index_page)))
}

#[get("/component/logout")]
pub fn logout_component() -> Response {
    AUTH.lock().unwrap().update(false, None);
    Response::Redirect(Redirect::to(uri!(login_component)))
}
