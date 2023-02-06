#![allow(unused)]


pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed
}

pub struct Epic {
    name: String,
    description: String,
    status: Status,
    stories: Vec<String>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
            stories: vec![],
        }
    }
}

pub struct Story {
    name: String,
    description: String,
    status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
        }

    }
}

pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    // TODO: add fields (make sure the fields are public)
    last_item_id: i32,
    epics: Vec<Epic>,
    stories: Vec<Story>,

}