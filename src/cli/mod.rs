//! Command Line Interface for Unicoin
//!
//! This module provides a comprehensive CLI for managing Unicoin nodes,
//! wallets, and blockchain operations.

use clap::{Parser, Subcommand};
use crate::{config::UnicoinConfig, Result};
use std::path::PathBuf;

pub mod commands;
pub mod wallet_cli;
pub mod node_cli;
pub mod blockchain_cli;

pub use commands::*;
pub use wallet_cli::*;
pub use node_cli::*;
pub use blockchain_cli::*;

/// Unicoin Command Line Interface
#[derive(Parser)]
#[command(name = "unicoin")]
#[command(about = "Unicoin - The Ultimate Secure Decentralized Cryptocurrency")]
#[command(version = crate::VERSION)]
#[command(long_about = "Unicoin is the most advanced cryptocurrency with AI-powered features, 
complete DeFi ecosystem, NFT platform, and quantum-resistant security.")]
pub struct Cli {
    /// Configuration file path
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Data directory path
    #[arg(short, long, value_name = "DIR")]
    pub data_dir: Option<PathBuf>,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,

    /// Network to connect to
    #[arg(short, long, default_value = "mainnet")]
    pub network: String,

    #[command(subcommand)]
    pub command: Commands,
}

/// Main CLI commands
#[derive(Subcommand)]
pub enum Commands {
    /// Node management commands
    Node {
        #[command(subcommand)]
        command: NodeCommands,
    },
    /// Wallet management commands
    Wallet {
        #[command(subcommand)]
        command: WalletCommands,
    },
    /// Blockchain operations
    Blockchain {
        #[command(subcommand)]
        command: BlockchainCommands,
    },
    /// Configuration management
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    /// Development and testing tools
    Dev {
        #[command(subcommand)]
        command: DevCommands,
    },
    /// Network diagnostics and monitoring
    Network {
        #[command(subcommand)]
        command: NetworkCommands,
    },
    /// Start the Unicoin node
    Start {
        /// Enable mining
        #[arg(long)]
        mine: bool,
        /// Enable API server
        #[arg(long, default_value = "true")]
        api: bool,
        /// Enable WebSocket API
        #[arg(long, default_value = "true")]
        websocket: bool,
        /// Enable GraphQL API
        #[arg(long, default_value = "true")]
        graphql: bool,
    },
}

/// Node management commands
#[derive(Subcommand)]
pub enum NodeCommands {
    /// Show node information
    Info,
    /// Show node status
    Status,
    /// Show node statistics
    Stats,
    /// Restart the node
    Restart,
    /// Stop the node
    Stop,
    /// Show node logs
    Logs {
        /// Number of lines to show
        #[arg(short, long, default_value = "100")]
        lines: usize,
        /// Follow logs in real-time
        #[arg(short, long)]
        follow: bool,
    },
}

/// Wallet management commands
#[derive(Subcommand)]
pub enum WalletCommands {
    /// Create a new wallet
    Create {
        /// Wallet name
        name: String,
        /// Encrypt wallet with password
        #[arg(short, long)]
        encrypt: bool,
        /// Generate seed phrase
        #[arg(long, default_value = "true")]
        seed: bool,
    },
    /// Import wallet from seed phrase
    Import {
        /// Wallet name
        name: String,
        /// Seed phrase (12 or 24 words)
        seed: String,
        /// Encrypt wallet
        #[arg(short, long)]
        encrypt: bool,
    },
    /// List all wallets
    List,
    /// Show wallet information
    Info {
        /// Wallet name
        name: String,
    },
    /// Generate new address
    Address {
        /// Wallet name
        name: String,
        /// Address type (p2pkh, p2sh, bech32)
        #[arg(short, long, default_value = "bech32")]
        address_type: String,
        /// Label for the address
        #[arg(short, long)]
        label: Option<String>,
    },
    /// Show wallet balance
    Balance {
        /// Wallet name
        name: String,
        /// Include unconfirmed transactions
        #[arg(short, long)]
        unconfirmed: bool,
    },
    /// Send transaction
    Send {
        /// Wallet name
        name: String,
        /// Recipient address
        to: String,
        /// Amount to send (in UNI)
        amount: f64,
        /// Transaction fee (in UNI)
        #[arg(short, long)]
        fee: Option<f64>,
        /// Message to include
        #[arg(short, long)]
        message: Option<String>,
    },
    /// Show transaction history
    History {
        /// Wallet name
        name: String,
        /// Number of transactions to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
}

/// Blockchain operations
#[derive(Subcommand)]
pub enum BlockchainCommands {
    /// Show blockchain information
    Info,
    /// Show latest blocks
    Blocks {
        /// Number of blocks to show
        #[arg(short, long, default_value = "10")]
        count: usize,
    },
    /// Show block by height or hash
    Block {
        /// Block height or hash
        identifier: String,
    },
    /// Show transaction by hash
    Tx {
        /// Transaction hash
        hash: String,
    },
    /// Show blockchain statistics
    Stats,
    /// Verify blockchain integrity
    Verify,
    /// Rebuild blockchain index
    Rebuild,
}

/// Configuration management commands
#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Show current configuration
    Show,
    /// Edit configuration
    Edit,
    /// Reset to default configuration
    Reset,
    /// Validate configuration
    Validate,
    /// Generate new configuration
    Generate {
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Configuration template
        #[arg(short, long, default_value = "mainnet")]
        template: String,
    },
}

