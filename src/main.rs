use futures::stream::StreamExt;
use reqwest::Client;
use std::error::Error;
use std::sync::{Arc, Mutex};

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
                            let mut session_id_lock = session_id_shared.lock().unwrap();
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
                session_id = session_id_shared.lock().unwrap().clone();
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
