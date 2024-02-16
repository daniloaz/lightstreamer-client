use reqwest::Client;
use futures::stream::StreamExt; // Actualizado para usar futures::StreamExt
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use std::thread;

async fn establish_persistent_http_connection(session_id_shared: Arc<Mutex<String>>) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let params = [("LS_adapter_set", "DEMO"), ("LS_cid", "mgQkwtwdysogQz2BJ4Ji%20kOj2Bg")];
    let request_url = "http://push.lightstreamer.com/lightstreamer/create_session.txt?LS_protocol=TLCP-2.0.0";

    let response = client.post(request_url)
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let mut stream = response.bytes_stream();

        while let Some(item) = stream.next().await {
            match item {
                Ok(bytes) => {
                    let response_text = String::from_utf8(bytes.to_vec()).expect("Failed to convert bytes to string");
                    // Intenta extraer el ID de sesión de la primera respuesta
                    if let Some(start) = response_text.find("CONOK,") {
                        if let Some(end) = response_text.find(",50000,5000,*\r\n") {
                            let session_id = &response_text[start + 6..end];
                            println!("Session ID: {}", session_id);
                            // Guardar el session_id en la variable compartida
                            let mut session_id_lock = session_id_shared.lock().unwrap();
                            *session_id_lock = session_id.to_string();
                        }
                    }
                },
                Err(e) => println!("Error while receiving: {:?}", e),
            }
        }
    } else {
        println!("Response was not successful: {}", response.status());
    }

    Ok(())
}

fn main() {
    let rt = Runtime::new().unwrap();

    // Crear una variable compartida para almacenar el session_id
    let session_id_shared = Arc::new(Mutex::new(String::new()));
    let session_id_shared_clone = session_id_shared.clone();

    // Lanzar la función establecer conexión en un nuevo hilo de Tokio
    rt.spawn(async move {
        establish_persistent_http_connection(session_id_shared_clone).await.unwrap();
    });

    // Crear otro hilo para acceder al session_id
    thread::spawn(move || {
        loop {
            // Simular un delay para esperar a que el session_id esté listo
            thread::sleep(std::time::Duration::from_secs(5));
            let session_id = session_id_shared.lock().unwrap();
            if !session_id.is_empty() {
                println!("Accessed Session ID from another thread: {}", *session_id);
                break; // Salir del bucle si ya se obtuvo el session_id
            }
        }
    })
    .join()
    .unwrap();
}


/*
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use reqwest::Error;
use tokio::time::sleep;
use std::time::Duration;
use std::string::FromUtf8Error;
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Session {
    key: String,
    server: String,
}

async fn establish_connection(client: &Client, server: &str) -> Result<Session, Error> {

    println!("Establishing connection to server: {}", server);

    let response = client.post(format!("{}/lightstreamer/create_session.txt", server))
        .body("LS_adapter_set=DEMO&LS_cid=mgQkwtwdysogQz2BJ4Ji%20kOj2Bg")
        .send()
        .await?;

    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        match item {
            Ok(bytes) => {
                // Convierte cualquier error de FromUtf8Error a Box<dyn Error>
                let message = String::from_utf8(bytes.to_vec()).map_err(|e| Box::new(e) as Box<dyn std::error::Error>);
                println!("Received: {:?}", message);
            },
            Err(e) => {
                eprintln!("Error while reading from stream: {}", e);
                break;
            }
        }
    }

    let session = Session {
        key: "S2520d1412903a84dM42fT4356206".to_string(),
        server: server.to_string(),
    };

    Ok(session)
}

async fn keep_alive(session: Arc<Session>) {
    let client = Client::new();

    loop {
        let response = client
            .post(&format!("{}/lightstreamer/control.txt", session.server))
            .body("LS_op=probe")
            .send()
            .await;

        if let Err(error) = response {
            eprintln!("Error en keep_alive: {}", error);
        }

        sleep(Duration::from_secs(5)).await;
    }
}

async fn subscribe_to_channel(session: &Session, channel_id: &str) -> Result<(), Error> {
    let client = Client::new();

    let response = client
        .post(&format!("{}/lightstreamer/control.txt", session.server))
        .body(format!(
            "LS_op=subscribe&LS_session={}&LS_id={}&LS_mode=DISTINCT",
            session.key, channel_id
        ))
        .send()
        .await?;

    let messages = response.json::<Vec<serde_json::Value>>().await?;

    for message in messages {
        println!("Message: {:?}", message);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let session = establish_connection(&client, "http://push.lightstreamer.com").await?;
    let session_arc = Arc::new(session);

    let keep_alive_session = session_arc.clone();
    tokio::spawn(async move {
        keep_alive(keep_alive_session).await;
    });

    subscribe_to_channel(&session_arc, "chat_room").await?;

    Ok(())
}
*/