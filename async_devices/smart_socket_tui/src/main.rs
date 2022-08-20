use smart_socket_client::SocketClient;
use state::{MainState, State};
use tokio::fs;

mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = get_server_addr().await;
    let mut client = SocketClient::new(addr).await?;

    let mut state: Box<dyn State> = Box::new(MainState);
    while !state.exit() {
        state = state.update(&mut client).await?;
    }

    Ok(())
}

async fn get_server_addr() -> String {
    fs::read_to_string("settings/addr")
        .await
        .unwrap_or_else(|_| String::from("127.0.0.1:55331"))
}
