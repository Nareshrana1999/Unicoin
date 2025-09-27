# Unicoin Build Guide

This guide provides detailed instructions for building Unicoin from source.

## Prerequisites

### System Requirements

- **Operating System**: Windows 10/11, macOS 10.15+, or Linux (Ubuntu 20.04+)
- **RAM**: Minimum 8GB, Recommended 16GB+
- **Storage**: Minimum 50GB free space
- **CPU**: Multi-core processor (4+ cores recommended)

### Software Dependencies

#### Rust (Required)
```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add required components
rustup component add rustfmt clippy

# Verify installation
rustc --version
cargo --version
```

#### Node.js (For Applications)
```bash
# Install Node.js 18+ using nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18
nvm use 18

# Verify installation
node --version
npm --version
```

#### Python (For Tools and Testing)
```bash
# Install Python 3.9+
# Ubuntu/Debian:
sudo apt update
sudo apt install python3 python3-pip python3-venv

# macOS:
brew install python@3.9

# Windows:
# Download from https://www.python.org/downloads/

# Verify installation
python3 --version
pip3 --version
```

#### Additional Dependencies

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev libudev-dev
```

**macOS:**
```bash
brew install pkg-config openssl
```

**Windows:**
- Install Visual Studio Build Tools
- Install Git for Windows

## Building from Source

### 1. Clone the Repository

```bash
git clone https://github.com/unicoin/unicoin.git
cd unicoin
```

### 2. Build Core Blockchain

```bash
# Build in release mode (optimized)
cargo build --release

# Build in debug mode (faster compilation)
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### 3. Build Applications

```bash
cd apps

# Install dependencies
npm install

# Build applications
npm run build

# Run tests
npm test
```

### 4. Build Tools

```bash
cd tools

# Create virtual environment
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Run tools
python -m unicoin_tools --help
```

## Configuration

### 1. Create Configuration File

```bash
# Copy example configuration
cp config/unicoin.example.toml ~/.unicoin/config.toml
```

### 2. Edit Configuration

```toml
# ~/.unicoin/config.toml
[network]
# Network port
port = 30303

# Bootstrap nodes
bootstrap_nodes = [
    "/ip4/127.0.0.1/tcp/30303/p2p/12D3KooW..."
]

[consensus]
# Block time in seconds
block_time = 10

# Epoch length in blocks
epoch_length = 100

# Minimum stake to become validator
min_stake = 100000000

[privacy]
# Enable privacy features
enable_zk_proofs = true

# Privacy level
privacy_level = "high"

[wallet]
# Wallet encryption
encrypt_wallet = true

# Backup location
backup_location = "~/.unicoin/backups"
```

## Running Unicoin

### 1. Start Core Node

```bash
# Start with default configuration
./target/release/unicoin-node

# Start with custom configuration
./target/release/unicoin-node --config ~/.unicoin/config.toml

# Start in background
nohup ./target/release/unicoin-node > unicoin.log 2>&1 &
```

### 2. Start Wallet Application

```bash
cd apps/wallet
npm start
```

### 3. Start Explorer Application

```bash
cd apps/explorer
npm start
```

## Development Setup

### 1. Development Dependencies

```bash
# Install additional development tools
cargo install cargo-watch
cargo install cargo-expand
cargo install cargo-audit
```

### 2. IDE Setup

**VS Code:**
```bash
# Install Rust extension
code --install-extension rust-lang.rust-analyzer
```

**IntelliJ IDEA:**
- Install Rust plugin
- Configure Rust toolchain

### 3. Git Hooks

```bash
# Install pre-commit hooks
cp scripts/pre-commit .git/hooks/
chmod +x .git/hooks/pre-commit
```

## Testing

### 1. Unit Tests

```bash
# Run all tests
cargo test

# Run specific test module
cargo test blockchain

# Run tests with output
cargo test -- --nocapture
```

### 2. Integration Tests

```bash
# Run integration tests
cargo test --test integration

# Run with test data
cargo test --test integration -- --ignored
```

### 3. Performance Tests

```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench blockchain_bench
```

### 4. Security Tests

```bash
# Run security audit
cargo audit

# Run fuzzing tests
cargo fuzz run blockchain_fuzz
```

## Docker Build

### 1. Build Docker Image

```bash
# Build core image
docker build -t unicoin-core .

# Build applications image
docker build -t unicoin-apps -f Dockerfile.apps .
```

### 2. Run with Docker Compose

```bash
# Start full stack
docker-compose up -d

# View logs
docker-compose logs -f
```

## Troubleshooting

### Common Issues

**1. Build Fails with "No such file or directory"**
```bash
# Ensure all dependencies are installed
sudo apt update && sudo apt install build-essential
```

**2. "Permission denied" errors**
```bash
# Fix permissions
sudo chown -R $USER:$USER ~/.cargo
```

**3. Out of memory during build**
```bash
# Increase swap space or use fewer parallel jobs
export CARGO_BUILD_JOBS=2
```

**4. SSL/TLS errors**
```bash
# Update certificates
sudo apt update && sudo apt install ca-certificates
```

### Getting Help

- Check the [FAQ](faq.md)
- Join our [Discord](https://discord.gg/unicoin)
- Open an [issue](https://github.com/unicoin/unicoin/issues)

## Advanced Build Options

### 1. Custom Features

```bash
# Build with specific features
cargo build --release --features "privacy,quantum-resistant"

# Build without optional features
cargo build --release --no-default-features
```

### 2. Cross-Compilation

```bash
# Install target for cross-compilation
rustup target add x86_64-unknown-linux-gnu

# Build for specific target
cargo build --release --target x86_64-unknown-linux-gnu
```

### 3. Optimization

```bash
# Build with maximum optimization
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Build with size optimization
cargo build --release --config 'build.rustflags = ["-C", "opt-level=s"]'
```

## Security Considerations

1. **Always verify checksums** of downloaded dependencies
2. **Use HTTPS** for all downloads
3. **Keep dependencies updated** regularly
4. **Run security audits** before production deployment
5. **Use hardware security modules** for key management in production

## License

This build guide is part of Unicoin and is licensed under the MIT License.
