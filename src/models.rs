#[allow(unused)]

use chrono::prelude::*;
use diesel::prelude::*;
use serde::Serialize;
use crate::schema::*;

#[derive(Debug, Clone, Queryable, Serialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub enabled: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, PartialEq, Serialize)]
#[diesel(table_name = certifications)]
pub struct Certification {
    pub id: i32,
    pub name: String,
    pub enabled: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Associations, PartialEq, Serialize)]
#[diesel(table_name = checklists)]
#[diesel(belongs_to(Certification))]
pub struct Checklist {
    pub id: i32,
    pub certification_id: i32,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize)]
pub struct CertificationWithChecklist {
    pub id: i32,
    pub name: String,
    pub enabled: bool,
    pub created_at: NaiveDateTime,
    pub checklists: Vec<Checklist>,
}
