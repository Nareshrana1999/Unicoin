use crate::crypto::hash::Hash;
use crate::blockchain::script::Script;
use crate::blockchain::transaction::{Transaction, TransactionInput, TransactionOutput};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Witness data for SegWit transactions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Witness {
    /// Witness stack items
    pub items: Vec<Vec<u8>>,
}

impl Witness {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn from_items(items: Vec<Vec<u8>>) -> Self {
        Self { items }
    }

    pub fn add_item(&mut self, item: Vec<u8>) {
        self.items.push(item);
    }

    pub fn get_item(&self, index: usize) -> Option<&Vec<u8>> {
        self.items.get(index)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Serialize witness to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        for item in &self.items {
            // Encode length (compact size)
            if item.len() < 253 {
                bytes.push(item.len() as u8);
            } else if item.len() < 0x10000 {
                bytes.push(253);
                bytes.extend_from_slice(&(item.len() as u16).to_le_bytes());
            } else if item.len() < 0x100000000 {
                bytes.push(254);
                bytes.extend_from_slice(&(item.len() as u32).to_le_bytes());
            } else {
                bytes.push(255);
                bytes.extend_from_slice(&(item.len() as u64).to_le_bytes());
            }
            
            bytes.extend_from_slice(item);
        }
        
        bytes
    }

    /// Deserialize witness from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let mut witness = Witness::new();
        let mut i = 0;
        
        while i < bytes.len() {
            let (len, consumed) = Self::read_compact_size(&bytes[i..])?;
            i += consumed;
            
            if i + len > bytes.len() {
                return Err("Invalid witness: item extends beyond data".to_string());
            }
            
            witness.items.push(bytes[i..i + len].to_vec());
            i += len;
        }
        
        Ok(witness)
    }

    fn read_compact_size(bytes: &[u8]) -> Result<(usize, usize), String> {
        if bytes.is_empty() {
            return Err("Invalid compact size: empty data".to_string());
        }
        
        match bytes[0] {
            0..=252 => Ok((bytes[0] as usize, 1)),
            253 => {
                if bytes.len() < 3 {
                    return Err("Invalid compact size: insufficient data for u16".to_string());
                }
                let len = u16::from_le_bytes([bytes[1], bytes[2]]) as usize;
                Ok((len, 3))
            }
            254 => {
                if bytes.len() < 5 {
                    return Err("Invalid compact size: insufficient data for u32".to_string());
                }
                let len = u32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]) as usize;
                Ok((len, 5))
            }
            255 => {
                if bytes.len() < 9 {
                    return Err("Invalid compact size: insufficient data for u64".to_string());
                }
                let len = u64::from_le_bytes([
                    bytes[1], bytes[2], bytes[3], bytes[4],
                    bytes[5], bytes[6], bytes[7], bytes[8],
                ]) as usize;
                Ok((len, 9))
            }
            _ => Err("Invalid compact size prefix".to_string()),
        }
    }
}

impl Default for Witness {
    fn default() -> Self {
        Self::new()
    }
}

/// SegWit transaction with witness data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SegWitTransaction {
    /// Transaction version
    pub version: i32,
    /// Marker and flag for SegWit
    pub marker: u8, // Always 0x00
    pub flag: u8,   // Always 0x01
    /// Transaction inputs
    pub inputs: Vec<TransactionInput>,
    /// Transaction outputs
    pub outputs: Vec<TransactionOutput>,
    /// Witness data for each input
    pub witness: Vec<Witness>,
    /// Lock time
    pub lock_time: u32,
    /// Transaction ID (hash without witness data)
    pub tx_id: Hash,
    /// Witness transaction ID (hash including witness data)
    pub witness_tx_id: Hash,
}

impl SegWitTransaction {
    pub fn new(
        version: i32,
        inputs: Vec<TransactionInput>,
        outputs: Vec<TransactionOutput>,
        witness: Vec<Witness>,
        lock_time: u32,
    ) -> Self {
        let mut tx = Self {
            version,
            marker: 0x00,
            flag: 0x01,
            inputs,
            outputs,
            witness,
            lock_time,
            tx_id: Hash::new([0u8; 32]),
            witness_tx_id: Hash::new([0u8; 32]),
        };
        
        tx.tx_id = tx.calculate_tx_id();
        tx.witness_tx_id = tx.calculate_witness_tx_id();
        
        tx
    }

