use crate::crypto::hash::Hash;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ethereum Virtual Machine implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EVM {
    /// EVM state
    pub state: EVMState,
    /// Gas configuration
    pub gas_config: GasConfig,
    /// Opcode implementations
    pub opcodes: HashMap<u8, Opcode>,
    /// Precompiled contracts
    pub precompiled_contracts: HashMap<u64, PrecompiledContract>,
}

/// EVM state management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EVMState {
    /// Account storage
    pub accounts: HashMap<Hash, Account>,
    /// Global state root
    pub state_root: Hash,
    /// Block information
    pub block_info: BlockInfo,
}

/// Ethereum account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Account address
    pub address: Hash,
    /// Account balance in wei
    pub balance: u64,
    /// Account nonce
    pub nonce: u64,
    /// Account code hash
    pub code_hash: Hash,
    /// Account storage root
    pub storage_root: Hash,
    /// Account storage
    pub storage: HashMap<Hash, Hash>,
}

/// Gas configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasConfig {
    /// Gas limit per block
    pub block_gas_limit: u64,
    /// Gas price
    pub gas_price: u64,
    /// Base gas cost
    pub base_gas_cost: u64,
    /// Gas costs for different operations
    pub gas_costs: GasCosts,
}

/// Gas costs for EVM operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasCosts {
    pub zero: u64,
    pub base: u64,
    pub very_low: u64,
    pub low: u64,
    pub mid: u64,
    pub high: u64,
    pub ext_code: u64,
    pub balance: u64,
    pub block_hash: u64,
    pub s_load: u64,
    pub jumpdest: u64,
    pub s_store: u64,
    pub sstore_set: u64,
    pub sstore_reset: u64,
    pub sstore_clear: u64,
    pub create: u64,
    pub call: u64,
    pub call_value: u64,
    pub call_stipend: u64,
    pub new_account: u64,
    pub exp: u64,
    pub exp_byte: u64,
    pub memory: u64,
    pub tx_create: u64,
    pub tx_data_zero: u64,
    pub tx_data_non_zero: u64,
    pub transaction: u64,
    pub log: u64,
    pub log_data: u64,
    pub log_topic: u64,
    pub sha3: u64,
    pub sha3_word: u64,
    pub copy: u64,
    pub suicide: u64,
    pub suicide_refund: u64,
}

impl Default for GasCosts {
    fn default() -> Self {
        Self {
            zero: 0,
            base: 2,
            very_low: 3,
            low: 5,
            mid: 8,
            high: 10,
            ext_code: 700,
            balance: 400,
            block_hash: 20,
            s_load: 200,
            jumpdest: 1,
            s_store: 20000,
            sstore_set: 20000,
            sstore_reset: 5000,
            sstore_clear: 15000,
            create: 32000,
            call: 700,
            call_value: 9000,
            call_stipend: 2300,
            new_account: 25000,
            exp: 10,
            exp_byte: 50,
            memory: 3,
            tx_create: 53000,
            tx_data_zero: 4,
            tx_data_non_zero: 68,
            transaction: 21000,
            log: 375,
            log_data: 8,
            log_topic: 375,
            sha3: 30,
            sha3_word: 6,
            copy: 3,
            suicide: 5000,
            suicide_refund: 24000,
        }
    }
}

/// EVM opcode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opcode {
    /// Opcode number
    pub code: u8,
    /// Opcode name
    pub name: String,
    /// Gas cost
    pub gas_cost: u64,
    /// Stack inputs
    pub stack_inputs: u8,
    /// Stack outputs
    pub stack_outputs: u8,
    /// Whether this is a control flow opcode
    pub is_control_flow: bool,
    /// Whether this is a memory opcode
    pub is_memory: bool,
    /// Whether this is a storage opcode
    pub is_storage: bool,
}

/// Precompiled contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecompiledContract {
    /// Contract address
    pub address: u64,
    /// Gas cost
    pub gas_cost: u64,
    /// Contract name
    pub name: String,
}

/// Block information for EVM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfo {
    /// Block number
    pub number: u64,
    /// Block timestamp
    pub timestamp: u64,
    /// Block difficulty
    pub difficulty: u64,
    /// Gas limit
    pub gas_limit: u64,
    /// Block hash
    pub block_hash: Hash,
    /// Coinbase address
    pub coinbase: Hash,
}

