use futures::stream::StreamExt;
use futures::SinkExt;
use reqwest::Client;
use serde_urlencoded;
use std::error::Error;
use std::sync::Arc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio::sync::Mutex;

async fn establish_persistent_http_connection(
    session_id_shared: Arc<Mutex<String>>,
) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let params = [
        ("LS_adapter_set", "DEMO"),
        ("LS_cid", "mgQkwtwdysogQz2BJ4Ji%20kOj2Bg"),
    ];
    let request_url =
        "http://push.lightstreamer.com/lightstreamer/create_session.txt?LS_protocol=TLCP-2.0.0";

    let response = client.post(request_url).form(&params).send().await?;

    if response.status().is_success() {
        let mut stream = response.bytes_stream();

        while let Some(item) = stream.next().await {
            match item {
                Ok(bytes) => {
                    let response_text = String::from_utf8(bytes.to_vec())
                        .expect("Failed to convert bytes to string");
                    if let Some(start) = response_text.find("CONOK,") {
                        if let Some(end) = response_text.find(",50000,5000,*\r\n") {
                            let session_id = &response_text[start + 6..end];
                            println!("Session ID: {}", session_id);
                            let mut session_id_lock = session_id_shared.lock().await;
                            *session_id_lock = session_id.to_string();
                        }
                    } else {
                        println!("New message: {}", response_text);
                    }
                }
                Err(e) => println!("Error while receiving: {:?}", e),
            }
        }
    } else {
        println!("Response was not successful: {}", response.status());
    }

    Ok(())
}

/*
// Establish a persistent WebSocket connection and handle the session creation
async fn establish_persistent_ws_connection(
    session_id_shared: Arc<Mutex<String>>,
) -> Result<(), Box<dyn Error>> {
    let ws_url = "wss://push.lightstreamer.com/lightstreamer";

    let (ws_stream, _) = tokio_tungstenite::connect_async_with_config(
        tokio_tungstenite::tungstenite::protocol::handshake::client::Request::from((ws_url, [("Sec-WebSocket-Protocol", "your-subprotocol")].iter().cloned()))
    ).await.expect("Failed to connect");

    let (mut write, mut read) = ws_stream.split();

    // Session creation parameters
    let params = [
        ("LS_op2", "create_session"),
        ("LS_cid", "mgQkwtwdysogQz2BJ4Ji kOj2Bg"),
        ("LS_adapter_set", "DEMO"),
    ];

    let encoded_params = serde_urlencoded::to_string(&params)?;

    // Send the create session message
    write
        .send(Message::Text(format!("{}\n", encoded_params)))
        .await?;

    // Listen for messages from the server
    while let Some(message) = read.next().await {
        match message? {
            Message::Text(text) => {
                if text.starts_with("CONOK") {
                    let session_info: Vec<&str> = text.split(",").collect();
                    let session_id = session_info.get(1).unwrap_or(&"").to_string();
                    *session_id_shared.lock().await = session_id.clone();
                    println!("Session established with ID: {}", session_id);
                    subscribe_to_channel_ws(session_id, write).await?;
                    break; // Exit after successful subscription
                }
            }
            _ => {}
        }
    }

    Ok(())
}
*/

async fn subscribe_to_channel(session_id: String) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let subscribe_url = "http://push.lightstreamer.com/lightstreamer/bind_session.txt";
    let params = [("LS_session", &session_id)];

    let response = client.post(subscribe_url).form(&params).send().await?;

    if response.status().is_success() {
        println!("Subscription successful!");
    } else {
        println!("Subscription failed: {}", response.status());
    }

    Ok(())
}

// Function to subscribe to a channel using WebSocket
async fn subscribe_to_channel_ws(
    session_id: String,
    mut write: futures::stream::SplitSink<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, tokio_tungstenite::tungstenite::protocol::Message>,
) -> Result<(), Box<dyn Error>> {
    // Example subscription to ITEM1 in MERGE mode from the DEMO adapter set
    let sub_params = [
        ("LS_table", "1"),
        ("LS_op2", "add"),
        ("LS_session", &session_id),
        ("LS_id", "item1"),
        ("LS_schema", "stock_name last_price"),
        ("LS_mode", "MERGE"),
    ];

    let encoded_sub_params = serde_urlencoded::to_string(&sub_params)?;

    // Send the subscription message
    write
        .send(Message::Text(encoded_sub_params))
        .await?;

    println!("Subscribed to channel with session ID: {}", session_id);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let session_id_shared = Arc::new(Mutex::new(String::new()));
    let session_id_shared_clone = session_id_shared.clone();

    let task1 = tokio::spawn(async move {
        establish_persistent_http_connection(session_id_shared_clone).await.unwrap();
    });

    println!("Established connection to Lightstreamer server");
    let task2 = tokio::spawn(async move {
        let mut session_established = false;
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            let session_id;
            {
                session_id = session_id_shared.lock().await.clone();
            }

            if !session_established && !session_id.is_empty() {
                println!("Accessed Session ID from another thread: {}", session_id);
                session_established = true;
                subscribe_to_channel(session_id).await.unwrap();
            }
        }
    });

    task1.await?;
    task2.await?;

    Ok(())
}