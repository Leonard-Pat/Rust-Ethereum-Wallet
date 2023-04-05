// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::path::Path;

mod ethereum;
use ethereum::hd_tree::{self, AllowedWordCount};
use ethereum::seed::Seed;
// use ethereum::connect;
// use ethereum::wallet::Wallet;

pub const WALLET_FILE_PATH: &'static Path = Path::new("../../wallets");
fn main() {
    hd_tree::full_flow();

    let my_wallet = Seed::new(AllowedWordCount::Words12, Option::None);
    println!("{:?}", my_wallet);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

////////////////////////////////////////////////////
//                 tauri commands                //
///////////////////////////////////////////////////

// #[tauri::command]
// async fn create_wallet() -> Wallet {
//     // let (secret_key, pub_key) = address::create_keypair();
//     // let crypto_wallet = Wallet::new(&secret_key, &pub_key);
//     // crypto_wallet.save_to_file("wallet.json");
//     // crypto_wallet
// }

// #[tauri::command]
// async fn get_balance() -> f64 {
//     dotenv::dotenv().ok();
//     let wallet = Wallet::from_file("wallet.json").unwrap();
//     let endpoint = env::var("ALCHEMY_KEY").unwrap();
//     let web3_con = connect::establish_web3_connection(&endpoint).await.unwrap();
//     let balance = wallet.get_balance(&web3_con).await.unwrap();
//     balance
// }