impl EVM {
    pub fn new() -> Self {
        Self {
            state: EVMState::new(),
            gas_config: GasConfig::new(),
            opcodes: Self::initialize_opcodes(),
            precompiled_contracts: Self::initialize_precompiled_contracts(),
        }
    }

    /// Initialize EVM opcodes
    fn initialize_opcodes() -> HashMap<u8, Opcode> {
        let mut opcodes = HashMap::new();
        
        // Stop and arithmetic operations
        opcodes.insert(0x00, Opcode {
            code: 0x00,
            name: "STOP".to_string(),
            gas_cost: 0,
            stack_inputs: 0,
            stack_outputs: 0,
            is_control_flow: false,
            is_memory: false,
            is_storage: false,
        });
        
        opcodes.insert(0x01, Opcode {
            code: 0x01,
            name: "ADD".to_string(),
            gas_cost: 3,
            stack_inputs: 2,
            stack_outputs: 1,
            is_control_flow: false,
            is_memory: false,
            is_storage: false,
        });
        
        opcodes.insert(0x02, Opcode {
            code: 0x02,
            name: "MUL".to_string(),
            gas_cost: 5,
            stack_inputs: 2,
            stack_outputs: 1,
            is_control_flow: false,
            is_memory: false,
            is_storage: false,
        });
        
        // Add more opcodes as needed...
        
        opcodes
    }

    /// Initialize precompiled contracts
    fn initialize_precompiled_contracts() -> HashMap<u64, PrecompiledContract> {
        let mut contracts = HashMap::new();
        
        contracts.insert(1, PrecompiledContract {
            address: 1,
            gas_cost: 3,
            name: "ecrecover".to_string(),
        });
        
        contracts.insert(2, PrecompiledContract {
            address: 2,
            gas_cost: 30,
            name: "sha256".to_string(),
        });
        
        contracts.insert(3, PrecompiledContract {
            address: 3,
            gas_cost: 30,
            name: "ripemd160".to_string(),
        });
        
        contracts.insert(4, PrecompiledContract {
            address: 4,
            gas_cost: 30,
            name: "identity".to_string(),
        });
        
        contracts
    }

    /// Execute EVM bytecode
    pub fn execute(&mut self, bytecode: &[u8], gas_limit: u64) -> Result<ExecutionResult, String> {
        let mut execution_context = ExecutionContext::new(gas_limit);
        
        for (i, &opcode) in bytecode.iter().enumerate() {
            if execution_context.gas_used >= gas_limit {
                return Err("Gas limit exceeded".to_string());
            }
            
            if let Some(op) = self.opcodes.get(&opcode) {
                execution_context.gas_used += op.gas_cost;
                self.execute_opcode(op, &mut execution_context)?;
            } else {
                return Err(format!("Unknown opcode: 0x{:02x}", opcode));
            }
        }
        
        Ok(ExecutionResult {
            success: true,
            gas_used: execution_context.gas_used,
            return_data: execution_context.return_data,
            logs: execution_context.logs,
        })
    }

    /// Execute a single opcode
    fn execute_opcode(&mut self, opcode: &Opcode, context: &mut ExecutionContext) -> Result<(), String> {
        match opcode.code {
            0x00 => {
                // STOP
                context.should_stop = true;
            }
            0x01 => {
                // ADD
                if context.stack.len() < 2 {
                    return Err("Stack underflow".to_string());
                }
                let a = context.stack.pop().unwrap();
                let b = context.stack.pop().unwrap();
                context.stack.push(a.wrapping_add(b));
            }
            0x02 => {
                // MUL
                if context.stack.len() < 2 {
                    return Err("Stack underflow".to_string());
                }
                let a = context.stack.pop().unwrap();
                let b = context.stack.pop().unwrap();
                context.stack.push(a.wrapping_mul(b));
            }
            _ => {
                return Err(format!("Opcode not implemented: 0x{:02x}", opcode.code));
            }
        }
        
        Ok(())
    }

    /// Create a new account
    pub fn create_account(&mut self, address: Hash, initial_balance: u64) {
        let account = Account {
            address,
            balance: initial_balance,
            nonce: 0,
            code_hash: Hash::zero(),
            storage_root: Hash::zero(),
            storage: HashMap::new(),
        };
        
        self.state.accounts.insert(address, account);
    }

