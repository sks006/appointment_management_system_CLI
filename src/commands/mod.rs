pub mod add;
pub mod list;
pub mod delete;

use clap::Subcommand;
use pickledb::PickleDb;

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new soul to the archive
    Add {
        #[arg(short, long)]
        first_name: String,
        #[arg(short, long)]
        last_name: Option<String>,
        #[arg(short, long)]
        pet_name: Option<String>,
        #[arg(short, long)]
        doctor_name: Option<String>,
    },
    /// List all souls
    List,
    /// Banish a soul
    Delete { 
        #[arg(short, long)]
        name: String 
    },
}

pub fn handle(command: Commands, db: &mut PickleDb) {
    match command {
        Commands::Add { first_name, last_name, pet_name, doctor_name } => {
            add::execute(db, first_name, last_name, pet_name, doctor_name);
        }
        Commands::List => {
            list::execute(db);
        }
        Commands::Delete { name } => {
            delete::execute(db, &name);
        }
    }
}