    /// Calculate transaction ID (without witness data)
    pub fn calculate_tx_id(&self) -> Hash {
        let mut data = Vec::new();
        
        // Version
        data.extend_from_slice(&self.version.to_le_bytes());
        
        // Input count
        data.extend_from_slice(&self.inputs.len().to_le_bytes());
        
        // Inputs
        for input in &self.inputs {
            data.extend_from_slice(&input.previous_tx_id.to_bytes());
            data.extend_from_slice(&input.previous_output_index.to_le_bytes());
            data.extend_from_slice(&input.script_sig.to_bytes());
            data.extend_from_slice(&input.sequence.to_le_bytes());
        }
        
        // Output count
        data.extend_from_slice(&self.outputs.len().to_le_bytes());
        
        // Outputs
        for output in &self.outputs {
            data.extend_from_slice(&output.amount.to_le_bytes());
            data.extend_from_slice(&output.script_pubkey.to_bytes());
        }
        
        // Lock time
        data.extend_from_slice(&self.lock_time.to_le_bytes());
        
        Hash::sha256(&data)
    }

    /// Calculate witness transaction ID (including witness data)
    pub fn calculate_witness_tx_id(&self) -> Hash {
        let mut data = Vec::new();
        
        // Version
        data.extend_from_slice(&self.version.to_le_bytes());
        
        // Marker and flag
        data.push(self.marker);
        data.push(self.flag);
        
        // Input count
        data.extend_from_slice(&self.inputs.len().to_le_bytes());
        
        // Inputs
        for input in &self.inputs {
            data.extend_from_slice(&input.previous_tx_id.to_bytes());
            data.extend_from_slice(&input.previous_output_index.to_le_bytes());
            data.extend_from_slice(&input.script_sig.to_bytes());
            data.extend_from_slice(&input.sequence.to_le_bytes());
        }
        
        // Output count
        data.extend_from_slice(&self.outputs.len().to_le_bytes());
        
        // Outputs
        for output in &self.outputs {
            data.extend_from_slice(&output.amount.to_le_bytes());
            data.extend_from_slice(&output.script_pubkey.to_bytes());
        }
        
        // Witness data
        for witness in &self.witness {
            data.extend_from_slice(&witness.to_bytes());
        }
        
        // Lock time
        data.extend_from_slice(&self.lock_time.to_le_bytes());
        
        Hash::sha256(&data)
    }

    /// Calculate transaction weight (used for fee calculation)
    pub fn calculate_weight(&self) -> u64 {
        let base_size = self.get_base_size();
        let witness_size = self.get_witness_size();
        
        // Weight = (base_size * 3) + total_size
        (base_size * 3) + (base_size + witness_size)
    }

    /// Get base transaction size (without witness data)
    pub fn get_base_size(&self) -> u64 {
        let mut size = 4; // version
        
        size += 1; // marker
        size += 1; // flag
        
        // Input count
        size += 1;
        
        // Inputs
        for input in &self.inputs {
            size += 32; // previous_tx_id
            size += 4;  // previous_output_index
            size += 1;  // script_sig length
            size += input.script_sig.to_bytes().len() as u64;
            size += 4;  // sequence
        }
        
        // Output count
        size += 1;
        
        // Outputs
        for output in &self.outputs {
            size += 8;  // amount
            size += 1;  // script_pubkey length
            size += output.script_pubkey.to_bytes().len() as u64;
        }
        
        // Witness count
        size += 1;
        
        size += 4; // lock_time
        
        size
    }

    /// Get witness data size
    pub fn get_witness_size(&self) -> u64 {
        let mut size = 0;
        
        for witness in &self.witness {
            size += witness.to_bytes().len() as u64;
        }
        
        size
    }

    /// Calculate virtual size (used for fee calculation)
    pub fn get_virtual_size(&self) -> u64 {
        self.calculate_weight() / 4
    }

    /// Check if this is a SegWit transaction
    pub fn is_segwit(&self) -> bool {
        self.marker == 0x00 && self.flag == 0x01
    }

    /// Convert to regular transaction (removes witness data)
    pub fn to_regular_transaction(&self) -> Transaction {
        Transaction::new(
            self.version,
            self.inputs.clone(),
            self.outputs.clone(),
            self.lock_time,
        )
    }
}

