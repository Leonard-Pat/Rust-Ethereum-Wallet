// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

mod ethereum;
use ethereum::address;
use ethereum::connect;
use ethereum::wallet::Wallet;

fn main() {
    address::create_mnemonic();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_wallet, get_balance])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

////////////////////////////////////////////////////
//                 tauri commands                //
///////////////////////////////////////////////////

#[tauri::command]
async fn create_wallet() -> Wallet {
    let (secret_key, pub_key) = address::create_keypair();
    let crypto_wallet = Wallet::new(&secret_key, &pub_key);
    crypto_wallet.save_to_file("wallet.json");
    crypto_wallet
}

#[tauri::command]
async fn get_balance() -> f64 {
    dotenv::dotenv().ok();
    let wallet = Wallet::from_file("wallet.json").unwrap();
    let endpoint = env::var("ALCHEMY_KEY").unwrap();
    let web3_con = connect::establish_web3_connection(&endpoint).await.unwrap();
    let balance = wallet.get_balance(&web3_con).await.unwrap();
    balance
}
