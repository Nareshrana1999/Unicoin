# Unicoin API Reference

Complete API reference for the Unicoin blockchain and wallet services.

## Table of Contents

1. [Authentication](#authentication)
2. [Blockchain API](#blockchain-api)
3. [Wallet API](#wallet-api)
4. [Network API](#network-api)
5. [Smart Contracts API](#smart-contracts-api)
6. [DeFi API](#defi-api)
7. [NFT API](#nft-api)
8. [Privacy API](#privacy-api)
9. [Error Codes](#error-codes)
10. [Rate Limits](#rate-limits)

## Authentication

### API Key Authentication

All API requests require authentication using an API key:

```http
Authorization: Bearer YOUR_API_KEY
```

### JWT Authentication

For enhanced security, use JWT tokens:

```http
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

### Getting an API Key

1. Register at [developer.unicoin.com](https://developer.unicoin.com)
2. Create a new project
3. Generate API key
4. Configure permissions and rate limits

## Blockchain API

### Base URL
```
https://api.unicoin.com/v1/blockchain
```

### Get Blockchain Status

Get current blockchain status and statistics.

```http
GET /status
```

**Response:**
```json
{
  "height": 12345,
  "hash": "0x1234567890abcdef...",
  "timestamp": 1640995200,
  "difficulty": 1000000,
  "total_supply": "21000000.00000000",
  "circulating_supply": "18900000.00000000",
  "block_reward": "6.25000000",
  "network_hashrate": "150000000000",
  "avg_block_time": 30,
  "last_block_time": 1640995170
}
```

### Get Block by Height

Retrieve block information by height.

```http
GET /block/{height}
```

**Parameters:**
- `height` (integer): Block height

**Response:**
```json
{
  "header": {
    "height": 12345,
    "hash": "0x1234567890abcdef...",
    "previous_hash": "0xabcdef1234567890...",
    "merkle_root": "0x9876543210fedcba...",
    "timestamp": 1640995200,
    "nonce": 123456789,
    "difficulty": 1000000,
    "version": 1
  },
  "transactions": [
    {
      "id": "0xabcdef1234567890...",
      "type": "coinbase",
      "inputs": [],
      "outputs": [
        {
          "address": "UNI1abc123...",
          "amount": "6.25000000"
        }
      ],
      "fee": "0.00000000",
      "timestamp": 1640995200
    }
  ],
  "transaction_count": 1,
  "size": 1024,
  "weight": 4096
}
```

### Get Block by Hash

Retrieve block information by hash.

```http
GET /block/hash/{hash}
```

**Parameters:**
- `hash` (string): Block hash

### Get Latest Blocks

Get the most recent blocks.

```http
GET /blocks
```

**Query Parameters:**
- `limit` (integer, optional): Number of blocks to return (default: 10, max: 100)
- `offset` (integer, optional): Number of blocks to skip (default: 0)

**Response:**
```json
{
  "blocks": [
    {
      "height": 12345,
      "hash": "0x1234567890abcdef...",
      "timestamp": 1640995200,
      "transaction_count": 1,
      "size": 1024
    }
  ],
  "total": 12345,
  "limit": 10,
  "offset": 0
}
```

### Get Transaction by Hash

Retrieve transaction details by hash.

```http
GET /transaction/{hash}
```

**Parameters:**
- `hash` (string): Transaction hash

**Response:**
```json
{
  "id": "0xabcdef1234567890...",
  "hash": "0xabcdef1234567890...",
  "type": "transfer",
  "version": 1,
  "inputs": [
    {
      "previous_output": {
        "tx_id": "0x1234567890abcdef...",
        "output_index": 0
      },
      "script_sig": "0x...",
      "sequence": 4294967295,
      "witness": null
    }
  ],
  "outputs": [
    {
      "address": "UNI1def456...",
      "amount": "1.00000000",
      "script_pubkey": "0x...",
      "output_type": "p2pkh"
    }
  ],
  "lock_time": 0,
  "fee": "0.00100000",
  "timestamp": 1640995200,
  "confirmations": 6,
  "block_height": 12345,
  "block_hash": "0x1234567890abcdef...",
  "size": 256,
  "weight": 1024
}
```

### Get UTXO Set

Get unspent transaction outputs.

```http
GET /utxos
```

**Query Parameters:**
- `address` (string, optional): Filter by address
- `limit` (integer, optional): Number of UTXOs to return (default: 100, max: 1000)
- `offset` (integer, optional): Number of UTXOs to skip (default: 0)

**Response:**
```json
{
  "utxos": [
    {
      "tx_id": "0x1234567890abcdef...",
      "output_index": 0,
      "address": "UNI1abc123...",
      "amount": "1.00000000",
      "height": 12340,
      "is_coinbase": false,
      "confirmations": 6
    }
  ],
  "total": 1500,
  "limit": 100,
  "offset": 0
}
```

### Get Address Balance

Get balance for a specific address.

```http
GET /balance/{address}
```

**Parameters:**
- `address` (string): Unicoin address

**Response:**
```json
{
  "address": "UNI1abc123...",
  "confirmed_balance": "10.00000000",
  "unconfirmed_balance": "0.50000000",
  "total_balance": "10.50000000",
  "utxo_count": 5,
  "last_activity": 1640995200
}
```

### Get Address Transactions

Get transaction history for an address.

```http
GET /address/{address}/transactions
```

**Query Parameters:**
- `limit` (integer, optional): Number of transactions to return (default: 20, max: 100)
- `offset` (integer, optional): Number of transactions to skip (default: 0)
- `type` (string, optional): Filter by transaction type (sent, received, all)

**Response:**
```json
{
  "transactions": [
    {
      "id": "0xabcdef1234567890...",
      "type": "received",
      "amount": "1.00000000",
      "fee": "0.00100000",
      "timestamp": 1640995200,
      "confirmations": 6,
      "from": "UNI1def456...",
      "to": "UNI1abc123..."
    }
  ],
  "total": 25,
  "limit": 20,
  "offset": 0
}
```

## Wallet API

### Base URL
```
https://api.unicoin.com/v1/wallet
```

### Create Wallet

Create a new wallet.

```http
POST /create
Content-Type: application/json

{
  "name": "My Wallet",
  "password": "secure_password",
  "network": "mainnet"
}
```

**Response:**
```json
{
  "wallet_id": "wallet_1234567890",
  "name": "My Wallet",
  "address": "UNI1abc123...",
  "seed_phrase": "abandon abandon abandon...",
  "created_at": "2023-01-01T00:00:00Z"
}
```

### Import Wallet

Import an existing wallet using seed phrase.

```http
POST /import
Content-Type: application/json

{
  "name": "Imported Wallet",
  "seed_phrase": "abandon abandon abandon...",
  "password": "secure_password",
  "network": "mainnet"
}
```

### Get Wallet Info

Get wallet information.

```http
GET /{wallet_id}
```

**Response:**
```json
{
  "wallet_id": "wallet_1234567890",
  "name": "My Wallet",
  "address": "UNI1abc123...",
  "balance": "10.00000000",
  "created_at": "2023-01-01T00:00:00Z",
  "last_activity": "2023-01-01T12:00:00Z"
}
```

### Generate New Address

Generate a new address for the wallet.

```http
POST /{wallet_id}/address
```

**Response:**
```json
{
  "address": "UNI1def456...",
  "index": 1,
  "derivation_path": "m/44'/1'/0'/0/1"
}
```

### Send Transaction

Send a transaction from the wallet.

```http
POST /{wallet_id}/send
Content-Type: application/json

{
  "to": "UNI1def456...",
  "amount": "1.00000000",
  "fee": "0.00100000",
  "memo": "Payment for services",
  "password": "secure_password"
}
```

**Response:**
```json
{
  "transaction_id": "0xabcdef1234567890...",
  "hash": "0xabcdef1234567890...",
  "status": "pending",
  "timestamp": 1640995200,
  "fee": "0.00100000"
}
```

### Get Wallet Transactions

Get transaction history for a wallet.

```http
GET /{wallet_id}/transactions
```

**Query Parameters:**
- `limit` (integer, optional): Number of transactions to return (default: 20, max: 100)
- `offset` (integer, optional): Number of transactions to skip (default: 0)
- `status` (string, optional): Filter by status (pending, confirmed, failed)

## Network API

### Base URL
```
https://api.unicoin.com/v1/network
```

### Get Network Status

Get current network status and statistics.

```http
GET /status
```

**Response:**
```json
{
  "connected_peers": 25,
  "total_peers": 150,
  "network_hashrate": "150000000000",
  "difficulty": 1000000,
  "avg_block_time": 30,
  "uptime": 86400,
  "version": "1.0.0",
  "protocol_version": 1
}
```

### Get Peer List

Get list of connected peers.

```http
GET /peers
```

**Response:**
```json
{
  "peers": [
    {
      "address": "192.168.1.100:30301",
      "peer_id": "peer_1234567890",
      "version": "1.0.0",
      "capabilities": ["full_node", "api"],
      "last_seen": 1640995200,
      "latency": 50
    }
  ],
  "total": 25
}
```

### Broadcast Transaction

Broadcast a transaction to the network.

```http
POST /broadcast
Content-Type: application/json

{
  "transaction": "0xabcdef1234567890..."
}
```

**Response:**
```json
{
  "success": true,
  "transaction_id": "0xabcdef1234567890...",
  "message": "Transaction broadcast successfully"
}
```

### Get Network Statistics

Get detailed network statistics.

```http
GET /stats
```

**Response:**
```json
{
  "blocks_per_hour": 120,
  "transactions_per_hour": 1500,
  "avg_transaction_size": 256,
  "network_throughput": "1000 tps",
  "mempool_size": 150,
  "pending_transactions": 150
}
```

## Smart Contracts API

### Base URL
```
https://api.unicoin.com/v1/contracts
```

### Deploy Contract

Deploy a smart contract.

```http
POST /deploy
Content-Type: application/json

{
  "bytecode": "0x608060405234801561001057600080fd5b50...",
  "constructor_args": [],
  "gas_limit": 1000000,
  "gas_price": "0.00000001"
}
```

**Response:**
```json
{
  "contract_address": "0x1234567890abcdef...",
  "transaction_hash": "0xabcdef1234567890...",
  "gas_used": 500000,
  "status": "success"
}
```

### Call Contract

Call a contract function.

```http
POST /call
Content-Type: application/json

{
  "contract_address": "0x1234567890abcdef...",
  "function": "transfer",
  "args": ["0xdef456...", "1000000000000000000"],
  "gas_limit": 100000,
  "gas_price": "0.00000001"
}
```

### Get Contract Info

Get contract information.

```http
GET /{contract_address}
```

**Response:**
```json
{
  "address": "0x1234567890abcdef...",
  "bytecode": "0x608060405234801561001057600080fd5b50...",
  "abi": [...],
  "creator": "UNI1abc123...",
  "created_at": "2023-01-01T00:00:00Z",
  "transaction_count": 150
}
```

## DeFi API

### Base URL
```
https://api.unicoin.com/v1/defi
```

### DEX Operations

#### Get Quote

Get a quote for a token swap.

```http
GET /dex/quote
```

**Query Parameters:**
- `from_token`: Source token address
- `to_token`: Destination token address
- `amount`: Amount to swap

**Response:**
```json
{
  "from_token": "0x1234567890abcdef...",
  "to_token": "0xabcdef1234567890...",
  "amount_in": "1.00000000",
  "amount_out": "0.95000000",
  "price_impact": "0.05",
  "slippage": "0.01",
  "route": [
    "0x1234567890abcdef...",
    "0xabcdef1234567890..."
  ]
}
```

#### Execute Swap

Execute a token swap.

```http
POST /dex/swap
Content-Type: application/json

{
  "from_token": "0x1234567890abcdef...",
  "to_token": "0xabcdef1234567890...",
  "amount_in": "1.00000000",
  "min_amount_out": "0.95000000",
  "slippage": "0.01",
  "deadline": 1640995200
}
```

### Lending Operations

#### Supply Assets

Supply assets to a lending pool.

```http
POST /lending/supply
Content-Type: application/json

{
  "asset": "0x1234567890abcdef...",
  "amount": "100.00000000",
  "interest_rate_mode": "variable"
}
```

#### Borrow Assets

Borrow assets from a lending pool.

```http
POST /lending/borrow
Content-Type: application/json

{
  "asset": "0x1234567890abcdef...",
  "amount": "50.00000000",
  "interest_rate_mode": "variable",
  "collateral_asset": "0xabcdef1234567890..."
}
```

### Yield Farming

#### Stake Tokens

Stake tokens in a farming pool.

```http
POST /farming/stake
Content-Type: application/json

{
  "pool": "0x1234567890abcdef...",
  "amount": "1000.00000000",
  "duration": 30
}
```

#### Harvest Rewards

Harvest farming rewards.

```http
POST /farming/harvest
Content-Type: application/json

{
  "pool": "0x1234567890abcdef..."
}
```

## NFT API

### Base URL
```
https://api.unicoin.com/v1/nft
```

### Create NFT

Create a new NFT.

```http
POST /create
Content-Type: application/json

{
  "name": "My NFT",
  "description": "A unique digital asset",
  "image": "https://example.com/image.png",
  "attributes": [
    {
      "trait_type": "Color",
      "value": "Blue"
    }
  ],
  "collection": "My Collection"
}
```

### Get NFT Info

Get NFT information.

```http
GET /{token_id}
```

**Response:**
```json
{
  "token_id": "1234567890",
  "name": "My NFT",
  "description": "A unique digital asset",
  "image": "https://example.com/image.png",
  "owner": "UNI1abc123...",
  "creator": "UNI1def456...",
  "collection": "My Collection",
  "attributes": [
    {
      "trait_type": "Color",
      "value": "Blue"
    }
  ],
  "created_at": "2023-01-01T00:00:00Z"
}
```

### Transfer NFT

Transfer an NFT to another address.

```http
POST /{token_id}/transfer
Content-Type: application/json

{
  "to": "UNI1def456...",
  "from": "UNI1abc123..."
}
```

### List NFT for Sale

List an NFT for sale.

```http
POST /{token_id}/list
Content-Type: application/json

{
  "price": "1.00000000",
  "currency": "UNI",
  "duration": 86400
}
```

## Privacy API

### Base URL
```
https://api.unicoin.com/v1/privacy
```

### Generate Zero-Knowledge Proof

Generate a zero-knowledge proof.

```http
POST /zk-proof
Content-Type: application/json

{
  "proof_type": "zk-snark",
  "circuit": "transfer_circuit",
  "inputs": {
    "amount": "1.00000000",
    "balance": "10.00000000"
  }
}
```

### Verify Zero-Knowledge Proof

Verify a zero-knowledge proof.

```http
POST /zk-verify
Content-Type: application/json

{
  "proof": "0xabcdef1234567890...",
  "public_inputs": ["1.00000000", "10.00000000"]
}
```

### Create Ring Signature

Create a ring signature for privacy.

```http
POST /ring-signature
Content-Type: application/json

{
  "message": "0x1234567890abcdef...",
  "ring_members": [
    "0x1111111111111111...",
    "0x2222222222222222...",
    "0x3333333333333333..."
  ],
  "private_key": "0xabcdef1234567890..."
}
```

## Error Codes

### HTTP Status Codes

- `200 OK`: Request successful
- `201 Created`: Resource created successfully
- `400 Bad Request`: Invalid request parameters
- `401 Unauthorized`: Authentication required
- `403 Forbidden`: Insufficient permissions
- `404 Not Found`: Resource not found
- `429 Too Many Requests`: Rate limit exceeded
- `500 Internal Server Error`: Server error

### Error Response Format

```json
{
  "error": {
    "code": "INVALID_ADDRESS",
    "message": "Invalid address format",
    "details": {
      "address": "invalid_address"
    }
  },
  "request_id": "req_1234567890",
  "timestamp": "2023-01-01T00:00:00Z"
}
```

### Common Error Codes

- `INVALID_ADDRESS`: Invalid address format
- `INSUFFICIENT_BALANCE`: Insufficient balance for transaction
- `INVALID_TRANSACTION`: Invalid transaction format
- `TRANSACTION_FAILED`: Transaction execution failed
- `CONTRACT_NOT_FOUND`: Smart contract not found
- `INVALID_PROOF`: Invalid zero-knowledge proof
- `RATE_LIMIT_EXCEEDED`: API rate limit exceeded

## Rate Limits

### Default Limits

- **Free Tier**: 100 requests per minute
- **Pro Tier**: 1000 requests per minute
- **Enterprise**: Custom limits

### Rate Limit Headers

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1640995200
```

### Rate Limit Exceeded Response

```json
{
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Rate limit exceeded. Try again in 60 seconds.",
    "retry_after": 60
  }
}
```

---

## SDKs and Libraries

### JavaScript/TypeScript

```bash
npm install @unicoin/sdk
```

```javascript
import { UnicoinAPI } from '@unicoin/sdk';

const api = new UnicoinAPI({
  apiKey: 'your-api-key',
  network: 'mainnet'
});

// Get blockchain status
const status = await api.blockchain.getStatus();

// Send transaction
const tx = await api.wallet.send({
  to: 'UNI1abc123...',
  amount: '1.00000000',
  fee: '0.00100000'
});
```

### Python

```bash
pip install unicoin-python
```

```python
from unicoin import UnicoinAPI

api = UnicoinAPI(api_key='your-api-key', network='mainnet')

# Get blockchain status
status = api.blockchain.get_status()

# Send transaction
tx = api.wallet.send(
    to='UNI1abc123...',
    amount='1.00000000',
    fee='0.00100000'
)
```

### Rust

```toml
[dependencies]
unicoin-sdk = "0.1.0"
```

```rust
use unicoin_sdk::UnicoinAPI;

let api = UnicoinAPI::new("your-api-key", "mainnet");

// Get blockchain status
let status = api.blockchain.get_status().await?;

// Send transaction
let tx = api.wallet.send(SendRequest {
    to: "UNI1abc123...".to_string(),
    amount: "1.00000000".to_string(),
    fee: "0.00100000".to_string(),
}).await?;
```

---

For more information and examples, visit our [developer documentation](https://docs.unicoin.com) or join our [Discord community](https://discord.gg/unicoin).
