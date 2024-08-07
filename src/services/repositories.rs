use std::collections::HashMap;
use uuid::{Uuid, uuid};
use crate::domain::entities;
use crate::domain::repositories;

pub struct CertificationRepository {
    data: HashMap<Uuid, entities::Certification>
}

impl CertificationRepository {
    pub fn new() -> CertificationRepository {
        let mut data = HashMap::new();
        let uuid = uuid!("14066921-d4d3-42cf-9688-52bf459de23b");

        data.insert(uuid, entities::Certification::new(1, uuid, "ISO-14001".to_string(), entities::ActivationStatus::Enabled));

        CertificationRepository{
            data,
        }
    }
}

impl repositories::Certification for CertificationRepository {
    fn get_certification_by_uuid(self, uuid: Uuid) -> Option<entities::Certification> {
        if self.data.contains_key(&uuid) {
            return Some(self.data.get(&uuid).unwrap().to_owned())
        }
        None
    }
}
