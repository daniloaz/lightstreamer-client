use hyper::client;
use lightstreamer_client::item_update::ItemUpdate;
use lightstreamer_client::ls_client::LightstreamerClient;
use lightstreamer_client::subscription::{Snapshot, Subscription, SubscriptionMode};
use lightstreamer_client::subscription_listener::SubscriptionListener;

use futures::stream::StreamExt;
use futures::SinkExt;
use reqwest::Client;
use serde_urlencoded;
use signal_hook::low_level::signal_name;
use signal_hook::{consts::SIGINT, consts::SIGTERM, iterator::Signals};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

/// Sets up a signal hook for SIGINT and SIGTERM.
///
/// Creates a signal hook for the specified signals and spawns a thread to handle them.
/// When a signal is received, it logs the signal name and performs cleanup before exiting with 0 code
/// to indicate orderly shutdown.
///
/// # Arguments
///
/// * `full_path` - The full path to the application configuration file.
///
/// # Panics
///
/// The function panics if it fails to create the signal iterator.
///
async fn setup_signal_hook(client: Arc<Mutex<LightstreamerClient>>) {
    // Create a signal set of signals to be handled and a signal iterator to monitor them.
    let signals = &[SIGINT, SIGTERM];
    let mut signals_iterator = Signals::new(signals).expect("Failed to create signal iterator");

    // Create a new thread to handle signals sent to the process
    tokio::spawn(async move {
        for signal in signals_iterator.forever() {
            println!("Received signal: {}", signal_name(signal).unwrap());
            //
            // Clean up and prepare to exit...
            // ...
            let mut client = client.lock().await;
            client.disconnect();

            // Exit with 0 code to indicate orderly shutdown.
            std::process::exit(0);
        }
    });
}

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
    //let subscribe_url = "http://push.lightstreamer.com/lightstreamer/bind_session.txt";
    //let params = [("LS_session", &session_id)];
    let subscribe_url =
        "http://push.lightstreamer.com/lightstreamer/control.txt?LS_protocol=TLCP-2.0.0";
    let params = [
        ("LS_session", &session_id),
        ("LS_op", &"add".to_string()),
        ("LS_subId", &"1".to_string()),
        ("LS_data_adapter", &"CHAT_ROOM".to_string()),
        ("LS_group", &"chat_room".to_string()),
        ("LS_schema", &"timestamp message".to_string()),
        ("LS_mode", &"DISTINCT".to_string()),
        ("LS_reqId", &"1".to_string()),
    ];

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
    mut write: futures::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        tokio_tungstenite::tungstenite::protocol::Message,
    >,
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
    write.send(Message::Text(encoded_sub_params)).await?;

    println!("Subscribed to channel with session ID: {}", session_id);

    Ok(())
}

/*
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
*/

pub struct MySubscriptionListener {}

impl SubscriptionListener for MySubscriptionListener {
    fn on_item_update(&mut self, update: ItemUpdate) {
        println!(
            "UPDATE {} {}",
            update.get_value("stock_name").unwrap(),
            update.get_value("last_price").unwrap()
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //
    // Create a new subscription instance.
    //
    let mut my_subscription = Subscription::new(
        SubscriptionMode::Merge,
        Some(vec![
            "item1".to_string(),
            "item2".to_string(),
            "item3".to_string(),
        ]),
        Some(vec!["stock_name".to_string(), "last_price".to_string()]),
    )?;

    my_subscription.set_data_adapter(Some(String::from("QUOTE_ADAPTER")))?;
    my_subscription.set_requested_snapshot(Some(Snapshot::Yes))?;
    my_subscription.add_listener(Box::new(MySubscriptionListener {}));

    // Create a new Lightstreamer client instance and wrap it in an Arc<Mutex<>> so it can be shared across threads.
    let client = Arc::new(Mutex::new(LightstreamerClient::new(
        Some("http://push.lightstreamer.com/lightstreamer"),
        Some("DEMO"),
    )?));

    //
    // Add the subscription to the client.
    //
    {
        let mut client = client.lock().await;
        client.subscribe(my_subscription);
    }

    // Spawn a new thread to handle SIGINT and SIGTERM process signals.
    setup_signal_hook(client.clone()).await;

    //
    // Infinite loop that will indefinitely retry failed connections unless
    // a SIGTERM or SIGINT signal is received.
    //
    let mut retry_interval_milis: u64 = 0;
    let mut retry_counter: u64 = 0;
    loop {
        let mut client = client.lock().await;
        match client.connect() {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to connect: {:?}", e);
                tokio::time::sleep(std::time::Duration::from_millis(retry_interval_milis)).await;
                retry_interval_milis = (retry_interval_milis + (200 * retry_counter)).min(5000);
                retry_counter += 1;
                println!(
                    "Retrying connection in {} seconds...",
                    format!("{:.2}", retry_interval_milis as f64 / 1000.0)
                );
            }
        }
    }
}
