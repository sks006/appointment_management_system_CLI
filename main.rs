use clap::{Arg, ArgGroup, command, ArgMatches};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

fn main() {
    let matches = command!()
        .group(
            ArgGroup::new("person-register")
                .arg("firstName")
                .arg("lastName"),
        )
        .arg(
            Arg::new("firstName")
                .help("First name")
                .group("person-register")
                .short('f')
                .long("first-name")
                .aliases(&["firstName", "first_name"])
                .required(true),
        )
        .arg(
            Arg::new("lastName")
                .short('l')
                .long("last-name")
                .aliases(&["lastName", "last_name"]),
        )
        .group(ArgGroup::new("pat-name")
            .arg("firstName")
            .arg("pet-name"))
        .arg(Arg::new("petName")
            .short('p')
            .long("pet-name")
            .aliases(&["petName", "pet_name"]))     
        .get_matches();

    // Process and save the data
    save_to_pickledb(&matches);
}

fn save_to_pickledb(matches: &ArgMatches) {
    // Create or open a PickleDB database
    let mut db = PickleDb::new(
        "mydata.db", 
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json
    );
    
    // Extract values from matches
    if let Some(first_name) = matches.get_one::<String>("firstName") {
        // Save first name to database
        db.set("first_name", first_name)
            .expect("Failed to save first name");
        println!("Saved first name: {}", first_name);
    }
    
    if let Some(last_name) = matches.get_one::<String>("lastName") {
        db.set("last_name", last_name)
            .expect("Failed to save last name");
        println!("Saved last name: {}", last_name);
    }
    
    if let Some(pet_name) = matches.get_one::<String>("petName") {
        db.set("pet_name", pet_name)
            .expect("Failed to save pet name");
        println!("Saved pet name: {}", pet_name);
    }
    
    // You can also save more complex structures
    // For example, create a struct and serialize it
    #[derive(serde::Serialize, serde::Deserialize)]
    struct Person {
        first_name: String,
        last_name: Option<String>,
        pet_name: Option<String>,
    }
    
    let person = Person {
        first_name: matches.get_one::<String>("firstName").unwrap().clone(),
        last_name: matches.get_one::<String>("lastName").map(|s| s.clone()),
        pet_name: matches.get_one::<String>("petName").map(|s| s.clone()),
    };
    
    // Save the whole person object
    db.set("current_person", &person)
        .expect("Failed to save person data");
    
    // You can also create a list/array of persons
    // First, get existing list or create new one
    let mut persons: Vec<Person> = db.get("all_persons").unwrap_or_else(|_| Vec::new());
    persons.push(person);
    db.set("all_persons", &persons)
        .expect("Failed to save persons list");
    
    println!("Data saved successfully to mydata.db");
    
    // Example: Read back the data to verify
    if let Ok(saved_name) = db.get::<String>("first_name") {
        println!("Verified: first_name in DB = {}", saved_name);
    }
}

// Optional: Add a function to retrieve data
fn read_from_pickledb() {
    let db = PickleDb::load(
        "mydata.db",
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json
    ).expect("Failed to load database");
    
    if let Ok(first_name) = db.get::<String>("first_name") {
        println!("Retrieved first name: {}", first_name);
    }
    
    if let Ok(person) = db.get::<Person>("current_person") {
        println!("Retrieved person: {:?}", person);  // You'll need to derive Debug
    }
}