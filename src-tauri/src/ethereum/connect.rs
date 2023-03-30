use std::error::Error;
use web3::{
    transports::WebSocket,
    types::{Address, U256},
    Web3,
};

pub async fn establish_web3_connection(url: &str) -> Result<Web3<WebSocket>, Box<dyn Error>> {
    let transport = web3::transports::WebSocket::new(url).await?;
    Ok(web3::Web3::new(transport))
}
