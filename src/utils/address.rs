use std::fmt;
use thiserror::Error;

const FRACTIS_PREFIX: &str = "fractis";
const SOLANA_ADDRESS_LENGTH: usize = 44;

#[derive(Error, Debug)]
pub enum AddressError {
    #[error("Invalid Solana address: {0}")]
    InvalidSolanaAddress(String),
    #[error("Invalid FRACTIS address: {0}")]
    InvalidFRACTISAddress(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FRACTISAddress(String);

impl FRACTISAddress {
    
    pub fn from_solana(solana_address: &str) -> Result<Self, AddressError> {
       
        if solana_address.len() != SOLANA_ADDRESS_LENGTH || 
           !solana_address.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(AddressError::InvalidSolanaAddress(
                format!("Invalid Solana address format: must be {} characters long and alphanumeric", 
                       SOLANA_ADDRESS_LENGTH)
            ));
        }

        
        let mut hashes = Vec::new();
        let mut prev_hash = solana_address.to_string();

        
        for i in 0..4 {
            let mut hash: u128 = 5381; // DJB2 初始值
            let input = format!("{}{}", prev_hash, i);

           
            for c in input.chars() {
                hash = ((hash << 5).wrapping_add(hash)).wrapping_add(c as u128);
            }

            
            let hash_hex = format!("{:016x}", hash % (1u128 << 64));
            hashes.push(hash_hex);
            prev_hash = hash_hex;
        }

        
        let fractis_addr = format!("{}{}", FRACTIS_PREFIX, hashes.join(""));
        Ok(FRACTISAddress(fractis_addr))
    }

   
    pub fn from_string(fractis_address: &str) -> Result<Self, AddressError> {
        
        if !fractis_address.starts_with(FRACTIS_PREFIX) {
            return Err(AddressError::InvalidFRACTISAddress(
                "Invalid FRACTIS address prefix".to_string()
            ));
        }

       
        let addr_part = &fractis_address[FRACTIS_PREFIX.len()..];
        if addr_part.len() != 64 || !addr_part.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(AddressError::InvalidFRACTISAddress(
                format!("Invalid FRACTIS address format: must be {} characters after prefix and hexadecimal", 64)
            ));
        }

        Ok(FRACTISSAddress(fractis_address.to_string()))
    }

    
    pub fn as_string(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FRACTISAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solana_to_fractis_conversion() {
        
        let solana_addr = "DYw8jCTfwHNRJhhmFcbXvVDTqWMEVFBX6ZKUmG5CNSKK";
        
        
        let fractis_addr = FRACTISAddress::from_solana(solana_addr).unwrap();
        
       
        assert!(fractis_addr.as_string().starts_with(FRACTIS_PREFIX));
        assert_eq!(fractis_addr.as_string().len(), FRACTIS_PREFIX.len() + 64);
    }

    #[test]
    fn test_invalid_solana_address() {
        
        let invalid_length = "DYw8jCTfwHNRJhhmFcbXvVDTqWMEVFBX6ZKUmG5CNS";
        assert!(FRACTISAddress::from_solana(invalid_length).is_err());

        
        let invalid_chars = "DYw8jCTfwHNRJhhmFcbXvVDTqWMEVFBX6ZKUmG5CNSKK!@";
        assert!(FRACTISAddress::from_solana(invalid_chars).is_err());
    }

    #[test]
    fn test_fractis_address_from_string() {
        
        let fractis_addr_str = format!("{}{}",
            FRACTIS_PREFIX,
            "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
        );
        
        
        let result = FRACTISAddress::from_string(&fractis_addr_str);
        assert!(result.is_ok());
        
        
        let invalid_prefix = "xx1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        assert!(FRACTISAddress::from_string(invalid_prefix).is_err());

        
        let invalid_length = format!("{}{}",
            FRACTIS_PREFIX,
            "1234567890abcdef1234567890abcdef"
        );
        assert!(FRACTISAddress::from_string(&invalid_length).is_err());

        
        let invalid_chars = format!("{}{}",
            FRACTIS_PREFIX,
            "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdeg"
        );
        assert!(FRACTISAddress::from_string(&invalid_chars).is_err());
    }

    #[test]
    fn test_hash_consistency() {
        let solana_addr = "DYw8jCTfwHNRJhhmFcbXvVDTqWMEVFBX6ZKUmG5CNSKK";
        let addr1 = FRACTISAddress::from_solana(solana_addr).unwrap();
        let addr2 = FRACTISAddress::from_solana(solana_addr).unwrap();
        assert_eq!(addr1, addr2);
    }
}
