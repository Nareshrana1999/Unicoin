use crate::crypto::{hash::Hash, keys::PublicKey, signatures::Signature};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Bitcoin Script operation codes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptOp {
    // Constants
    OP_0 = 0x00,
    OP_PUSHDATA1 = 0x4c,
    OP_PUSHDATA2 = 0x4d,
    OP_PUSHDATA4 = 0x4e,
    OP_1NEGATE = 0x4f,
    OP_1 = 0x51,
    OP_2 = 0x52,
    OP_3 = 0x53,
    OP_4 = 0x54,
    OP_5 = 0x55,
    OP_6 = 0x56,
    OP_7 = 0x57,
    OP_8 = 0x58,
    OP_9 = 0x59,
    OP_10 = 0x5a,
    OP_11 = 0x5b,
    OP_12 = 0x5c,
    OP_13 = 0x5d,
    OP_14 = 0x5e,
    OP_15 = 0x5f,
    OP_16 = 0x60,

    // Flow control
    OP_NOP = 0x61,
    OP_IF = 0x63,
    OP_NOTIF = 0x64,
    OP_ELSE = 0x67,
    OP_ENDIF = 0x68,
    OP_VERIFY = 0x69,
    OP_RETURN = 0x6a,

    // Stack
    OP_TOALTSTACK = 0x6b,
    OP_FROMALTSTACK = 0x6c,
    OP_2DROP = 0x6d,
    OP_2DUP = 0x6e,
    OP_3DUP = 0x6f,
    OP_2OVER = 0x70,
    OP_2ROT = 0x71,
    OP_2SWAP = 0x72,
    OP_IFDUP = 0x73,
    OP_DEPTH = 0x74,
    OP_DROP = 0x75,
    OP_DUP = 0x76,
    OP_NIP = 0x77,
    OP_OVER = 0x78,
    OP_PICK = 0x79,
    OP_ROLL = 0x7a,
    OP_ROT = 0x7b,
    OP_SWAP = 0x7c,
    OP_TUCK = 0x7d,

    // Splice
    OP_CAT = 0x7e,
    OP_SUBSTR = 0x7f,
    OP_LEFT = 0x80,
    OP_RIGHT = 0x81,
    OP_SIZE = 0x82,

    // Bitwise logic
    OP_INVERT = 0x83,
    OP_AND = 0x84,
    OP_OR = 0x85,
    OP_XOR = 0x86,
    OP_EQUAL = 0x87,
    OP_EQUALVERIFY = 0x88,
    OP_RESERVED1 = 0x89,
    OP_RESERVED2 = 0x8a,

    // Arithmetic
    OP_1ADD = 0x8b,
    OP_1SUB = 0x8c,
    OP_2MUL = 0x8d,
    OP_2DIV = 0x8e,
    OP_NEGATE = 0x8f,
    OP_ABS = 0x90,
    OP_NOT = 0x91,
    OP_0NOTEQUAL = 0x92,
    OP_ADD = 0x93,
    OP_SUB = 0x94,
    OP_MUL = 0x95,
    OP_DIV = 0x96,
    OP_MOD = 0x97,
    OP_LSHIFT = 0x98,
    OP_RSHIFT = 0x99,
    OP_BOOLAND = 0x9a,
    OP_BOOLOR = 0x9b,
    OP_NUMEQUAL = 0x9c,
    OP_NUMEQUALVERIFY = 0x9d,
    OP_NUMNOTEQUAL = 0x9e,
    OP_LESSTHAN = 0x9f,
    OP_GREATERTHAN = 0xa0,
    OP_LESSTHANOREQUAL = 0xa1,
    OP_GREATERTHANOREQUAL = 0xa2,
    OP_MIN = 0xa3,
    OP_MAX = 0xa4,
    OP_WITHIN = 0xa5,

    // Crypto
    OP_RIPEMD160 = 0xa6,
    OP_SHA1 = 0xa7,
    OP_SHA256 = 0xa8,
    OP_HASH160 = 0xa9,
    OP_HASH256 = 0xaa,
    OP_CODESEPARATOR = 0xab,
    OP_CHECKSIG = 0xac,
    OP_CHECKSIGVERIFY = 0xad,
    OP_CHECKMULTISIG = 0xae,
    OP_CHECKMULTISIGVERIFY = 0xaf,

    // Locktime
    OP_NOP1 = 0xb0,
    OP_CHECKLOCKTIMEVERIFY = 0xb1,
    OP_CHECKSEQUENCEVERIFY = 0xb2,
    OP_NOP4 = 0xb3,
    OP_NOP5 = 0xb4,
    OP_NOP6 = 0xb5,
    OP_NOP7 = 0xb6,
    OP_NOP8 = 0xb7,
    OP_NOP9 = 0xb8,
    OP_NOP10 = 0xb9,

    // Data push
    PUSH_DATA(u8), // For variable length data pushes
}

