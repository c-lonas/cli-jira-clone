#![allow(unused)]

use std::collections::HashMap;


pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed
}

pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<String>,
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
    pub name: String,
    pub description: String,
    pub status: Status,
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
    
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,

}