/// SegWit validator for transaction verification
pub struct SegWitValidator {
    /// UTXO set for validation
    utxo_set: HashMap<(Hash, u32), TransactionOutput>,
}

impl SegWitValidator {
    pub fn new() -> Self {
        Self {
            utxo_set: HashMap::new(),
        }
    }

    pub fn add_utxo(&mut self, tx_id: Hash, output_index: u32, output: TransactionOutput) {
        self.utxo_set.insert((tx_id, output_index), output);
    }

    pub fn remove_utxo(&mut self, tx_id: Hash, output_index: u32) {
        self.utxo_set.remove(&(tx_id, output_index));
    }

    /// Validate SegWit transaction
    pub fn validate(&self, tx: &SegWitTransaction) -> Result<(), String> {
        // Check marker and flag
        if !tx.is_segwit() {
            return Err("Invalid SegWit marker/flag".to_string());
        }

        // Check witness count matches input count
        if tx.witness.len() != tx.inputs.len() {
            return Err("Witness count doesn't match input count".to_string());
        }

        // Validate each input
        for (i, input) in tx.inputs.iter().enumerate() {
            let witness = &tx.witness[i];
            
            // Get the previous output
            let prev_output = self.utxo_set.get(&(input.previous_tx_id, input.previous_output_index))
                .ok_or_else(|| "Previous output not found".to_string())?;

            // Validate witness against script
            self.validate_witness(witness, &prev_output.script_pubkey, tx, i)?;
        }

        Ok(())
    }

    /// Validate witness data against script
    fn validate_witness(
        &self,
        witness: &Witness,
        script_pubkey: &Script,
        tx: &SegWitTransaction,
        input_index: usize,
    ) -> Result<(), String> {
        match script_pubkey.get_type() {
            crate::blockchain::script::ScriptType::P2WPKH => {
                self.validate_p2wpkh(witness, script_pubkey, tx, input_index)?;
            }
            crate::blockchain::script::ScriptType::P2WSH => {
                self.validate_p2wsh(witness, script_pubkey, tx, input_index)?;
            }
            _ => {
                return Err("Invalid script type for SegWit".to_string());
            }
        }

        Ok(())
    }

    /// Validate P2WPKH witness
    fn validate_p2wpkh(
        &self,
        witness: &Witness,
        script_pubkey: &Script,
        tx: &SegWitTransaction,
        input_index: usize,
    ) -> Result<(), String> {
        if witness.len() != 2 {
            return Err("P2WPKH witness must have 2 items".to_string());
        }

        let signature = witness.get_item(0).unwrap();
        let pubkey = witness.get_item(1).unwrap();

        // Verify public key hash matches script
        let pubkey_hash = crate::crypto::hash::ripemd160(pubkey);
        let expected_hash = script_pubkey.data[0].clone();
        
        if pubkey_hash.to_vec() != expected_hash {
            return Err("Public key hash mismatch".to_string());
        }

        // Verify signature (simplified)
        if signature.len() < 64 {
            return Err("Invalid signature length".to_string());
        }

        Ok(())
    }

    /// Validate P2WSH witness
    fn validate_p2wsh(
        &self,
        witness: &Witness,
        script_pubkey: &Script,
        tx: &SegWitTransaction,
        input_index: usize,
    ) -> Result<(), String> {
        if witness.is_empty() {
            return Err("P2WSH witness cannot be empty".to_string());
        }

        let witness_script_bytes = witness.get_item(witness.len() - 1).unwrap();
        let witness_script = Script::from_bytes(witness_script_bytes)?;

        // Verify witness script hash matches script
        let script_hash = crate::crypto::hash::sha256(witness_script_bytes);
        let expected_hash = script_pubkey.data[0].clone();
        
        if script_hash.to_vec() != expected_hash {
            return Err("Witness script hash mismatch".to_string());
        }

        // Execute witness script
        let mut engine = crate::blockchain::script::ScriptEngine::new();
        let witness_stack: Vec<Vec<u8>> = witness.items[..witness.len() - 1].to_vec();
        
        let result = engine.execute(&witness_script, Some(&witness_stack))?;
        if !result {
            return Err("Witness script execution failed".to_string());
        }

        Ok(())
    }

    /// Calculate transaction fee
    pub fn calculate_fee(&self, tx: &SegWitTransaction, fee_rate: u64) -> u64 {
        let virtual_size = tx.get_virtual_size();
        virtual_size * fee_rate
    }

