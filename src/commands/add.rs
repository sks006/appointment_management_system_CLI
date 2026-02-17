use pickledb::PickleDb;
use crate::engine::model::Person;
use crate::engine::storage;

pub fn execute(
    db: &mut PickleDb, 
    first_name: String, 
    last_name: Option<String>, 
    pet_name: Option<String>, 
    doctor_name: Option<String>
) {
    let person = Person {
        first_name: first_name.clone(),
        last_name,
        pet_name,
        doctor_name,
    };
    
    println!("📖 Writing {} into the Great Chronicle...", person.first_name);
    match storage::save_person(db, person) {
        Ok(_) => println!("✅ Saved successfully."),
        Err(e) => eprintln!("❌ Engine Failure: {:?}", e),
    }
}