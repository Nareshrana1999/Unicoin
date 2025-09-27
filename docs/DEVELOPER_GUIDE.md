# Unicoin Developer Guide

Welcome to the Unicoin Developer Guide! This comprehensive documentation will help you build on the Unicoin blockchain, integrate with our APIs, and develop applications using our advanced features.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Architecture Overview](#architecture-overview)
3. [API Documentation](#api-documentation)
4. [Smart Contract Development](#smart-contract-development)
5. [DeFi Integration](#defi-integration)
6. [Privacy Features](#privacy-features)
7. [Node Development](#node-development)
8. [Security Guidelines](#security-guidelines)
9. [Testing](#testing)
10. [Deployment](#deployment)

## Getting Started

### Prerequisites

**Development Environment:**
- Rust 1.70+ (for core development)
- Node.js 16+ (for web development)
- Git (for version control)
- Docker (for containerization)

**Recommended Tools:**
- VS Code with Rust and TypeScript extensions
- Postman (for API testing)
- Hardhat or Foundry (for smart contract development)
- Metamask (for wallet integration)

### Quick Start

1. **Clone the Repository**
   ```bash
   git clone https://github.com/Nareshrana1999/Unicoin.git
   cd Unicoin
   ```

2. **Install Dependencies**
   ```bash
   # Install Rust dependencies
   cargo build

   # Install Node.js dependencies
   npm install

   # Install web wallet dependencies
   cd apps/web-wallet
   npm install
   ```

3. **Run a Test Node**
   ```bash
   cargo run --bin unicoin-node
   ```

4. **Access the API**
   ```bash
   curl http://localhost:8080/api/v1/status
   ```

### Development Setup

**Environment Variables:**
```bash
export UNICOIN_NETWORK=testnet
export UNICOIN_API_PORT=8080
export UNICOIN_RPC_PORT=8545
export UNICOIN_P2P_PORT=30301
```

**Configuration File:**
```toml
# config.toml
[network]
listen_address = "0.0.0.0"
listen_port = 30301
network_id = 1

[api]
enabled = true
port = 8080
host = "0.0.0.0"

[blockchain]
chain_id = 1
genesis_timestamp = 1640995200
```

## Architecture Overview

### Core Components

**Blockchain Layer:**
- Block structure and validation
- Transaction processing
- UTXO management
- Merkle tree implementation
- Difficulty adjustment

**Consensus Layer:**
- UniConsensus algorithm
- Validator management
- Slashing mechanisms
- Geographic distribution
- Multi-factor scoring

**Network Layer:**
- P2P communication
- Message routing
- Peer discovery
- Protocol handshake
- Network statistics

**Cryptography Layer:**
- Quantum-resistant algorithms
- Signature schemes
- Hash functions
- Encryption protocols
- Key management

**Application Layer:**
- REST API
- WebSocket API
- GraphQL API
- CLI interface
- Web interface

### Data Structures

**Block Structure:**
```rust
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub merkle_root: Hash,
    pub timestamp: u64,
    pub nonce: u64,
}
```

**Transaction Structure:**
```rust
pub struct Transaction {
    pub id: Hash,
    pub version: u32,
    pub tx_type: TransactionType,
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub lock_time: u32,
    pub fee: u64,
    pub signature: Option<Signature>,
}
```

**UTXO Structure:**
```rust
pub struct UTXO {
    pub tx_id: Hash,
    pub output_index: u32,
    pub output: TransactionOutput,
    pub height: u64,
    pub is_coinbase: bool,
    pub confirmations: u64,
}
```

## API Documentation

### REST API

**Base URL:** `http://localhost:8080/api/v1`

#### Blockchain Endpoints

**Get Blockchain Status**
```http
GET /blockchain/status
```

Response:
```json
{
  "height": 12345,
  "hash": "0x1234...",
  "timestamp": 1640995200,
  "difficulty": 1000000,
  "total_supply": 21000000
}
```

**Get Block by Height**
```http
GET /blockchain/block/{height}
```

**Get Transaction by Hash**
```http
GET /blockchain/transaction/{hash}
```

**Get UTXO Set**
```http
GET /blockchain/utxos
```

#### Wallet Endpoints

**Create Wallet**
```http
POST /wallet/create
Content-Type: application/json

{
  "name": "My Wallet",
  "password": "secure_password"
}
```

**Get Balance**
```http
GET /wallet/balance/{address}
```

**Send Transaction**
```http
POST /wallet/send
Content-Type: application/json

{
  "from": "UNI1...",
  "to": "UNI1...",
  "amount": 1000000,
  "fee": 1000,
  "memo": "Payment for services"
}
```

#### Network Endpoints

**Get Peer List**
```http
GET /network/peers
```

**Get Network Stats**
```http
GET /network/stats
```

**Broadcast Transaction**
```http
POST /network/broadcast
Content-Type: application/json

{
  "transaction": "..."
}
```

### WebSocket API

**Connection:**
```javascript
const ws = new WebSocket('ws://localhost:8080/ws');

ws.onopen = function() {
  // Subscribe to events
  ws.send(JSON.stringify({
    type: 'subscribe',
    events: ['new_block', 'new_transaction']
  }));
};

ws.onmessage = function(event) {
  const data = JSON.parse(event.data);
  console.log('Received:', data);
};
```

**Event Types:**
- `new_block`: New block added to blockchain
- `new_transaction`: New transaction in mempool
- `peer_connected`: New peer connected
- `peer_disconnected`: Peer disconnected
- `consensus_event`: Consensus-related events

### GraphQL API

**Schema:**
```graphql
type Query {
  blockchain: Blockchain!
  transaction(hash: String!): Transaction
  block(height: Int): Block
  wallet(address: String!): Wallet
}

type Blockchain {
  height: Int!
  hash: String!
  timestamp: Int!
  difficulty: Int!
  totalSupply: String!
}

type Transaction {
  id: String!
  hash: String!
  type: TransactionType!
  inputs: [TransactionInput!]!
  outputs: [TransactionOutput!]!
  fee: String!
  timestamp: Int!
}
```

**Example Query:**
```graphql
query {
  blockchain {
    height
    hash
    totalSupply
  }
  transaction(hash: "0x1234...") {
    id
    type
    inputs {
      previousOutput
      scriptSig
    }
    outputs {
      address
      amount
    }
  }
}
```

## Smart Contract Development

### EVM Compatibility

Unicoin supports Ethereum Virtual Machine (EVM) compatibility, allowing you to deploy and interact with Ethereum smart contracts.

**Supported Features:**
- Solidity contracts
- Vyper contracts
- Precompiled contracts
- Gas optimization
- Account abstraction

### Development Tools

**Hardhat Integration:**
```javascript
// hardhat.config.js
module.exports = {
  networks: {
    unicoin: {
      url: "http://localhost:8545",
      chainId: 1,
      accounts: [process.env.PRIVATE_KEY]
    }
  }
};
```

**Contract Example:**
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract UnicoinToken {
    mapping(address => uint256) public balanceOf;
    string public name = "Unicoin Token";
    string public symbol = "UNI";
    uint8 public decimals = 18;
    
    event Transfer(address indexed from, address indexed to, uint256 value);
    
    constructor() {
        balanceOf[msg.sender] = 1000000 * 10**decimals;
    }
    
    function transfer(address to, uint256 value) public returns (bool) {
        require(balanceOf[msg.sender] >= value);
        balanceOf[msg.sender] -= value;
        balanceOf[to] += value;
        emit Transfer(msg.sender, to, value);
        return true;
    }
}
```

### AI-Enhanced Smart Contracts

Unicoin supports AI-enhanced smart contracts with automatic optimization and intelligent features.

**AI Features:**
- Gas optimization
- Security analysis
- Performance monitoring
- Automated testing
- Predictive analytics

**Example AI Contract:**
```solidity
contract AITradingBot {
    using AI for *;
    
    function executeTrade(
        address token,
        uint256 amount,
        uint256 slippage
    ) public {
        // AI analyzes market conditions
        MarketAnalysis memory analysis = AI.analyzeMarket(token);
        
        // AI optimizes trade execution
        TradeOptimization memory optimization = AI.optimizeTrade(
            token,
            amount,
            slippage,
            analysis
        );
        
        // Execute optimized trade
        executeOptimizedTrade(optimization);
    }
}
```

## DeFi Integration

### DEX Integration

**Swap Function:**
```javascript
async function swapTokens(
  fromToken,
  toToken,
  amount,
  slippage = 0.01
) {
  const quote = await api.getQuote(fromToken, toToken, amount);
  const minAmountOut = quote.amountOut * (1 - slippage);
  
  const tx = await dex.swap(
    fromToken,
    toToken,
    amount,
    minAmountOut,
    userAddress
  );
  
  return tx;
}
```

**Liquidity Provision:**
```javascript
async function addLiquidity(
  tokenA,
  tokenB,
  amountA,
  amountB
) {
  const tx = await dex.addLiquidity(
    tokenA,
    tokenB,
    amountA,
    amountB,
    userAddress
  );
  
  return tx;
}
```

### Lending Protocol

**Borrow Function:**
```javascript
async function borrow(
  asset,
  amount,
  collateral
) {
  const tx = await lending.borrow(
    asset,
    amount,
    collateral,
    userAddress
  );
  
  return tx;
}
```

**Supply Function:**
```javascript
async function supply(asset, amount) {
  const tx = await lending.supply(asset, amount, userAddress);
  return tx;
}
```

### Yield Farming

**Stake Function:**
```javascript
async function stake(pool, amount) {
  const tx = await farming.stake(pool, amount, userAddress);
  return tx;
}
```

**Harvest Function:**
```javascript
async function harvest(pool) {
  const rewards = await farming.calculateRewards(pool, userAddress);
  const tx = await farming.harvest(pool, userAddress);
  return { tx, rewards };
}
```

## Privacy Features

### Zero-Knowledge Proofs

**zk-SNARKs Implementation:**
```rust
use unicoin::privacy::zk_proofs::ZkSnark;

let proof = ZkSnark::generate_proof(
    &witness,
    &proving_key,
)?;

let is_valid = ZkSnark::verify_proof(
    &proof,
    &verifying_key,
    &public_inputs,
)?;
```

**zk-STARKs Implementation:**
```rust
use unicoin::privacy::zk_proofs::ZkStark;

let proof = ZkStark::generate_proof(
    &witness,
    &stark_params,
)?;

let is_valid = ZkStark::verify_proof(
    &proof,
    &stark_params,
)?;
```

### Ring Signatures

**Creating Ring Signature:**
```rust
use unicoin::privacy::ring_signature::RingSignature;

let ring_sig = RingSignature::create(
    &message,
    &private_key,
    &ring_members,
)?;

let is_valid = ring_sig.verify(&message, &ring_members)?;
```

### Stealth Addresses

**Generating Stealth Address:**
```rust
use unicoin::privacy::stealth_address::StealthAddress;

let stealth_addr = StealthAddress::generate(
    &sender_private_key,
    &recipient_public_key,
)?;

let derived_address = stealth_addr.derive_address(&index)?;
```

## Node Development

### Running a Node

**Basic Node:**
```bash
cargo run --bin unicoin-node
```

**Custom Configuration:**
```bash
cargo run --bin unicoin-node -- --config config.toml
```

**CLI Commands:**
```bash
# Start node
unicoin-cli node start

# Stop node
unicoin-cli node stop

# Get status
unicoin-cli node status

# Get logs
unicoin-cli node logs
```

### Validator Setup

**Become a Validator:**
```bash
# Create validator key
unicoin-cli validator create-key

# Register validator
unicoin-cli validator register \
  --stake 1000000 \
  --commission 0.05 \
  --description "My Validator Node"

# Start validating
unicoin-cli validator start
```

**Validator Management:**
```bash
# Update commission
unicoin-cli validator update-commission 0.03

# Update description
unicoin-cli validator update-description "Updated Description"

# Unbond validator
unicoin-cli validator unbond
```

### Network Participation

**Peer Management:**
```bash
# Add peer
unicoin-cli network add-peer 192.168.1.100:30301

# Remove peer
unicoin-cli network remove-peer 192.168.1.100:30301

# List peers
unicoin-cli network list-peers
```

**Network Monitoring:**
```bash
# Get network stats
unicoin-cli network stats

# Monitor connections
unicoin-cli network monitor

# Check sync status
unicoin-cli network sync-status
```

## Security Guidelines

### Best Practices

**Code Security:**
- Use latest compiler versions
- Enable all security warnings
- Implement proper error handling
- Use secure random number generation
- Validate all inputs

**Key Management:**
- Use hardware security modules (HSMs)
- Implement key rotation policies
- Secure key storage
- Use multi-signature schemes
- Regular security audits

**Network Security:**
- Use encrypted connections
- Implement rate limiting
- Monitor for suspicious activity
- Use firewalls and VPNs
- Regular security updates

### Vulnerability Reporting

**Security Disclosure:**
- Email: security@unicoin.com
- PGP Key: [Available on website]
- Bug Bounty: [Bug bounty program details]

**Response Process:**
1. Acknowledge receipt within 24 hours
2. Initial assessment within 72 hours
3. Regular updates during investigation
4. Coordinated disclosure timeline
5. Public disclosure after fix

## Testing

### Unit Testing

**Rust Tests:**
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_transaction_validation

# Run with coverage
cargo test --coverage
```

**JavaScript Tests:**
```bash
# Run web wallet tests
cd apps/web-wallet
npm test

# Run with coverage
npm run test:coverage
```

### Integration Testing

**Blockchain Tests:**
```bash
# Run integration tests
cargo test --test integration_tests

# Run with different configurations
cargo test --test integration_tests -- --test-threads=1
```

**API Tests:**
```bash
# Test API endpoints
npm run test:api

# Test with different environments
npm run test:api:testnet
npm run test:api:mainnet
```

### Performance Testing

**Benchmarking:**
```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench transaction_processing

# Generate performance report
cargo bench -- --output-format=json > benchmarks.json
```

**Load Testing:**
```bash
# Test API performance
npm run test:load

# Test transaction throughput
npm run test:throughput
```

## Deployment

### Docker Deployment

**Dockerfile:**
```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/unicoin-node /usr/local/bin/
EXPOSE 8080 8545 30301
CMD ["unicoin-node"]
```

**Docker Compose:**
```yaml
version: '3.8'
services:
  unicoin-node:
    build: .
    ports:
      - "8080:8080"
      - "8545:8545"
      - "30301:30301"
    environment:
      - UNICOIN_NETWORK=mainnet
    volumes:
      - ./data:/app/data
      - ./config.toml:/app/config.toml
```

### Kubernetes Deployment

**Deployment YAML:**
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: unicoin-node
spec:
  replicas: 3
  selector:
    matchLabels:
      app: unicoin-node
  template:
    metadata:
      labels:
        app: unicoin-node
    spec:
      containers:
      - name: unicoin-node
        image: unicoin/node:latest
        ports:
        - containerPort: 8080
        - containerPort: 8545
        - containerPort: 30301
        env:
        - name: UNICOIN_NETWORK
          value: "mainnet"
        volumeMounts:
        - name: config
          mountPath: /app/config.toml
      volumes:
      - name: config
        configMap:
          name: unicoin-config
```

### Cloud Deployment

**AWS Deployment:**
```bash
# Create EC2 instance
aws ec2 run-instances \
  --image-id ami-0c02fb55956c7d316 \
  --instance-type t3.medium \
  --key-name unicoin-key \
  --security-groups unicoin-sg

# Install dependencies
sudo apt-get update
sudo apt-get install -y docker.io

# Run container
sudo docker run -d \
  --name unicoin-node \
  -p 8080:8080 \
  -p 8545:8545 \
  -p 30301:30301 \
  unicoin/node:latest
```

**Google Cloud Deployment:**
```bash
# Create VM instance
gcloud compute instances create unicoin-node \
  --image-family ubuntu-2004-lts \
  --image-project ubuntu-os-cloud \
  --machine-type e2-medium \
  --tags unicoin-node

# Install Docker
sudo apt-get update
sudo apt-get install -y docker.io

# Run container
sudo docker run -d \
  --name unicoin-node \
  -p 8080:8080 \
  -p 8545:8545 \
  -p 30301:30301 \
  unicoin/node:latest
```

---

## Conclusion

This developer guide provides comprehensive documentation for building on the Unicoin blockchain. From basic API integration to advanced smart contract development, Unicoin offers powerful tools and features for developers.

For additional resources and support:
- Documentation: [docs.unicoin.com](https://docs.unicoin.com)
- API Reference: [api.unicoin.com](https://api.unicoin.com)
- Community: [discord.gg/unicoin](https://discord.gg/unicoin)
- GitHub: [github.com/Nareshrana1999/Unicoin](https://github.com/Nareshrana1999/Unicoin)

Happy building with Unicoin! 🚀
