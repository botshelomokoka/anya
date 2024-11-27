//! Module documentation for $moduleName
//!
//! # Overview
//! This module is part of the Anya Core project, located at $modulePath.
//!
//! # Architecture
//! [Add module-specific architecture details]
//!
//! # API Reference
//! [Document public functions and types]
//!
//! # Usage Examples
//! `ust
//! // Add usage examples
//! `
//!
//! # Error Handling
//! This module uses proper error handling with Result types.
//!
//! # Security Considerations
//! [Document security features and considerations]
//!
//! # Performance
//! [Document performance characteristics]

use bitcoincore_rpc::{Auth, Client, RpcApi};
use dlc_btc_lib::{Dlc}; // Add other necessary imports here
use std::error::Error;

pub struct BitcoinRPC {
    client: Client,
}

impl BitcoinRPC {
    pub fn new(
        url: &str, 
        username: &str, 
        password: &str
    ) -> Result<Self, Box<dyn Error>> {
        let auth = Auth::UserPass(username.to_string(), password.to_string());
        let client = Client::new(url, auth)?;
        Ok(Self { client })
    }

    pub fn send_transaction(
        Ok(self.client.get_balance(None, None)?.to_btc())
    }

    pub fn send_transaction(&self, address: &str, amount: f64) -> Result<String, Box<dyn Error>> {
        let txid = self.client.send_to_address(
            &address.parse()?,
            amount.into(),
            None, // comment
            None, // comment
            None, // comment
            None, // comment
            None, // comment
            None  // comment
        )?;
        Ok(txid.to_string())
    }       None,
            None
        )?;
        Ok(txid.to_string())
    }

    // Add more RPC methods as needed
}

