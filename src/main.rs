use lightstreamer_client::item_update::ItemUpdate;
use lightstreamer_client::ls_client::{LightstreamerClient, Transport};
use lightstreamer_client::subscription::{Snapshot, Subscription, SubscriptionMode};
use lightstreamer_client::subscription_listener::SubscriptionListener;

use signal_hook::low_level::signal_name;
use signal_hook::{consts::SIGINT, consts::SIGTERM, iterator::Signals};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::{Notify, Mutex};

const MAX_CONNECTION_ATTEMPTS: u64 = 1;

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
async fn setup_signal_hook(shutdown_signal: Arc<Notify>) {
    // Create a signal set of signals to be handled and a signal iterator to monitor them.
    let signals = &[SIGINT, SIGTERM];
    let mut signals_iterator = Signals::new(signals).expect("Failed to create signal iterator");

    // Create a new thread to handle signals sent to the process
    tokio::spawn(async move {
        for signal in signals_iterator.forever() {
            println!("Received signal: {}", signal_name(signal).unwrap());
            let _ = shutdown_signal.notify_one();
            break;
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
            "item2".to_string(),
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
        None,
        None,
    )?));

    //
    // Add the subscription to the client.
    //
    {
        let mut client = client.lock().await;
        client.subscribe(my_subscription);
        client.connection_options.set_forced_transport(Some(Transport::WsStreaming));
    }

    // Create a new Notify instance to send a shutdown signal to the signal handler thread.
    let shutdown_signal = Arc::new(tokio::sync::Notify::new());
    // Spawn a new thread to handle SIGINT and SIGTERM process signals.
    setup_signal_hook(Arc::clone(&shutdown_signal)).await;

    //
    // Infinite loop that will indefinitely retry failed connections unless
    // a SIGTERM or SIGINT signal is received.
    //
    let mut retry_interval_milis: u64 = 0;
    let mut retry_counter: u64 = 0;
    while retry_counter < MAX_CONNECTION_ATTEMPTS {
        let mut client = client.lock().await;
        match client.connect(Arc::clone(&shutdown_signal)).await {
            Ok(_) => {
                client.disconnect().await;
                break;
            }
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

    if retry_counter == MAX_CONNECTION_ATTEMPTS {
        println!("Failed to connect after {} retries. Exiting...", retry_counter);
    } else {
        println!("Exiting orderly from Lightstreamer client...");
    }

    // Exit using std::process::exit() to avoid waiting for existing tokio tasks to complete.
    std::process::exit(0);
}
