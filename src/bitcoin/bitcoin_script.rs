use bitcoin::blockdata::script::Builder;
use bitcoin::blockdata::opcodes::all::{OP_CHECKSIG, OP_DUP, OP_HASH160, OP_EQUALVERIFY};
use bitcoin::util::address::Address;
use bitcoin::network::constants::Network;
use bitcoin::secp256k1::PublicKey;

pub struct BitcoinScriptModule;

impl BitcoinScriptModule {
    pub fn new() -> Self {
        Self
    }

    pub fn create_p2pkh_script(&self, pubkey: &PublicKey) -> Builder {
        let pubkey_hash = Address::p2pkh(pubkey, Network::Bitcoin).script_pubkey();
        Builder::new()
            .push_opcode(OP_DUP)
            .push_opcode(OP_HASH160)
            .push_slice(&pubkey_hash[..])
            .push_opcode(OP_EQUALVERIFY)
            .push_opcode(OP_CHECKSIG)
    }

    pub fn create_multisig_script(&self, pubkeys: &[PublicKey], required_sigs: usize) -> Builder {
        let mut builder = Builder::new().push_int(required_sigs as i64);
        for pubkey in pubkeys {
            builder = builder.push_slice(&pubkey.serialize());
        }
        builder.push_int(pubkeys.len() as i64).push_opcode(OP_CHECKSIG)
    }

    pub fn create_timelock_script(&self, lock_time: u32, pubkey: &PublicKey) -> Builder {
        Builder::new()
            .push_int(lock_time as i64)
            .push_opcode(OP_CHECKSIG)
            .push_slice(&pubkey.serialize())
    }
}