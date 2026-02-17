use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use super::model::Person;
use crate::errors::RwaError;

/// Initialize the Garage (Open DB)
pub fn load_db() -> Result<PickleDb, RwaError> {
    PickleDb::load(
        "assets.db",
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    )
    .or_else(|_| {
        Ok(PickleDb::new(
            "assets.db",
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        ))
    })
    .map_err(|e: pickledb::error::Error| RwaError::DatabaseLoadFailure(e.to_string()))
}

pub fn save_person(db: &mut PickleDb, person: Person) -> Result<(), RwaError> {
    // 1. Load the Registry (The Scroll)
    let mut history: Vec<Person> = db.get("all_persons").unwrap_or_default();
    
    // 2. Add the new entry
    history.push(person);
    
    // 3. Commit to Disk (Atomic Write)
    db.set("all_persons", &history)
        .map_err(|_| RwaError::WriteFailure("Failed to write to scroll".into()))
}

pub fn get_all_persons(db: &PickleDb) -> Vec<Person> {
    db.get("all_persons").unwrap_or_default()
}

pub fn remove_person(db: &mut PickleDb, name: &str) -> Result<bool, RwaError> {
    let mut history: Vec<Person> = db.get("all_persons").unwrap_or_default();
    let initial_len = history.len();
    
    // The Banishment Logic
    history.retain(|p| p.first_name.to_lowercase() != name.to_lowercase());
    
    if history.len() < initial_len {
        db.set("all_persons", &history)
            .map_err(|_| RwaError::WriteFailure("Failed to update scroll".into()))?;
        Ok(true)
    } else {
        Ok(false)
    }
}