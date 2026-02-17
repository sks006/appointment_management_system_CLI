use pickledb::PickleDb;
use crate::engine::storage;

pub fn execute(db: &mut PickleDb, name: &str) {
    match storage::remove_person(db, name) {
        Ok(true) => println!("🔥 {} has been removed from the Chronicle.", name),
        Ok(false) => println!("❓ No one named '{}' was found.", name),
        Err(e) => eprintln!("❌ Write Error: {:?}", e),
    }
}