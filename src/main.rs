use hyper::client;
use lightstreamer_client::item_update::ItemUpdate;
use lightstreamer_client::ls_client::{LightstreamerClient, Transport};
use lightstreamer_client::subscription::{Snapshot, Subscription, SubscriptionMode};
use lightstreamer_client::subscription_listener::SubscriptionListener;

use futures::stream::StreamExt;
use futures::SinkExt;
use reqwest::Client;
use serde_urlencoded;
use signal_hook::low_level::signal_name;
use signal_hook::{consts::SIGINT, consts::SIGTERM, iterator::Signals};
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

struct SharedState {
    client: Arc<Mutex<LightstreamerClient>>,
    should_disconnect: Arc<AtomicBool>,
}

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
async fn setup_signal_hook(shared_state: Arc<Mutex<SharedState>>) {
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
            {
                let shared_state = shared_state.lock().await;
                shared_state.should_disconnect.store(true, Ordering::Relaxed);
                let mut client = shared_state.client.lock().await;
                client.disconnect();
            }

            // Exit with 0 code to indicate orderly shutdown.
            std::process::exit(0);
        }
    });
}

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
        client.connection_options.set_forced_transport(Some(Transport::WsStreaming));
    }

    let should_disconnect = Arc::new(AtomicBool::new(false));
    let shared_state = Arc::new(Mutex::new(SharedState {
        client: client.clone(),
        should_disconnect: should_disconnect.clone(),
    }));

    // Spawn a new thread to handle SIGINT and SIGTERM process signals.
    setup_signal_hook(shared_state).await;

    //
    // Infinite loop that will indefinitely retry failed connections unless
    // a SIGTERM or SIGINT signal is received.
    //
    let mut retry_interval_milis: u64 = 0;
    let mut retry_counter: u64 = 0;
    loop {
        let mut client = client.lock().await;
        match client.connect().await {
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
