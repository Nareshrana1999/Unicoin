//! Unicoin CLI - Command Line Interface
//!
//! This binary provides a comprehensive command-line interface for managing
//! Unicoin nodes, wallets, and blockchain operations.

use unicoin::cli::CliApp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the CLI application
    let app = CliApp::new();
    
    // Run the application
    if let Err(e) = app.run().await {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }
    
    Ok(())
}
