
use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoin::{block, key::{KeyPair, PrivateKey, XOnlyPublicKey}};
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use bitcoincore_rpc::bitcoin::{ Address, Network};

fn main() {
    let rpc = Client::new("http://localhost:18443",
                          Auth::UserPass("mihir123".to_string(),
                                         "mihir@123".to_string())).unwrap();
    
    match rpc.get_blockchain_info() {
       Ok(blockchain_info) => {
           println!("Blockchain Info: {:?}", blockchain_info);
       }
       Err(e) => {
           eprintln!("Error: {:?}", e);
       }
    }

    // // Derive a taproot address from the key
    let sk = bitcoin::secp256k1::SecretKey::from_slice(&[0xcd; 32]).expect("32 bytes, within curve order");
    let s = bitcoin::key::secp256k1::Secp256k1::new();
    let private_key = PrivateKey::new(sk, bitcoin::Network::Regtest);
    let public_key = PrivateKey::public_key(&private_key, &s);

    // println!("Public key: {}", public_key);
    
    let secp = bitcoincore_rpc::bitcoin::secp256k1::Secp256k1::new();
    let sk2 = bitcoincore_rpc::bitcoin::secp256k1::SecretKey::from_slice(&[0xcd; 32]).unwrap();
    let keypair = bitcoincore_rpc::bitcoin::secp256k1::Keypair::from_secret_key(&secp, &sk2);
    let (x_only_pubkey, _parity) = bitcoincore_rpc::bitcoin::key::XOnlyPublicKey::from_keypair(&keypair);

    // println!("{}", x_only_pubkey);

    let secp = bitcoincore_rpc::bitcoin::secp256k1::Secp256k1::new();

    // // // let p2tr_addr = bitcoincore_rpc::bitcoin::Address::p2tr(&secp, x_only_pubkey,None, bitcoincore_rpc::bitcoin::Network::Regtest);
    let p2tr_addr = Address::p2tr(&secp, x_only_pubkey, None, Network::Regtest);

    // println!("pay to taproot: {}", p2tr_addr);

    let wallet = rpc.create_wallet("p2tr_wallet", Some(false), Some(false), Some("hello world"), Some(true));
    match wallet {
        Ok(wallet_id) => {
          println!("Wallet created successfully! Wallet id: {:?}", wallet_id);
        }
        Err(e) => println!("Failed to create wallet!: {}", e),
      }

    // // // mine 101 blocks
    let mined_blocks = rpc.generate_to_address(101, &p2tr_addr).expect("error");
    let first_block_hash = mined_blocks.get(0);
    let first_block = rpc.get_block(&mined_blocks.get(0).unwrap()).unwrap();
    let transaction = first_block.txdata.get(0).expect("No transaction in block");
    let txid = transaction.txid();

    println!("{:?}", txid);

    // println!("First block: {:?}", first_block);
    // println!("First block hash: {:?}", first_block_hash);

    let secp2 = bitcoincore_rpc::bitcoin::secp256k1::Secp256k1::new();
    let sk3 = bitcoincore_rpc::bitcoin::secp256k1::SecretKey::from_slice(&[0xfd; 32]).unwrap();
    let keypair2 = bitcoincore_rpc::bitcoin::secp256k1::Keypair::from_secret_key(&secp2, &sk3);
    let (x_only_pubkey_2, _parity) = bitcoincore_rpc::bitcoin::key::XOnlyPublicKey::from_keypair(&keypair2);

    let recipient_addr = Address::p2tr(&secp2, x_only_pubkey_2, None, Network::Regtest);

    let utxo = bitcoincore_rpc_json::CreateRawTransactionInput {
        txid: txid,
        vout: 1,
        sequence: Some(0)
    };


    // let raw_txn = rpc.create_raw_transaction(&[utxo], )

    // let info = rpc.get_address_info(&p2tr_addr).unwrap();
    // println!("{:?}", info);
    
    // let block_hash = rpc.get_best_block_hash().unwrap();
    // let block = rpc.get_block(&block_hash).unwrap();

    // println!("{:?}", block);

    // let balance = rpc.get_balance(None, Some(true));
    // println!("wallet balance: {:?}", balance);

    
}