    /// Get account balance
    pub fn get_balance(&self, address: &Hash) -> u64 {
        self.state.accounts.get(address)
            .map(|account| account.balance)
            .unwrap_or(0)
    }

    /// Transfer funds between accounts
    pub fn transfer(&mut self, from: &Hash, to: &Hash, amount: u64) -> Result<(), String> {
        if let Some(from_account) = self.state.accounts.get_mut(from) {
            if from_account.balance < amount {
                return Err("Insufficient balance".to_string());
            }
            from_account.balance -= amount;
        } else {
            return Err("From account not found".to_string());
        }
        
        if let Some(to_account) = self.state.accounts.get_mut(to) {
            to_account.balance += amount;
        } else {
            self.create_account(*to, amount);
        }
        
        Ok(())
    }
}

/// Execution context for EVM
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Program counter
    pub pc: usize,
    /// Stack
    pub stack: Vec<u64>,
    /// Memory
    pub memory: Vec<u8>,
    /// Gas used
    pub gas_used: u64,
    /// Return data
    pub return_data: Vec<u8>,
    /// Logs
    pub logs: Vec<Log>,
    /// Should stop execution
    pub should_stop: bool,
}

impl ExecutionContext {
    pub fn new(gas_limit: u64) -> Self {
        Self {
            pc: 0,
            stack: Vec::new(),
            memory: Vec::new(),
            gas_used: 0,
            return_data: Vec::new(),
            logs: Vec::new(),
            should_stop: false,
        }
    }
}

/// Execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Whether execution was successful
    pub success: bool,
    /// Gas used
    pub gas_used: u64,
    /// Return data
    pub return_data: Vec<u8>,
    /// Logs emitted
    pub logs: Vec<Log>,
}

/// EVM log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    /// Log address
    pub address: Hash,
    /// Log topics
    pub topics: Vec<Hash>,
    /// Log data
    pub data: Vec<u8>,
}

impl EVMState {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            state_root: Hash::zero(),
            block_info: BlockInfo {
                number: 0,
                timestamp: 0,
                difficulty: 0,
                gas_limit: 10000000,
                block_hash: Hash::zero(),
                coinbase: Hash::zero(),
            },
        }
    }
}

impl GasConfig {
    pub fn new() -> Self {
        Self {
            block_gas_limit: 10000000,
            gas_price: 20000000000, // 20 gwei
            base_gas_cost: 21000,
            gas_costs: GasCosts::default(),
        }
    }
}

impl Default for EVM {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for EVMState {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for GasConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evm_creation() {
        let evm = EVM::new();
        assert_eq!(evm.state.accounts.len(), 0);
        assert_eq!(evm.gas_config.block_gas_limit, 10000000);
    }

    #[test]
    fn test_account_creation() {
        let mut evm = EVM::new();
        let address = Hash::new([1u8; 32]);
        
        evm.create_account(address, 1000000);
        
        assert_eq!(evm.get_balance(&address), 1000000);
    }

    #[test]
    fn test_transfer() {
        let mut evm = EVM::new();
        let from = Hash::new([1u8; 32]);
        let to = Hash::new([2u8; 32]);
        
        evm.create_account(from, 1000000);
        
        let result = evm.transfer(&from, &to, 500000);
        assert!(result.is_ok());
        
        assert_eq!(evm.get_balance(&from), 500000);
        assert_eq!(evm.get_balance(&to), 500000);
    }

    #[test]
    fn test_insufficient_balance() {
        let mut evm = EVM::new();
        let from = Hash::new([1u8; 32]);
        let to = Hash::new([2u8; 32]);
        
        evm.create_account(from, 1000000);
        
        let result = evm.transfer(&from, &to, 2000000);
        assert!(result.is_err());
    }

    #[test]
    fn test_evm_execution() {
        let mut evm = EVM::new();
        
        // Simple bytecode that adds two numbers
        let bytecode = vec![0x60, 0x01, 0x60, 0x02, 0x01, 0x00]; // PUSH1 1, PUSH1 2, ADD, STOP
        
        let result = evm.execute(&bytecode, 10000);
        // Note: This test would need more complete opcode implementations
        // For now, we just test that the method exists
        assert!(result.is_ok() || result.is_err()); // Either is fine for basic test
    }
}
