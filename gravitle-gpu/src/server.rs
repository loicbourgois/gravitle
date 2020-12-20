use crate::pid;
use crate::Configuration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tungstenite::accept;
#[derive(Clone, Serialize, Deserialize)]
enum ClientCommand {
    SetActivation(Activation),
}
#[derive(Clone, Serialize, Deserialize)]
struct Activation {
    pid: usize,
    value: f32,
}
pub fn handle_websocket(
    client_data_lock: std::sync::Arc<std::sync::RwLock<String>>,
    configuration: Configuration,
    activations_lock: std::sync::Arc<std::sync::RwLock<HashMap<pid, f32>>>,
) {
    let address = configuration.address;
    let port = configuration.port;
    let host = format!("{}:{}", address, port);
    let server = TcpListener::bind(host.to_owned()).unwrap();
    println!("server started");
    for stream in server.incoming() {
        println!("incoming");
        let client_data_lock_clone = Arc::clone(&client_data_lock);
        let activations_lock_clone = Arc::clone(&activations_lock);
        thread::spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            let message = websocket.read_message().unwrap();
            println!("message: {}", message);
            if message == tungstenite::Message::Text("server_to_client".to_string()) {
                loop {
                    {
                        let message_write = tungstenite::Message::Text(
                            client_data_lock_clone.read().unwrap().to_string(),
                        );
                        match websocket.write_message(message_write) {
                            Ok(_) => {
                                // Do nothing
                            }
                            Err(error) => {
                                println!("error writer socket: {}", error);
                                break;
                            }
                        }
                    }
                    thread::sleep(Duration::from_millis(10));
                }
            } else if message == tungstenite::Message::Text("latency_checker".to_string()) {
                loop {
                    match websocket.read_message() {
                        Ok(message) => {
                            if message == tungstenite::Message::Text("check".to_string()) {
                                websocket
                                    .write_message(tungstenite::Message::Text(
                                        "check_back".to_string(),
                                    ))
                                    .unwrap();
                            } else {
                                println!("message not handled: {}", message);
                            }
                        }
                        Err(error) => {
                            println!("error: {}", error);
                            break;
                        }
                    }
                }
            } else if message == tungstenite::Message::Text("writer".to_string()) {
                loop {
                    match websocket.read_message() {
                        Ok(message) => match serde_json::from_str(&message.to_string()) {
                            Ok(ClientCommand::SetActivation(client_command)) => {
                                activations_lock_clone
                                    .write()
                                    .unwrap()
                                    .insert(client_command.pid, client_command.value);
                            }
                            Err(e) => {
                                println!("{}", e);
                            }
                        },
                        Err(error) => {
                            println!("error: {}", error);
                            break;
                        }
                    }
                }
            } else {
                println!("starting message not handled: {}", message);
            }
        });
    }
}