/// Development and testing commands
#[derive(Subcommand)]
pub enum DevCommands {
    /// Run tests
    Test {
        /// Test pattern to match
        pattern: Option<String>,
        /// Run in release mode
        #[arg(long)]
        release: bool,
    },
    /// Run benchmarks
    Bench {
        /// Benchmark pattern to match
        pattern: Option<String>,
    },
    /// Generate test data
    GenTestData {
        /// Number of transactions to generate
        #[arg(short, long, default_value = "100")]
        transactions: usize,
        /// Number of blocks to generate
        #[arg(short, long, default_value = "10")]
        blocks: usize,
    },
    /// Simulate network
    SimNetwork {
        /// Number of nodes to simulate
        #[arg(short, long, default_value = "5")]
        nodes: usize,
        /// Duration of simulation (seconds)
        #[arg(short, long, default_value = "60")]
        duration: u64,
    },
}

/// Network diagnostics commands
#[derive(Subcommand)]
pub enum NetworkCommands {
    /// Show network information
    Info,
    /// Show connected peers
    Peers,
    /// Show network statistics
    Stats,
    /// Ping a peer
    Ping {
        /// Peer address
        address: String,
    },
    /// Connect to a peer
    Connect {
        /// Peer address
        address: String,
    },
    /// Disconnect from a peer
    Disconnect {
        /// Peer ID
        peer_id: String,
    },
    /// Show network topology
    Topology,
}

/// CLI application runner
pub struct CliApp {
    cli: Cli,
    config: UnicoinConfig,
}

impl CliApp {
    /// Create a new CLI application
    pub fn new() -> Self {
        let cli = Cli::parse();
        let config = Self::load_config(&cli);
        
        Self { cli, config }
    }

    /// Load configuration from file or create default
    fn load_config(cli: &Cli) -> UnicoinConfig {
        // Try to load from specified config file
        if let Some(config_path) = &cli.config {
            if let Ok(config) = UnicoinConfig::load_from_file(config_path) {
                return config;
            }
        }

        // Try to load from default location
        if let Ok(config) = UnicoinConfig::load_from_file("config.toml") {
            return config;
        }

        // Create default configuration
        UnicoinConfig::new()
    }

    /// Run the CLI application
    pub async fn run(self) -> Result<()> {
        // Setup logging
        self.setup_logging()?;

        // Execute the command
        match self.cli.command {
            Commands::Node { command } => self.handle_node_command(command).await,
            Commands::Wallet { command } => self.handle_wallet_command(command).await,
            Commands::Blockchain { command } => self.handle_blockchain_command(command).await,
            Commands::Config { command } => self.handle_config_command(command).await,
            Commands::Dev { command } => self.handle_dev_command(command).await,
            Commands::Network { command } => self.handle_network_command(command).await,
            Commands::Start { mine, api, websocket, graphql } => {
                self.start_node(mine, api, websocket, graphql).await
            }
        }
    }

    /// Setup logging based on CLI options
    fn setup_logging(&self) -> Result<()> {
        let log_level = if self.cli.verbose {
            "debug"
        } else {
            "info"
        };

        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_ansi(!self.cli.no_color)
            .init();

        Ok(())
    }

