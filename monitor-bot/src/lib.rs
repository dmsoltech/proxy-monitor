use monitor::Monitor;
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config;
use solana_client::rpc_client::SerializableTransaction;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::VersionedTransaction;
use solana_sdk::signer::SignerError;
use solana_sdk::instruction::Instruction;
use solana_sdk::hash::Hash;
use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::system_instruction::transfer;
use solana_sdk::signature::Signer;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::signature;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::compute_budget::ComputeBudgetInstruction;

use solana_entry::entry::Entry;
use solana_transaction_status::UiTransactionEncoding;
use spl_token::instruction::close_account;
use spl_token::instruction::sync_native;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicU64;
use std::time::Instant;
use std::sync::atomic::AtomicBool;
use spl_associated_token_account::get_associated_token_address;
use spl_associated_token_account::instruction::create_associated_token_account;
use solana_client::rpc_client::RpcClient;
use std::env;
use std::fs;
use std::sync::Arc;
use serde::{Deserialize};
use jito_protos::shredstream::{
    shredstream_proxy_client::ShredstreamProxyClient, SubscribeEntriesRequest,
};
use tokio::runtime::Runtime;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono;

pub mod monitor;

pub struct PumpBot {
    pub monitor: Monitor,
    pub enabled: AtomicBool,
}
impl PumpBot {
    pub const SHREDSTREAM_ENDPOINT: &str = "http://127.0.0.1:9999";
    pub const PUMP_PID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
    pub const PUMP_AMM_PID: &str = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA";
    pub const RAY_PID: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
    pub const RAY_LAUNCH_PID: &str = "LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj";
    

    pub fn create() -> Self {
        
        Self {
            monitor: Monitor::create(),
            enabled: AtomicBool::new(true),
            
        }
    }
    
    pub fn process_entries(&self, entries: Vec<Entry>, slot: u64, start: u32, end: u32) {
        if self.enabled.load(Ordering::SeqCst) {

            for entry in entries.iter() {
                for tx in entry.transactions.iter() {
                    let static_keys = tx.message.static_account_keys();
                    
                    for ix in tx.message.instructions() {
                        let pid: &Pubkey = &static_keys[ix.program_id_index as usize];
                        let ix_data = &ix.data;
                        let res = self.monitor.detect_swap(pid, ix_data);
                        if res.is_some() {
                            let swap_result = res.unwrap();
                            println!("");
                            println!("slot: {}, swap_result: {:#?} ", slot, swap_result);
                        }
                    }
                
                }
            }

        }
    }
    
    pub fn receive_entry_update(bot_param: Arc<PumpBot>) {
        let bot = Arc::clone(&bot_param);
        std::thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            let result = rt.block_on(
                PumpBot::run_entry_update(bot)
            );
        });
        
    }
    pub async fn run_entry_update(bot: Arc<PumpBot>) {
        println!("bot started!");
        let mut client = ShredstreamProxyClient::connect(PumpBot::SHREDSTREAM_ENDPOINT)
            .await
            .unwrap();
        let mut stream = client
            .subscribe_entries(SubscribeEntriesRequest {})
            .await
            .unwrap()
            .into_inner();
    
        while let Some(slot_entry) = stream.message().await.unwrap() {
            // let start_deserialize = Instant::now();

            let entries =
                match bincode::deserialize::<Vec<solana_entry::entry::Entry>>(&slot_entry.entries) {
                    Ok(e) => e,
                    Err(e) => {
                        println!("Deserialization failed with err: {e}");
                        continue;
                    }
                };
            // let duration_deserialize = start_deserialize.elapsed();
            // println!("Time taken deserialize: {}ms", duration_deserialize.as_millis());

            let bot_clone:  Arc<PumpBot> = Arc::clone(&bot);
            let slot = slot_entry.slot as u64;

            

            std::thread::spawn(move || {
                bot_clone.process_entries(entries, slot, 0, 0);
            });
            
            // let duration = start.elapsed();
            // println!("Time taken: {}ms", duration.as_millis());
        }
    }
}

