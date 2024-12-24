mod gen_words;
mod pre_proc;
mod words;

use bip39::{Mnemonic, Seed};
use bitcoin::PublicKey;
use bitcoin::bip32::{DerivationPath, Xpriv, Xpub};
use bitcoin::hashes::Hash;
use bitcoin::hashes::hash160;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::{Address, Network};
use std::str::FromStr;

#[allow(unused)]
mod prelude {
    pub use crate::gen_words::gen_words_12;
    pub use crate::gen_words::gen_words_24;
    pub use crate::gen_words::words_12_to_str;
    pub use crate::pre_proc::generate;
    pub use crate::words::WORDS;
}

use crate::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // generate();
    // let x = gen_words_12();
    // println!("{:?}", x);
    let mut buffer = String::new();
    let mnemonic_phrase = words_12_to_str(gen_words_12(), &mut buffer);

    let mnemonic = Mnemonic::from_phrase(mnemonic_phrase, bip39::Language::English)?;

    let seed = Seed::new(&mnemonic, "");

    let network = Network::Bitcoin;

    let secp = Secp256k1::new();
    let master_key = Xpriv::new_master(network, &seed.as_bytes())?;

    let derivation_path = DerivationPath::from_str("m/44'/0'/0'/0/0")?;
    let derived_key = master_key.derive_priv(&secp, &derivation_path)?;

    let public_key = derived_key.private_key.public_key(&secp);

    let secp_pub_key = public_key.x_only_public_key();
    let pub_key_hash = secp_pub_key.0.to_bytes();

    let address = Address::p2pkh(&public_key, network);

    println!("Endereço público: {}", address);

    Ok(())
}
