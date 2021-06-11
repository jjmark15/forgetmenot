use std::collections::HashMap;

pub(crate) use checklist_service::*;
pub(crate) use error::*;

mod checklist_service;
mod error;

pub(crate) struct Checklist {
    entries: HashMap<String, bool>,
}

impl Checklist {
    pub(crate) fn new(entries: HashMap<String, bool>) -> Self {
        Checklist { entries }
    }

    pub(crate) fn entries(&self) -> &HashMap<String, bool> {
        &self.entries
    }
}

impl From<HashMap<String, bool>> for Checklist {
    fn from(entries: HashMap<String, bool>) -> Self {
        Checklist::new(entries)
    }
}
