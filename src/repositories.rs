use diesel::prelude::*;
use rocket::futures::StreamExt;
use crate::container::*;
use crate::models::{Certification, CertificationWithChecklist, Checklist, User};
use crate::schema;

#[derive(Debug,Clone,Copy)]
pub struct UserRepository {
}

impl UserRepository {
    pub fn get_user_by_id(self, id_param: i32) -> User {
        use schema::users;

        let mut binding = POOL.get();
        let conn = binding.as_mut().unwrap();

        users::table
            .filter(users::id.eq(id_param.clone()))
            .first::<User>(conn)
            .expect(format!("User #{} not found", id_param.clone()).as_str())
    }

    pub fn get_user_by_login(self, email_param: String, passwd_param: String) -> User {
        use schema::users;

        let mut binding = POOL.get();
        let conn = binding.as_mut().unwrap();

        users::table
            .filter(users::email.eq(email_param.clone()))
            .filter(users::password.eq(passwd_param.clone()))
            .first::<User>(conn)
            .expect(format!("User {} not found", email_param.clone()).as_str())
    }
}

#[derive(Debug,Clone,Copy)]
pub struct CertificationRepository {
}

impl CertificationRepository {
    pub fn get_certifications_with_checklists(self) -> Vec<CertificationWithChecklist> {
        use schema::certifications;

        let mut binding = POOL.get();
        let conn = binding.as_mut().unwrap();

        let enabled_certifications = certifications::table
            .filter(certifications::enabled.eq(true))
            .select(Certification::as_select())
            .load::<Certification>(conn)
            .expect("error while fetching enabled certifications");

        let enabled_checklists = Checklist::belonging_to(&enabled_certifications)
            .select(Checklist::as_select())
            .load::<Checklist>(conn)
            .expect("error while fetching enabled checklists for certifications");

        enabled_checklists
            .clone()
            .grouped_by(&enabled_certifications)
            .into_iter()
            .zip(enabled_certifications)
            .map(|(checks, cert)| {
                CertificationWithChecklist{
                    id: cert.id,
                    name: cert.name,
                    enabled: cert.enabled,
                    created_at: cert.created_at,
                    checklists: checks,
                }
            })
            .collect::<Vec<(CertificationWithChecklist)>>()
    }
}
