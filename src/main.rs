use clap::Parser;
use rust_task_manager::commands; // Uses lib.rs exports
use rust_task_manager::engine::storage;

#[derive(Parser)]
#[command(name = "RWA_Manager", version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: commands::Commands,
}

fn main() {
    // 1. Parse Inputs (The Dashboard)
    let cli = Cli::parse();
    
    // 2. Start Engine (The Storage)
    let mut db = storage::load_db().expect("CRITICAL: Failed to load Garage DB");

    // 3. Drive (Dispatch Command)
    commands::handle(cli.command, &mut db);
}