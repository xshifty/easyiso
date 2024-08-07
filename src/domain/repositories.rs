use uuid::Uuid;
use crate::domain::entities;

pub trait Certification {
    fn get_certification_by_uuid(self, uuid: Uuid) -> Option<entities::Certification>;
}