    /// Check if transaction is RBF (Replace-By-Fee) eligible
    pub fn is_rbf_eligible(&self, tx: &SegWitTransaction) -> bool {
        // Check if any input has sequence number < 0xffffffff
        tx.inputs.iter().any(|input| input.sequence < 0xffffffff)
    }

    /// Get transaction priority (for mempool ordering)
    pub fn get_priority(&self, tx: &SegWitTransaction) -> f64 {
        let mut total_input_age = 0u64;
        let mut total_input_value = 0u64;

        for input in &tx.inputs {
            if let Some(output) = self.utxo_set.get(&(input.previous_tx_id, input.previous_output_index)) {
                total_input_value += output.amount;
                // Simplified age calculation
                total_input_age += 1;
            }
        }

        if total_input_value == 0 {
            return 0.0;
        }

        (total_input_age as f64 * total_input_value as f64) / tx.get_virtual_size() as f64
    }
}

impl Default for SegWitValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// SegWit version and features
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SegWitVersion {
    V0, // Original SegWit
    V1, // Taproot (future)
}

impl SegWitVersion {
    pub fn from_witness_script(witness_script: &[u8]) -> Option<Self> {
        if witness_script.is_empty() {
            return None;
        }
        
        match witness_script[0] {
            0 => Some(SegWitVersion::V0),
            1 => Some(SegWitVersion::V1),
            _ => None,
        }
    }
}

/// SegWit address encoding
pub struct SegWitAddress;

impl SegWitAddress {
    /// Encode witness program to Bech32 address
    pub fn encode_bech32(hrp: &str, witness_program: &[u8]) -> Result<String, String> {
        // Simplified Bech32 encoding
        // In a real implementation, this would use proper Bech32 encoding
        let mut encoded = hrp.to_string() + "1";
        
        for byte in witness_program {
            encoded.push_str(&format!("{:02x}", byte));
        }
        
        Ok(encoded)
    }

    /// Decode Bech32 address to witness program
    pub fn decode_bech32(bech32: &str) -> Result<(String, Vec<u8>), String> {
        // Simplified Bech32 decoding
        // In a real implementation, this would use proper Bech32 decoding
        if let Some(pos) = bech32.find('1') {
            let hrp = bech32[..pos].to_string();
            let data_part = &bech32[pos + 1..];
            
            let mut witness_program = Vec::new();
            for chunk in data_part.as_bytes().chunks(2) {
                if chunk.len() == 2 {
                    let hex_str = std::str::from_utf8(chunk).unwrap_or("00");
                    if let Ok(byte) = u8::from_str_radix(hex_str, 16) {
                        witness_program.push(byte);
                    }
                }
            }
            
            Ok((hrp, witness_program))
        } else {
            Err("Invalid Bech32 address format".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::script::Script;

    #[test]
    fn test_witness_creation() {
        let witness = Witness::new();
        assert!(witness.is_empty());
        assert_eq!(witness.len(), 0);
    }

    #[test]
    fn test_witness_serialization() {
        let mut witness = Witness::new();
        witness.add_item(vec![1, 2, 3, 4]);
        witness.add_item(vec![5, 6, 7, 8, 9]);
        
        let bytes = witness.to_bytes();
        let deserialized = Witness::from_bytes(&bytes).unwrap();
        
        assert_eq!(witness.items, deserialized.items);
    }

    #[test]
    fn test_segwit_transaction() {
        let inputs = vec![];
        let outputs = vec![];
        let witness = vec![];
        
        let tx = SegWitTransaction::new(1, inputs, outputs, witness, 0);
        
        assert!(tx.is_segwit());
        assert_eq!(tx.marker, 0x00);
        assert_eq!(tx.flag, 0x01);
    }

    #[test]
    fn test_segwit_validator() {
        let validator = SegWitValidator::new();
        assert!(validator.utxo_set.is_empty());
    }

    #[test]
    fn test_bech32_encoding() {
        let hrp = "bc";
        let witness_program = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
        
        let encoded = SegWitAddress::encode_bech32(hrp, &witness_program).unwrap();
        let (decoded_hrp, decoded_program) = SegWitAddress::decode_bech32(&encoded).unwrap();
        
        assert_eq!(hrp, decoded_hrp);
        assert_eq!(witness_program, decoded_program);
    }
}
