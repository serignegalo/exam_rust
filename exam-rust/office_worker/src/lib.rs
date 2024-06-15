// use crate::OfficeWorker::*;

#[derive(Debug, PartialEq, Eq)]
pub struct OfficeWorker {
    pub name: String,
    pub age: u32,
    pub role: WorkerRole,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorkerRole {
    Admin,
    User,
    Guest,
}

impl From<&str> for OfficeWorker {
    fn from(str: &str) -> OfficeWorker {
        let str_split: Vec<&str> = str.split(',').collect();
        let name = str_split[0].to_string();
        let age = str_split[1].parse::<u32>().unwrap();
        let role = WorkerRole::from(str_split[2]);

        OfficeWorker { name, age, role }
    }
}

impl From<&str> for WorkerRole {
    fn from(str: &str) -> WorkerRole {
        match str {
            "admin" => WorkerRole::Admin,
            "user" => WorkerRole::User,
            "guest" => WorkerRole::Guest,
            _ => todo!(),
        }
    }
}