    /// Handle node commands
    async fn handle_node_command(&self, command: NodeCommands) -> Result<()> {
        match command {
            NodeCommands::Info => self.show_node_info().await,
            NodeCommands::Status => self.show_node_status().await,
            NodeCommands::Stats => self.show_node_stats().await,
            NodeCommands::Restart => self.restart_node().await,
            NodeCommands::Stop => self.stop_node().await,
            NodeCommands::Logs { lines, follow } => self.show_node_logs(lines, follow).await,
        }
    }

    /// Handle wallet commands
    async fn handle_wallet_command(&self, command: WalletCommands) -> Result<()> {
        match command {
            WalletCommands::Create { name, encrypt, seed } => {
                self.create_wallet(name, encrypt, seed).await
            }
            WalletCommands::Import { name, seed, encrypt } => {
                self.import_wallet(name, seed, encrypt).await
            }
            WalletCommands::List => self.list_wallets().await,
            WalletCommands::Info { name } => self.show_wallet_info(name).await,
            WalletCommands::Address { name, address_type, label } => {
                self.generate_address(name, address_type, label).await
            }
            WalletCommands::Balance { name, unconfirmed } => {
                self.show_wallet_balance(name, unconfirmed).await
            }
            WalletCommands::Send { name, to, amount, fee, message } => {
                self.send_transaction(name, to, amount, fee, message).await
            }
            WalletCommands::History { name, limit } => {
                self.show_transaction_history(name, limit).await
            }
        }
    }

    /// Handle blockchain commands
    async fn handle_blockchain_command(&self, command: BlockchainCommands) -> Result<()> {
        match command {
            BlockchainCommands::Info => self.show_blockchain_info().await,
            BlockchainCommands::Blocks { count } => self.show_latest_blocks(count).await,
            BlockchainCommands::Block { identifier } => self.show_block(identifier).await,
            BlockchainCommands::Tx { hash } => self.show_transaction(hash).await,
            BlockchainCommands::Stats => self.show_blockchain_stats().await,
            BlockchainCommands::Verify => self.verify_blockchain().await,
            BlockchainCommands::Rebuild => self.rebuild_blockchain_index().await,
        }
    }

    /// Handle configuration commands
    async fn handle_config_command(&self, command: ConfigCommands) -> Result<()> {
        match command {
            ConfigCommands::Show => self.show_config().await,
            ConfigCommands::Edit => self.edit_config().await,
            ConfigCommands::Reset => self.reset_config().await,
            ConfigCommands::Validate => self.validate_config().await,
            ConfigCommands::Generate { output, template } => {
                self.generate_config(output, template).await
            }
        }
    }

    /// Handle development commands
    async fn handle_dev_command(&self, command: DevCommands) -> Result<()> {
        match command {
            DevCommands::Test { pattern, release } => self.run_tests(pattern, release).await,
            DevCommands::Bench { pattern } => self.run_benchmarks(pattern).await,
            DevCommands::GenTestData { transactions, blocks } => {
                self.generate_test_data(transactions, blocks).await
            }
            DevCommands::SimNetwork { nodes, duration } => {
                self.simulate_network(nodes, duration).await
            }
        }
    }

    /// Handle network commands
    async fn handle_network_command(&self, command: NetworkCommands) -> Result<()> {
        match command {
            NetworkCommands::Info => self.show_network_info().await,
            NetworkCommands::Peers => self.show_peers().await,
            NetworkCommands::Stats => self.show_network_stats().await,
            NetworkCommands::Ping { address } => self.ping_peer(address).await,
            NetworkCommands::Connect { address } => self.connect_peer(address).await,
            NetworkCommands::Disconnect { peer_id } => self.disconnect_peer(peer_id).await,
            NetworkCommands::Topology => self.show_network_topology().await,
        }
    }

    /// Start the Unicoin node
    async fn start_node(&self, mine: bool, api: bool, websocket: bool, graphql: bool) -> Result<()> {
        println!("🚀 Starting Unicoin Node v{}", crate::VERSION);
        println!("📊 Network: {}", self.cli.network);
        println!("⛏️  Mining: {}", if mine { "Enabled" } else { "Disabled" });
        println!("🌐 API Server: {}", if api { "Enabled" } else { "Disabled" });
        println!("🔌 WebSocket API: {}", if websocket { "Enabled" } else { "Disabled" });
        println!("📈 GraphQL API: {}", if graphql { "Enabled" } else { "Disabled" });
        
        // TODO: Implement actual node startup
        println!("✅ Node started successfully!");
        
        Ok(())
    }
}
