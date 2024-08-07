use axum::http::Response;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::types::AppEngine;

#[derive(Clone,Serialize,Deserialize)]
pub enum ActivationStatus {
    Enabled,
    Disabled,
}

impl ActivationStatus {
    fn to_string(self) -> String {
        match self {
            ActivationStatus::Enabled => "Enabled".to_string(),
            ActivationStatus::Disabled => "Disabled".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Certification {
    id: u64,
    uuid: Uuid,
    name: String,
    status: ActivationStatus,
}

impl Certification {
    pub fn new(id: u64, uuid: Uuid, name: String, status: ActivationStatus) -> Certification {
        Certification{ id, uuid, name, status }
    }

    pub fn get_id(self) -> u64 {
        self.id
    }

    pub fn get_uuid(self) -> Uuid {
        self.uuid
    }

    pub fn get_name(self) -> String {
        self.name
    }

    pub fn get_status(self) -> ActivationStatus {
        self.status
    }

    pub fn to_dto(self) -> CertificationDto {
        CertificationDto{
            id: self.id,
            uuid: self.uuid.to_string(),
            name: self.name,
            status: self.status,
            checklists: "".to_string(),
        }
    }
}

#[derive(Clone,Serialize,Deserialize)]
pub struct CertificationDto {
    pub id: u64,
    pub uuid: String,
    pub name: String,
    pub status: ActivationStatus,
    pub checklists: String,
}
