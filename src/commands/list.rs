use pickledb::PickleDb;
use crate::engine::storage;

pub fn execute(db: &PickleDb) {
    let history = storage::get_all_persons(db);
    
    if history.is_empty() {
        println!("📭 The Chronicle is empty.");
    } else {
        println!("📜 --- The Great Chronicle ---");
        for (i, p) in history.iter().enumerate() {
            println!(
                "{}. {} {} (Pet: {:?}) (Doc: {:?})", 
                i + 1, 
                p.first_name, 
                p.last_name.as_deref().unwrap_or(""), 
                p.pet_name,
                p.doctor_name
            );
        }
    }
}