impl ScriptOp {
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(ScriptOp::OP_0),
            0x4c => Some(ScriptOp::OP_PUSHDATA1),
            0x4d => Some(ScriptOp::OP_PUSHDATA2),
            0x4e => Some(ScriptOp::OP_PUSHDATA4),
            0x4f => Some(ScriptOp::OP_1NEGATE),
            0x51..=0x60 => Some(ScriptOp::PUSH_DATA(byte)),
            0x61 => Some(ScriptOp::OP_NOP),
            0x63 => Some(ScriptOp::OP_IF),
            0x64 => Some(ScriptOp::OP_NOTIF),
            0x67 => Some(ScriptOp::OP_ELSE),
            0x68 => Some(ScriptOp::OP_ENDIF),
            0x69 => Some(ScriptOp::OP_VERIFY),
            0x6a => Some(ScriptOp::OP_RETURN),
            0x76 => Some(ScriptOp::OP_DUP),
            0x87 => Some(ScriptOp::OP_EQUAL),
            0x88 => Some(ScriptOp::OP_EQUALVERIFY),
            0xa9 => Some(ScriptOp::OP_HASH160),
            0xaa => Some(ScriptOp::OP_HASH256),
            0xac => Some(ScriptOp::OP_CHECKSIG),
            0xae => Some(ScriptOp::OP_CHECKMULTISIG),
            0xb1 => Some(ScriptOp::OP_CHECKLOCKTIMEVERIFY),
            0xb2 => Some(ScriptOp::OP_CHECKSEQUENCEVERIFY),
            _ => None,
        }
    }

    pub fn to_byte(&self) -> u8 {
        match self {
            ScriptOp::OP_0 => 0x00,
            ScriptOp::OP_PUSHDATA1 => 0x4c,
            ScriptOp::OP_PUSHDATA2 => 0x4d,
            ScriptOp::OP_PUSHDATA4 => 0x4e,
            ScriptOp::OP_1NEGATE => 0x4f,
            ScriptOp::OP_1 => 0x51,
            ScriptOp::OP_2 => 0x52,
            ScriptOp::OP_3 => 0x53,
            ScriptOp::OP_4 => 0x54,
            ScriptOp::OP_5 => 0x55,
            ScriptOp::OP_6 => 0x56,
            ScriptOp::OP_7 => 0x57,
            ScriptOp::OP_8 => 0x58,
            ScriptOp::OP_9 => 0x59,
            ScriptOp::OP_10 => 0x5a,
            ScriptOp::OP_11 => 0x5b,
            ScriptOp::OP_12 => 0x5c,
            ScriptOp::OP_13 => 0x5d,
            ScriptOp::OP_14 => 0x5e,
            ScriptOp::OP_15 => 0x5f,
            ScriptOp::OP_16 => 0x60,
            ScriptOp::OP_NOP => 0x61,
            ScriptOp::OP_IF => 0x63,
            ScriptOp::OP_NOTIF => 0x64,
            ScriptOp::OP_ELSE => 0x67,
            ScriptOp::OP_ENDIF => 0x68,
            ScriptOp::OP_VERIFY => 0x69,
            ScriptOp::OP_RETURN => 0x6a,
            ScriptOp::OP_TOALTSTACK => 0x6b,
            ScriptOp::OP_FROMALTSTACK => 0x6c,
            ScriptOp::OP_2DROP => 0x6d,
            ScriptOp::OP_2DUP => 0x6e,
            ScriptOp::OP_3DUP => 0x6f,
            ScriptOp::OP_2OVER => 0x70,
            ScriptOp::OP_2ROT => 0x71,
            ScriptOp::OP_2SWAP => 0x72,
            ScriptOp::OP_IFDUP => 0x73,
            ScriptOp::OP_DEPTH => 0x74,
            ScriptOp::OP_DROP => 0x75,
            ScriptOp::OP_DUP => 0x76,
            ScriptOp::OP_NIP => 0x77,
            ScriptOp::OP_OVER => 0x78,
            ScriptOp::OP_PICK => 0x79,
            ScriptOp::OP_ROLL => 0x7a,
            ScriptOp::OP_ROT => 0x7b,
            ScriptOp::OP_SWAP => 0x7c,
            ScriptOp::OP_TUCK => 0x7d,
            ScriptOp::OP_CAT => 0x7e,
            ScriptOp::OP_SUBSTR => 0x7f,
            ScriptOp::OP_LEFT => 0x80,
            ScriptOp::OP_RIGHT => 0x81,
            ScriptOp::OP_SIZE => 0x82,
            ScriptOp::OP_INVERT => 0x83,
            ScriptOp::OP_AND => 0x84,
            ScriptOp::OP_OR => 0x85,
            ScriptOp::OP_XOR => 0x86,
            ScriptOp::OP_EQUAL => 0x87,
            ScriptOp::OP_EQUALVERIFY => 0x88,
            ScriptOp::OP_RESERVED1 => 0x89,
            ScriptOp::OP_RESERVED2 => 0x8a,
            ScriptOp::OP_1ADD => 0x8b,
            ScriptOp::OP_1SUB => 0x8c,
            ScriptOp::OP_2MUL => 0x8d,
            ScriptOp::OP_2DIV => 0x8e,
            ScriptOp::OP_NEGATE => 0x8f,
            ScriptOp::OP_ABS => 0x90,
            ScriptOp::OP_NOT => 0x91,
            ScriptOp::OP_0NOTEQUAL => 0x92,
            ScriptOp::OP_ADD => 0x93,
            ScriptOp::OP_SUB => 0x94,
            ScriptOp::OP_MUL => 0x95,
            ScriptOp::OP_DIV => 0x96,
            ScriptOp::OP_MOD => 0x97,
            ScriptOp::OP_LSHIFT => 0x98,
            ScriptOp::OP_RSHIFT => 0x99,
            ScriptOp::OP_BOOLAND => 0x9a,
            ScriptOp::OP_BOOLOR => 0x9b,
            ScriptOp::OP_NUMEQUAL => 0x9c,
            ScriptOp::OP_NUMEQUALVERIFY => 0x9d,
            ScriptOp::OP_NUMNOTEQUAL => 0x9e,
            ScriptOp::OP_LESSTHAN => 0x9f,
            ScriptOp::OP_GREATERTHAN => 0xa0,
            ScriptOp::OP_LESSTHANOREQUAL => 0xa1,
            ScriptOp::OP_GREATERTHANOREQUAL => 0xa2,
            ScriptOp::OP_MIN => 0xa3,
            ScriptOp::OP_MAX => 0xa4,
            ScriptOp::OP_WITHIN => 0xa5,
            ScriptOp::OP_RIPEMD160 => 0xa6,
            ScriptOp::OP_SHA1 => 0xa7,
            ScriptOp::OP_SHA256 => 0xa8,
            ScriptOp::OP_HASH160 => 0xa9,
            ScriptOp::OP_HASH256 => 0xaa,
            ScriptOp::OP_CODESEPARATOR => 0xab,
            ScriptOp::OP_CHECKSIG => 0xac,
            ScriptOp::OP_CHECKSIGVERIFY => 0xad,
            ScriptOp::OP_CHECKMULTISIG => 0xae,
            ScriptOp::OP_CHECKMULTISIGVERIFY => 0xaf,
            ScriptOp::OP_NOP1 => 0xb0,
            ScriptOp::OP_CHECKLOCKTIMEVERIFY => 0xb1,
            ScriptOp::OP_CHECKSEQUENCEVERIFY => 0xb2,
            ScriptOp::OP_NOP4 => 0xb3,
            ScriptOp::OP_NOP5 => 0xb4,
            ScriptOp::OP_NOP6 => 0xb5,
            ScriptOp::OP_NOP7 => 0xb6,
            ScriptOp::OP_NOP8 => 0xb7,
            ScriptOp::OP_NOP9 => 0xb8,
            ScriptOp::OP_NOP10 => 0xb9,
            ScriptOp::PUSH_DATA(byte) => *byte,
        }
    }
}

