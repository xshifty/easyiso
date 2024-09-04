use askama::Template;

pub struct Base {
    pub navbar: bool,
    pub authenticated: bool,
}

#[derive(Template)]
#[template(path = "index.j2")]
pub struct Index {
    pub base: Base,
}

#[derive(Template)]
#[template(path = "login.j2")]
pub struct Login {
    pub base: Base,
}

#[derive(Template)]
#[template(path = "dashboard.j2")]
pub struct Dashboard {
    pub base: Base,
}