/// Bitcoin Script - a stack-based programming language
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Script {
    pub ops: Vec<ScriptOp>,
    pub data: Vec<Vec<u8>>, // Data pushed by PUSH_DATA operations
}

impl Script {
    pub fn new() -> Self {
        Self {
            ops: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let mut script = Script::new();
        let mut i = 0;
        
        while i < bytes.len() {
            let byte = bytes[i];
            
            if byte <= 75 {
                // Direct data push (1-75 bytes)
                if i + byte as usize >= bytes.len() {
                    return Err("Invalid script: data push extends beyond script".to_string());
                }
                script.data.push(bytes[i + 1..i + 1 + byte as usize].to_vec());
                script.ops.push(ScriptOp::PUSH_DATA(byte));
                i += 1 + byte as usize;
            } else if byte == ScriptOp::OP_PUSHDATA1.to_byte() {
                // OP_PUSHDATA1
                if i + 1 >= bytes.len() {
                    return Err("Invalid script: OP_PUSHDATA1 without length".to_string());
                }
                let len = bytes[i + 1] as usize;
                if i + 2 + len >= bytes.len() {
                    return Err("Invalid script: OP_PUSHDATA1 data extends beyond script".to_string());
                }
                script.data.push(bytes[i + 2..i + 2 + len].to_vec());
                script.ops.push(ScriptOp::OP_PUSHDATA1);
                i += 2 + len;
            } else {
                // Regular opcode
                if let Some(op) = ScriptOp::from_byte(byte) {
                    script.ops.push(op);
                } else {
                    return Err(format!("Invalid opcode: 0x{:02x}", byte));
                }
                i += 1;
            }
        }
        
        Ok(script)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut data_index = 0;
        
        for op in &self.ops {
            match op {
                ScriptOp::PUSH_DATA(len) => {
                    if *len <= 75 {
                        bytes.push(*len);
                        if data_index < self.data.len() {
                            bytes.extend_from_slice(&self.data[data_index]);
                            data_index += 1;
                        }
                    }
                }
                _ => {
                    bytes.push(op.to_byte());
                }
            }
        }
        
        bytes
    }

    /// Create P2PKH script (Pay to Public Key Hash)
    pub fn p2pkh(pubkey_hash: &[u8]) -> Self {
        let mut script = Script::new();
        script.ops.extend_from_slice(&[
            ScriptOp::OP_DUP,
            ScriptOp::OP_HASH160,
            ScriptOp::OP_EQUALVERIFY,
            ScriptOp::OP_CHECKSIG,
        ]);
        script.data.push(pubkey_hash.to_vec());
        script
    }

    /// Create P2SH script (Pay to Script Hash)
    pub fn p2sh(script_hash: &[u8]) -> Self {
        let mut script = Script::new();
        script.ops.extend_from_slice(&[
            ScriptOp::OP_HASH160,
            ScriptOp::OP_EQUAL,
        ]);
        script.data.push(script_hash.to_vec());
        script
    }

    /// Create P2WPKH script (Pay to Witness Public Key Hash)
    pub fn p2wpkh(pubkey_hash: &[u8]) -> Self {
        let mut script = Script::new();
        script.ops.extend_from_slice(&[
            ScriptOp::OP_0,
            ScriptOp::OP_PUSHDATA1,
        ]);
        script.data.push(pubkey_hash.to_vec());
        script
    }

    /// Create P2WSH script (Pay to Witness Script Hash)
    pub fn p2wsh(script_hash: &[u8]) -> Self {
        let mut script = Script::new();
        script.ops.extend_from_slice(&[
            ScriptOp::OP_0,
            ScriptOp::OP_PUSHDATA1,
        ]);
        script.data.push(script_hash.to_vec());
        script
    }

    /// Create multi-sig script
    pub fn multisig(threshold: u8, public_keys: Vec<PublicKey>) -> Self {
        let mut script = Script::new();
        
        // Add threshold
        script.ops.push(ScriptOp::PUSH_DATA(threshold));
        
        // Add public keys
        for pubkey in public_keys {
            script.ops.push(ScriptOp::PUSH_DATA(33)); // Compressed public key length
            script.data.push(pubkey.to_bytes());
        }
        
        // Add number of public keys
        script.ops.push(ScriptOp::PUSH_DATA(public_keys.len() as u8));
        script.ops.push(ScriptOp::OP_CHECKMULTISIG);
        
        script
    }

    /// Get script type
    pub fn get_type(&self) -> ScriptType {
        if self.ops.len() == 5
            && self.ops[0] == ScriptOp::OP_DUP
            && self.ops[1] == ScriptOp::OP_HASH160
            && self.ops[2] == ScriptOp::OP_PUSHDATA1
            && self.ops[3] == ScriptOp::OP_EQUALVERIFY
            && self.ops[4] == ScriptOp::OP_CHECKSIG
        {
            ScriptType::P2PKH
        } else if self.ops.len() == 3
            && self.ops[0] == ScriptOp::OP_HASH160
            && self.ops[1] == ScriptOp::OP_PUSHDATA1
            && self.ops[2] == ScriptOp::OP_EQUAL
        {
            ScriptType::P2SH
        } else if self.ops.len() == 2
            && self.ops[0] == ScriptOp::OP_0
            && self.ops[1] == ScriptOp::OP_PUSHDATA1
            && self.data[0].len() == 20
        {
            ScriptType::P2WPKH
        } else if self.ops.len() == 2
            && self.ops[0] == ScriptOp::OP_0
            && self.ops[1] == ScriptOp::OP_PUSHDATA1
            && self.data[0].len() == 32
        {
            ScriptType::P2WSH
        } else {
            ScriptType::Other
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptType {
    P2PKH,  // Pay to Public Key Hash
    P2SH,   // Pay to Script Hash
    P2WPKH, // Pay to Witness Public Key Hash
    P2WSH,  // Pay to Witness Script Hash
    Other,
}

/// Script execution engine
pub struct ScriptEngine {
    stack: Vec<Vec<u8>>,
    alt_stack: Vec<Vec<u8>>,
    op_count: u32,
    max_op_count: u32,
}

impl ScriptEngine {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            alt_stack: Vec::new(),
            op_count: 0,
            max_op_count: 201, // Bitcoin's limit
        }
    }

    pub fn execute(&mut self, script: &Script, witness: Option<&[Vec<u8>]>) -> Result<bool, String> {
        self.stack.clear();
        self.alt_stack.clear();
        self.op_count = 0;

        // Push witness data if provided (for SegWit)
        if let Some(witness_data) = witness {
            for data in witness_data.iter().rev() {
                self.stack.push(data.clone());
            }
        }

        let mut data_index = 0;
        
        for op in &script.ops {
            self.op_count += 1;
            if self.op_count > self.max_op_count {
                return Err("Script operation count exceeded".to_string());
            }

            match op {
                ScriptOp::PUSH_DATA(_) => {
                    if data_index < script.data.len() {
                        self.stack.push(script.data[data_index].clone());
                        data_index += 1;
                    }
                }
                ScriptOp::OP_DUP => {
                    if self.stack.is_empty() {
                        return Err("Stack underflow".to_string());
                    }
                    let top = self.stack.last().unwrap().clone();
                    self.stack.push(top);
                }
                ScriptOp::OP_HASH160 => {
                    if self.stack.is_empty() {
                        return Err("Stack underflow".to_string());
                    }
                    let data = self.stack.pop().unwrap();
                    let hash = crate::crypto::hash::ripemd160(&data);
                    self.stack.push(hash.to_vec());
                }
                ScriptOp::OP_HASH256 => {
                    if self.stack.is_empty() {
                        return Err("Stack underflow".to_string());
                    }
                    let data = self.stack.pop().unwrap();
                    let hash = crate::crypto::hash::double_sha256(&data);
                    self.stack.push(hash.to_vec());
                }
                ScriptOp::OP_EQUAL => {
                    if self.stack.len() < 2 {
                        return Err("Stack underflow".to_string());
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = if a == b { 1u8 } else { 0u8 };
                    self.stack.push(vec![result]);
                }
                ScriptOp::OP_EQUALVERIFY => {
                    if self.stack.len() < 2 {
                        return Err("Stack underflow".to_string());
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if a != b {
                        return Err("OP_EQUALVERIFY failed".to_string());
                    }
                }
                ScriptOp::OP_CHECKSIG => {
                    if self.stack.len() < 2 {
                        return Err("Stack underflow".to_string());
                    }
                    let pubkey_bytes = self.stack.pop().unwrap();
                    let sig_bytes = self.stack.pop().unwrap();
                    
                    // Simplified signature verification
                    // In a real implementation, this would verify the signature
                    let result = if pubkey_bytes.len() == 33 && sig_bytes.len() >= 64 { 1u8 } else { 0u8 };
                    self.stack.push(vec![result]);
                }
                ScriptOp::OP_VERIFY => {
                    if self.stack.is_empty() {
                        return Err("Stack underflow".to_string());
                    }
                    let top = self.stack.pop().unwrap();
                    if top.is_empty() || top[0] == 0 {
                        return Err("OP_VERIFY failed".to_string());
                    }
                }
                ScriptOp::OP_RETURN => {
                    return Err("OP_RETURN executed".to_string());
                }
                _ => {
                    // Handle other opcodes as needed
                }
            }
        }

        // Script executed successfully
        Ok(!self.stack.is_empty() && !self.stack.last().unwrap().is_empty() && self.stack.last().unwrap()[0] != 0)
    }
}

/// Standard script templates
pub struct P2PKH;
pub struct P2SH;
pub struct P2WPKH;
pub struct P2WSH;

impl P2PKH {
    pub fn create(pubkey_hash: &[u8]) -> Script {
        Script::p2pkh(pubkey_hash)
    }
}

impl P2SH {
    pub fn create(script_hash: &[u8]) -> Script {
        Script::p2sh(script_hash)
    }
}

impl P2WPKH {
    pub fn create(pubkey_hash: &[u8]) -> Script {
        Script::p2wpkh(pubkey_hash)
    }
}

impl P2WSH {
    pub fn create(script_hash: &[u8]) -> Script {
        Script::p2wsh(script_hash)
    }
}

impl Default for Script {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keys::PublicKey;

    #[test]
    fn test_script_creation() {
        let script = Script::new();
        assert!(script.ops.is_empty());
        assert!(script.data.is_empty());
    }

    #[test]
    fn test_script_p2pkh() {
        let pubkey_hash = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let script = Script::p2pkh(&pubkey_hash);
        
        assert_eq!(script.ops.len(), 4);
        assert_eq!(script.data.len(), 1);
        assert_eq!(script.data[0], pubkey_hash);
        assert_eq!(script.get_type(), ScriptType::P2PKH);
    }

    #[test]
    fn test_script_execution() {
        let mut engine = ScriptEngine::new();
        let script = Script::new();
        
        let result = engine.execute(&script, None);
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Empty script should return false
    }

    #[test]
    fn test_script_from_bytes() {
        let bytes = vec![0x76, 0xa9, 0x14, 0x88, 0xac]; // OP_DUP OP_HASH160 OP_PUSHDATA1 OP_EQUALVERIFY OP_CHECKSIG
        let script = Script::from_bytes(&bytes);
        assert!(script.is_ok());
        
        let script = script.unwrap();
        assert_eq!(script.ops.len(), 5);
        assert_eq!(script.ops[0], ScriptOp::OP_DUP);
        assert_eq!(script.ops[1], ScriptOp::OP_HASH160);
        assert_eq!(script.ops[2], ScriptOp::OP_PUSHDATA1);
        assert_eq!(script.ops[3], ScriptOp::OP_EQUALVERIFY);
        assert_eq!(script.ops[4], ScriptOp::OP_CHECKSIG);
    }
}
